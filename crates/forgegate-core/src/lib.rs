use anyhow::Result;
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
}
