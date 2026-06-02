use anyhow::{Context, Result};
use forgegate_policy::PolicyConfig;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ProjectPaths {
    pub root: PathBuf,
    pub forgegate_dir: PathBuf,
    pub agents_dir: PathBuf,
    pub prompts_dir: PathBuf,
    pub policies_dir: PathBuf,
    pub evals_dir: PathBuf,
    pub traces_dir: PathBuf,
}

impl ProjectPaths {
    pub fn new(root: PathBuf) -> Self {
        let forgegate_dir = root.join(".forgegate");
        Self {
            root,
            agents_dir: forgegate_dir.join("agents"),
            prompts_dir: forgegate_dir.join("prompts"),
            policies_dir: forgegate_dir.join("policies"),
            evals_dir: forgegate_dir.join("evals"),
            traces_dir: forgegate_dir.join("traces"),
            forgegate_dir,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: String,
    pub name: String,
    pub active_version: u32,
    pub model: ModelConfig,
    pub prompt: FileRef,
    pub policy: FileRef,
    pub tools: Vec<String>,
    pub eval_suite: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRef {
    pub file: String,
}

pub fn init_project(paths: &ProjectPaths) -> Result<()> {
    fs::create_dir_all(&paths.agents_dir)?;
    fs::create_dir_all(&paths.prompts_dir)?;
    fs::create_dir_all(&paths.policies_dir)?;
    fs::create_dir_all(&paths.evals_dir.join("coding-agent-core"))?;
    fs::create_dir_all(&paths.traces_dir)?;

    write_if_missing(
        &paths.forgegate_dir.join("config.yaml"),
        include_str!("../../../.forgegate.example/config.yaml"),
    )?;
    write_if_missing(
        &paths.agents_dir.join("coding-agent.yaml"),
        include_str!("../../../.forgegate.example/agents/coding-agent.yaml"),
    )?;
    write_if_missing(
        &paths.prompts_dir.join("coding-agent.md"),
        include_str!("../../../.forgegate.example/prompts/coding-agent.md"),
    )?;
    write_if_missing(
        &paths.policies_dir.join("coding-agent.yaml"),
        include_str!("../../../.forgegate.example/policies/coding-agent.yaml"),
    )?;
    write_if_missing(
        &paths
            .evals_dir
            .join("coding-agent-core")
            .join("fix-parser-tests.yaml"),
        include_str!("../../../.forgegate.example/evals/coding-agent-core/fix-parser-tests.yaml"),
    )?;

    Ok(())
}

pub fn load_agent_config(paths: &ProjectPaths, agent_id: &str) -> Result<AgentConfig> {
    let config_path = paths.agents_dir.join(format!("{agent_id}.yaml"));
    let contents = fs::read_to_string(&config_path)
        .with_context(|| format!("agent config not found: {}", config_path.display()))?;
    let config: AgentConfig =
        serde_yaml::from_str(&contents).with_context(|| "failed to parse agent config")?;
    Ok(config)
}

pub fn load_prompt(paths: &ProjectPaths, file_ref: &FileRef) -> Result<String> {
    let prompt_path = paths.forgegate_dir.join(&file_ref.file);
    fs::read_to_string(&prompt_path)
        .with_context(|| format!("prompt file not found: {}", prompt_path.display()))
}

pub fn load_policy_config(paths: &ProjectPaths, file_ref: &FileRef) -> Result<PolicyConfig> {
    let policy_path = paths.forgegate_dir.join(&file_ref.file);
    let contents = fs::read_to_string(&policy_path)
        .with_context(|| format!("policy file not found: {}", policy_path.display()))?;
    let config: PolicyConfig =
        serde_yaml::from_str(&contents).with_context(|| "failed to parse policy config")?;
    Ok(config)
}

fn write_if_missing(path: &Path, contents: &str) -> Result<()> {
    if !path.exists() {
        fs::write(path, contents)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_paths_resolve_expected_dirs() {
        let paths = ProjectPaths::new(PathBuf::from("/tmp/example"));
        assert_eq!(
            paths.forgegate_dir,
            PathBuf::from("/tmp/example/.forgegate")
        );
        assert_eq!(
            paths.traces_dir,
            PathBuf::from("/tmp/example/.forgegate/traces")
        );
    }

    #[test]
    fn load_agent_config_from_initialized_project() {
        let temp = std::env::temp_dir().join("forgegate_test_init");
        let _ = std::fs::remove_dir_all(&temp);
        let paths = ProjectPaths::new(temp.clone());
        init_project(&paths).expect("init");

        let config = load_agent_config(&paths, "coding-agent").expect("load config");
        assert_eq!(config.id, "coding-agent");
        assert_eq!(config.active_version, 1);
        assert_eq!(config.model.provider, "mock");
        assert!(config.tools.contains(&"read_file".to_string()));

        let _ = std::fs::remove_dir_all(&temp);
    }

    #[test]
    fn load_prompt_from_initialized_project() {
        let temp = std::env::temp_dir().join("forgegate_test_prompt");
        let _ = std::fs::remove_dir_all(&temp);
        let paths = ProjectPaths::new(temp.clone());
        init_project(&paths).expect("init");

        let config = load_agent_config(&paths, "coding-agent").expect("load config");
        let prompt = load_prompt(&paths, &config.prompt).expect("load prompt");
        assert!(prompt.contains("Coding Agent Prompt"));
        assert!(prompt.contains("Operating rules"));

        let _ = std::fs::remove_dir_all(&temp);
    }

    #[test]
    fn load_policy_config_from_initialized_project() {
        let temp = std::env::temp_dir().join("forgegate_test_policy");
        let _ = std::fs::remove_dir_all(&temp);
        let paths = ProjectPaths::new(temp.clone());
        init_project(&paths).expect("init");

        let config = load_agent_config(&paths, "coding-agent").expect("load config");
        let policy = load_policy_config(&paths, &config.policy).expect("load policy");
        assert_eq!(policy.shell.allowed_commands.len(), 9);
        assert!(policy.shell.denied_commands.contains(&"sudo".to_string()));
        assert_eq!(policy.run_limits.max_tool_calls, 30);

        let _ = std::fs::remove_dir_all(&temp);
    }
}
