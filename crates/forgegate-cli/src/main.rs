use anyhow::Result;
use clap::{Parser, Subcommand};
use forgegate_core::{init_project, ProjectPaths};
use forgegate_store::{inspect_last_trace, write_mock_trace};

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
            let trace_path = write_mock_trace(&paths, &task)?;
            println!("Recorded mock trace: {}", trace_path.display());
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
