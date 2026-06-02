use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunTrace {
    pub trace_id: String,
    pub run_id: String,
    pub agent_id: String,
    pub agent_version: u32,
    pub task: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: RunStatus,
    pub score: Option<f64>,
    pub events: Vec<TraceEvent>,
    pub outcome: Option<RunOutcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Queued,
    Running,
    Passed,
    Failed,
    BlockedByPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TraceEvent {
    ModelCall {
        model: String,
        input_tokens: u32,
        output_tokens: u32,
        latency_ms: u64,
    },
    ToolCall {
        tool: String,
        args: serde_json::Value,
        status: ToolStatus,
        latency_ms: u64,
        error: Option<String>,
    },
    PolicyDecision {
        allowed: bool,
        reason: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolStatus {
    Success,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunOutcome {
    pub summary: String,
    pub failure_category: Option<String>,
}

impl RunTrace {
    pub fn new(agent_id: impl Into<String>, agent_version: u32, task: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            trace_id: format!("trace_{}", Uuid::new_v4().simple()),
            run_id: format!("run_{}", Uuid::new_v4().simple()),
            agent_id: agent_id.into(),
            agent_version,
            task: task.into(),
            started_at: now,
            finished_at: None,
            status: RunStatus::Queued,
            score: None,
            events: Vec::new(),
            outcome: None,
        }
    }

    pub fn start(&mut self) {
        self.started_at = Utc::now();
        self.status = RunStatus::Running;
    }

    pub fn add_event(&mut self, event: TraceEvent) {
        self.events.push(event);
    }

    pub fn finish(&mut self, status: RunStatus, outcome: RunOutcome) {
        self.finished_at = Some(Utc::now());
        self.status = status;
        self.outcome = Some(outcome);
    }

    pub fn mock(task: &str) -> Self {
        let now = Utc::now();
        Self {
            trace_id: format!("trace_{}", Uuid::new_v4().simple()),
            run_id: format!("run_{}", Uuid::new_v4().simple()),
            agent_id: "coding-agent".to_string(),
            agent_version: 1,
            task: task.to_string(),
            started_at: now,
            finished_at: Some(now),
            status: RunStatus::Passed,
            score: Some(0.50),
            events: vec![
                TraceEvent::PolicyDecision {
                    allowed: true,
                    reason: "mock run allowed by scaffold policy".to_string(),
                },
                TraceEvent::ModelCall {
                    model: "mock-model".to_string(),
                    input_tokens: 1200,
                    output_tokens: 400,
                    latency_ms: 25,
                },
                TraceEvent::ToolCall {
                    tool: "git_diff".to_string(),
                    args: serde_json::json!({}),
                    status: ToolStatus::Success,
                    latency_ms: 10,
                    error: None,
                },
            ],
            outcome: Some(RunOutcome {
                summary: "mock run completed; replace with real coding-agent loop in Phase 1"
                    .to_string(),
                failure_category: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_serializes_to_json() {
        let trace = RunTrace::mock("test task");
        let json = serde_json::to_string(&trace).expect("trace should serialize");
        assert!(json.contains("coding-agent"));
        assert!(json.contains("model_call"));
    }

    #[test]
    fn trace_lifecycle_records_correct_statuses() {
        let mut trace = RunTrace::new("coding-agent", 1, "test task");
        assert_eq!(trace.status, RunStatus::Queued);
        assert!(trace.events.is_empty());

        trace.start();
        assert_eq!(trace.status, RunStatus::Running);

        trace.add_event(TraceEvent::PolicyDecision {
            allowed: true,
            reason: "test".to_string(),
        });
        assert_eq!(trace.events.len(), 1);

        trace.finish(
            RunStatus::Passed,
            RunOutcome {
                summary: "done".to_string(),
                failure_category: None,
            },
        );
        assert_eq!(trace.status, RunStatus::Passed);
        assert!(trace.finished_at.is_some());
        assert!(trace.outcome.is_some());
    }

    #[test]
    fn trace_roundtrip_through_json() {
        let mut trace = RunTrace::new("test-agent", 2, "some task");
        trace.start();
        trace.add_event(TraceEvent::ModelCall {
            model: "mock/test".to_string(),
            input_tokens: 100,
            output_tokens: 50,
            latency_ms: 10,
        });
        trace.add_event(TraceEvent::ToolCall {
            tool: "list_files".to_string(),
            args: serde_json::json!({"path": "."}),
            status: ToolStatus::Success,
            latency_ms: 5,
            error: None,
        });
        trace.finish(
            RunStatus::Passed,
            RunOutcome {
                summary: "all good".to_string(),
                failure_category: None,
            },
        );

        let json = serde_json::to_string(&trace).expect("serialize");
        let deserialized: RunTrace = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(deserialized.trace_id, trace.trace_id);
        assert_eq!(deserialized.run_id, trace.run_id);
        assert_eq!(deserialized.agent_id, "test-agent");
        assert_eq!(deserialized.agent_version, 2);
        assert_eq!(deserialized.task, "some task");
        assert_eq!(deserialized.status, RunStatus::Passed);
        assert_eq!(deserialized.events.len(), 2);
        assert_eq!(deserialized.outcome.unwrap().summary, "all good");
    }
}
