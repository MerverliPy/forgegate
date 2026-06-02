use anyhow::{Context, Result};
use forgegate_core::ProjectPaths;
use forgegate_trace::RunTrace;
use std::fs;
use std::path::PathBuf;

pub fn write_mock_trace(paths: &ProjectPaths, task: &str) -> Result<PathBuf> {
    fs::create_dir_all(&paths.traces_dir)?;
    let trace = RunTrace::mock(task);
    let filename = format!("{}.json", trace.run_id);
    let path = paths.traces_dir.join(filename);
    let json = serde_json::to_string_pretty(&trace)?;
    fs::write(&path, json)?;
    Ok(path)
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
