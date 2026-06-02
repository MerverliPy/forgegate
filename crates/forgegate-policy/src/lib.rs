use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyConfig {
    pub denied_paths: Vec<String>,
    pub denied_commands: Vec<String>,
    pub allowed_commands: Vec<String>,
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
    for denied in &policy.denied_paths {
        if path_text.contains(denied) {
            return PolicyDecision::deny(format!("path contains denied segment: {denied}"));
        }
    }
    PolicyDecision::allow("path allowed")
}

pub fn check_command(policy: &PolicyConfig, command: &str) -> PolicyDecision {
    for denied in &policy.denied_commands {
        if command.contains(denied) {
            return PolicyDecision::deny(format!("command contains denied pattern: {denied}"));
        }
    }

    if !policy.allowed_commands.is_empty()
        && !policy
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
            denied_paths: vec![".env".to_string()],
            ..Default::default()
        };
        let decision = check_path(&policy, &PathBuf::from("project/.env"));
        assert!(!decision.allowed);
    }

    #[test]
    fn denies_commands_outside_allowlist() {
        let policy = PolicyConfig {
            allowed_commands: vec!["npm test".to_string()],
            denied_commands: vec!["rm -rf".to_string()],
            ..Default::default()
        };
        assert!(check_command(&policy, "npm test").allowed);
        assert!(!check_command(&policy, "cargo test").allowed);
        assert!(!check_command(&policy, "rm -rf /tmp/x").allowed);
    }
}
