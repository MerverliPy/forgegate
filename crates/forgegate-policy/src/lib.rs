use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyConfig {
    pub shell: ShellPolicy,
    pub file_edits: FileEditPolicy,
    pub run_limits: RunLimits,
    pub promotion: PromotionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShellPolicy {
    #[serde(default)]
    pub allowed_commands: Vec<String>,
    #[serde(default)]
    pub denied_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileEditPolicy {
    #[serde(default)]
    pub require_existing_file: bool,
    #[serde(default)]
    pub deny_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunLimits {
    #[serde(default)]
    pub max_tool_calls: u32,
    #[serde(default)]
    pub max_runtime_seconds: u64,
    #[serde(default)]
    pub max_cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromotionPolicy {
    #[serde(default)]
    pub min_pass_rate_delta: f64,
    #[serde(default)]
    pub max_cost_delta_ratio: f64,
    #[serde(default)]
    pub max_latency_delta_ratio: f64,
    #[serde(default)]
    pub require_no_new_policy_violations: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub reason: String,
}

impl PolicyDecision {
    pub fn allow(reason: impl Into<String>) -> Self {
        Self {
            allowed: true,
            reason: reason.into(),
        }
    }

    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            allowed: false,
            reason: reason.into(),
        }
    }
}

pub fn check_path(policy: &PolicyConfig, path: &Path) -> PolicyDecision {
    let path_text = path.to_string_lossy();
    for denied in &policy.file_edits.deny_paths {
        if path_text.contains(denied) {
            return PolicyDecision::deny(format!("path contains denied segment: {denied}"));
        }
    }
    PolicyDecision::allow("path allowed")
}

pub fn check_command(policy: &PolicyConfig, command: &str) -> PolicyDecision {
    for denied in &policy.shell.denied_commands {
        if command.contains(denied) {
            return PolicyDecision::deny(format!("command contains denied pattern: {denied}"));
        }
    }

    if !policy.shell.allowed_commands.is_empty()
        && !policy
            .shell
            .allowed_commands
            .iter()
            .any(|allowed| command.starts_with(allowed))
    {
        return PolicyDecision::deny("command is not in the allowlist");
    }

    PolicyDecision::allow("command allowed")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn denies_path_segments() {
        let policy = PolicyConfig {
            file_edits: FileEditPolicy {
                deny_paths: vec![".env".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        let decision = check_path(&policy, &PathBuf::from("project/.env"));
        assert!(!decision.allowed);
    }

    #[test]
    fn allows_safe_paths() {
        let policy = PolicyConfig {
            file_edits: FileEditPolicy {
                deny_paths: vec![".env".to_string(), ".git/".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(check_path(&policy, &PathBuf::from("src/main.rs")).allowed);
    }

    #[test]
    fn denies_commands_outside_allowlist() {
        let policy = PolicyConfig {
            shell: ShellPolicy {
                allowed_commands: vec!["npm test".to_string()],
                denied_commands: vec!["rm -rf".to_string()],
            },
            ..Default::default()
        };
        assert!(check_command(&policy, "npm test").allowed);
        assert!(!check_command(&policy, "cargo test").allowed);
        assert!(!check_command(&policy, "rm -rf /tmp/x").allowed);
    }

    #[test]
    fn denies_commands_matching_denylist() {
        let policy = PolicyConfig {
            shell: ShellPolicy {
                denied_commands: vec!["sudo".to_string(), "curl".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(!check_command(&policy, "sudo rm file").allowed);
        assert!(!check_command(&policy, "curl http://evil.com").allowed);
        assert!(check_command(&policy, "cargo test").allowed);
    }

    #[test]
    fn empty_allowlist_permits_all_non_denied() {
        let policy = PolicyConfig {
            shell: ShellPolicy {
                denied_commands: vec!["wget".to_string()],
                allowed_commands: vec![],
            },
            ..Default::default()
        };
        assert!(!check_command(&policy, "wget file").allowed);
        assert!(check_command(&policy, "cargo build").allowed);
    }

    #[test]
    fn deserializes_full_policy_yaml() {
        let yaml = r#"
shell:
  allowed_commands:
    - npm test
    - cargo test
  denied_commands:
    - rm -rf
    - sudo
file_edits:
  require_existing_file: true
  deny_paths:
    - .env
    - .git/
run_limits:
  max_tool_calls: 30
  max_runtime_seconds: 600
  max_cost_usd: 2.0
promotion:
  min_pass_rate_delta: 0.05
  max_cost_delta_ratio: 0.15
  max_latency_delta_ratio: 0.25
  require_no_new_policy_violations: true
"#;
        let config: PolicyConfig =
            serde_yaml::from_str(yaml).expect("should deserialize policy yaml");
        assert_eq!(config.shell.allowed_commands.len(), 2);
        assert_eq!(config.shell.denied_commands.len(), 2);
        assert!(config.file_edits.require_existing_file);
        assert_eq!(config.file_edits.deny_paths.len(), 2);
        assert_eq!(config.run_limits.max_tool_calls, 30);
        assert_eq!(config.promotion.min_pass_rate_delta, 0.05);
    }
}
