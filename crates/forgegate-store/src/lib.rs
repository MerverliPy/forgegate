use anyhow::{Context, Result};
use forgegate_core::ProjectPaths;
use forgegate_trace::RunTrace;
use std::fs;
use std::path::PathBuf;

pub fn write_trace(paths: &ProjectPaths, trace: &RunTrace) -> Result<PathBuf> {
    fs::create_dir_all(&paths.traces_dir)?;
    let filename = format!("{}.json", trace.run_id);
    let path = paths.traces_dir.join(filename);
    let json = serde_json::to_string_pretty(trace)?;
    fs::write(&path, json)?;
    Ok(path)
}

pub fn read_trace(paths: &ProjectPaths, run_id: &str) -> Result<RunTrace> {
    let path = paths.traces_dir.join(format!("{run_id}.json"));
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("trace not found: {}", path.display()))?;
    serde_json::from_str(&contents).with_context(|| "failed to deserialize trace")
}

pub fn inspect_last_trace(paths: &ProjectPaths) -> Result<String> {
    let latest =
        latest_trace_path(paths)?.context("no traces found; run `forgegate run \"task\"` first")?;
    let contents = fs::read_to_string(&latest)?;
    let trace: RunTrace = serde_json::from_str(&contents)?;

    Ok(format!(
        "Trace: {}\nRun: {}\nAgent: {} v{}\nTask: {}\nStatus: {:?}\nScore: {:?}\nEvents: {}\nFile: {}",
        trace.trace_id,
        trace.run_id,
        trace.agent_id,
        trace.agent_version,
        trace.task,
        trace.status,
        trace.score,
        trace.events.len(),
        latest.display()
    ))
}

fn latest_trace_path(paths: &ProjectPaths) -> Result<Option<PathBuf>> {
    if !paths.traces_dir.exists() {
        return Ok(None);
    }

    let mut entries: Vec<_> = fs::read_dir(&paths.traces_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "json"))
        .collect();

    entries.sort_by_key(|entry| entry.metadata().and_then(|m| m.modified()).ok());

    Ok(entries.last().map(|entry| entry.path()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use forgegate_core::ProjectPaths;
    use forgegate_trace::{RunOutcome, RunStatus, RunTrace, ToolStatus, TraceEvent};

    #[test]
    fn write_and_read_trace_roundtrip() {
        let temp = std::env::temp_dir().join("forgegate_test_store");
        let _ = std::fs::remove_dir_all(&temp);
        let paths = ProjectPaths::new(temp.clone());

        let mut trace = RunTrace::new("test-agent", 3, "roundtrip task");
        trace.start();
        trace.add_event(TraceEvent::ModelCall {
            model: "mock/test".to_string(),
            input_tokens: 200,
            output_tokens: 100,
            latency_ms: 15,
        });
        trace.add_event(TraceEvent::ToolCall {
            tool: "upload_file".to_string(),
            args: serde_json::json!({"path": "src/lib.rs"}),
            status: ToolStatus::Success,
            latency_ms: 8,
            error: None,
        });
        trace.finish(
            RunStatus::Passed,
            RunOutcome {
                summary: "roundtrip ok".to_string(),
                failure_category: None,
            },
        );
        trace.score = Some(0.85);

        let written_path = write_trace(&paths, &trace).expect("write trace");
        assert!(written_path.exists());

        let read = read_trace(&paths, &trace.run_id).expect("read trace");
        assert_eq!(read.trace_id, trace.trace_id);
        assert_eq!(read.run_id, trace.run_id);
        assert_eq!(read.agent_id, "test-agent");
        assert_eq!(read.agent_version, 3);
        assert_eq!(read.task, "roundtrip task");
        assert_eq!(read.status, RunStatus::Passed);
        assert_eq!(read.score, Some(0.85));
        assert_eq!(read.events.len(), 2);
        assert_eq!(read.outcome.unwrap().summary, "roundtrip ok");

        let _ = std::fs::remove_dir_all(&temp);
    }
}
