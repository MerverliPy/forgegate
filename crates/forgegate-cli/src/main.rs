use anyhow::Result;
use clap::{Parser, Subcommand};
use forgegate_core::{
    init_project, load_agent_config, load_policy_config, load_prompt, ProjectPaths,
};
use forgegate_policy::PolicyDecision;
use forgegate_store::{inspect_last_trace, write_trace};
use forgegate_trace::{RunOutcome, RunStatus, RunTrace, ToolStatus, TraceEvent};

#[derive(Debug, Parser)]
#[command(name = "forgegate")]
#[command(about = "Local-first CLI/TUI for traceable, self-improving coding agents")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize .forgegate/ in the current working directory.
    Init,

    /// Run the default coding agent against a task.
    Run {
        /// Task for the coding agent.
        task: String,
    },

    /// Inspect a recorded run trace.
    Inspect {
        /// Use `last` for the latest trace. Trace ID support is planned.
        target: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cwd = std::env::current_dir()?;
    let paths = ProjectPaths::new(cwd);

    match cli.command {
        Commands::Init => {
            init_project(&paths)?;
            println!("Initialized ForgeGate project at {}", paths.root.display());
        }
        Commands::Run { task } => {
            run_agent(&paths, &task)?;
        }
        Commands::Inspect { target } => {
            if target != "last" {
                anyhow::bail!("only `inspect last` is implemented in the scaffold");
            }
            let rendered = inspect_last_trace(&paths)?;
            println!("{rendered}");
        }
    }

    Ok(())
}

fn run_agent(paths: &ProjectPaths, task: &str) -> Result<()> {
    let agent_id = "coding-agent";
    let config = load_agent_config(paths, agent_id)?;
    let prompt = load_prompt(paths, &config.prompt)?;
    let policy = load_policy_config(paths, &config.policy)?;

    let mut trace = RunTrace::new(&config.id, config.active_version, task);
    trace.start();

    trace.add_event(TraceEvent::PolicyDecision {
        allowed: true,
        reason: "start-of-run policy check passed".to_string(),
    });

    let model = format!("{} ({})", config.model.provider, config.model.name);
    trace.add_event(TraceEvent::ModelCall {
        model: model.clone(),
        input_tokens: (prompt.len() as u32) + (task.len() as u32) + 200,
        output_tokens: 480,
        latency_ms: 30,
    });

    let tool_sequence: Vec<(&str, &str, &str)> = vec![
        ("list_files", ".", "list_files ."),
        ("read_file", "src/lib.rs", "read_file src/lib.rs"),
        ("git_diff", "--stat", "git diff --stat"),
    ];

    let mut tool_calls = 0u32;
    let mut blocked = false;
    let mut blocked_reason = String::new();

    for (tool_name, args_text, command) in &tool_sequence {
        if tool_calls >= policy.run_limits.max_tool_calls {
            blocked = true;
            blocked_reason = format!(
                "max tool calls ({}) exceeded",
                policy.run_limits.max_tool_calls
            );
            break;
        }

        let decision: PolicyDecision = match *tool_name {
            "read_file" | "list_files" | "write_file" => {
                let path_check =
                    forgegate_policy::check_path(&policy, &std::path::PathBuf::from(args_text));
                path_check
            }
            "git_diff" | "git_status" | "run_tests" | "shell_command" => {
                forgegate_policy::check_command(&policy, command)
            }
            _ => PolicyDecision::allow("allowed tool"),
        };

        trace.add_event(TraceEvent::PolicyDecision {
            allowed: decision.allowed,
            reason: decision.reason.clone(),
        });

        if !decision.allowed {
            blocked = true;
            blocked_reason = decision.reason;
            break;
        }

        trace.add_event(TraceEvent::ToolCall {
            tool: tool_name.to_string(),
            args: serde_json::json!({ "args": args_text }),
            status: ToolStatus::Success,
            latency_ms: 10,
            error: None,
        });

        tool_calls += 1;
    }

    let (final_status, final_outcome) = if blocked {
        (
            RunStatus::BlockedByPolicy,
            RunOutcome {
                summary: format!("run blocked by policy: {blocked_reason}"),
                failure_category: Some("policy_violation".to_string()),
            },
        )
    } else {
        (
            RunStatus::Passed,
            RunOutcome {
                summary: format!(
                    "mock run completed: {} tool calls, {} model calls",
                    tool_calls, 1
                ),
                failure_category: None,
            },
        )
    };

    trace.finish(final_status, final_outcome);
    trace.score = Some(0.75);

    let trace_path = write_trace(paths, &trace)?;
    println!("Run {} completed", trace.run_id);
    println!("Status: {:?}", trace.status);
    println!("Trace: {}", trace_path.display());

    Ok(())
}
