# ForgeGate implementation phases

## Phase 0 — Repository sanity pass

Goal: make the scaffold build, format, lint, and run the mock commands.

Tasks:

1. Confirm Rust toolchain is available.
2. Run `cargo fmt`.
3. Run `cargo check --workspace`.
4. Run `cargo test --workspace`.
5. Run `cargo run -p forgegate-cli -- init`.
6. Run `cargo run -p forgegate-cli -- run "Fix the failing parser tests"`.
7. Run `cargo run -p forgegate-cli -- inspect last`.

Acceptance criteria:

- Workspace builds.
- `.forgegate/` is created.
- A trace JSON file is written under `.forgegate/traces/`.
- `inspect last` prints the latest trace.

---

## Phase 1 — Traceable local runner

Goal: implement a local coding-agent runner with real trace events and no TUI dependency.

Build:

1. Agent config loader from `.forgegate/agents/*.yaml`.
2. Prompt loader from `.forgegate/prompts/*.md`.
3. Policy loader from `.forgegate/policies/*.yaml`.
4. Trace event writer with append-safe JSONL or JSON output.
5. Basic run lifecycle:
   - queued
   - running
   - passed
   - failed
   - blocked_by_policy
6. Mock model adapter interface.
7. Mock coding-agent loop that emits model-call and tool-call events.

Acceptance criteria:

- `forgegate run "task"` records a complete trace.
- Trace has agent ID, version, task, status, score, events, failure summary if failed.
- No model provider is required for this phase.

---

## Phase 2 — Coding tools and policy enforcement

Goal: add bounded, auditable coding tools.

Tools:

1. `read_file`
2. `write_file`
3. `list_files`
4. `git_status`
5. `git_diff`
6. `run_tests`
7. `shell_command`

Policy controls:

1. Allowed shell commands.
2. Denied shell commands.
3. Denied paths.
4. Require existing file for edits.
5. Max tool calls.
6. Max runtime seconds.
7. Max cost placeholder.

Acceptance criteria:

- Policy blocks denied commands and denied paths.
- Every tool call is traced.
- Failed tools record structured errors.
- `run_tests` captures exit code, stdout, stderr, duration.

---

## Phase 3 — Eval runner

Goal: turn coding tasks into repeatable regression checks.

Build:

1. Eval YAML parser.
2. Fixture/project copy mechanism.
3. Command checks.
4. Git diff checks.
5. Policy checks.
6. Weighted scoring.
7. `forgegate eval coding-agent` command.

Acceptance criteria:

- Eval cases run in isolated fixture directories.
- Pass/fail and score are recorded in the local store.
- Eval results can be compared across agent versions.

---

## Phase 4 — Prompt/policy versioning

Goal: treat agent behavior as versioned artifacts.

Build:

1. Agent version directories or versioned DB rows.
2. Prompt snapshots.
3. Policy snapshots.
4. Candidate records.
5. Diff summaries.
6. Promotion records.

Acceptance criteria:

- Every run points to an immutable agent version.
- Promotion creates a new active version.
- Old versions can still be evaluated.

---

## Phase 5 — Automatic improvement engine

Goal: generate and evaluate prompt/policy candidates from failed traces.

Build:

1. Failure summarizer.
2. Candidate generator interface.
3. Prompt candidate writer.
4. Policy candidate writer.
5. Candidate eval runner.
6. Promotion gate.
7. `forgegate improve coding-agent --auto` command.

Acceptance criteria:

- A failed trace can become a candidate change.
- Candidate changes are evaluated before promotion.
- Candidate promotion is blocked by regression, cost, latency, or policy violation thresholds.

---

## Phase 6 — Rust TUI

Goal: expose the existing runner, traces, evals, and candidates through a terminal UI.

Screens:

1. Agent list.
2. Run list.
3. Live run timeline.
4. Trace detail.
5. Eval result view.
6. Candidate comparison view.
7. Policy violation view.

Acceptance criteria:

- The TUI does not own business logic.
- CLI commands and TUI use the same core crates.
- User can inspect runs and candidates without reading raw JSON.

---

## Phase 7 — Hosted/team-ready boundary

Goal: stay local-first while avoiding architectural dead ends.

Build:

1. Local API boundary.
2. Exportable trace bundles.
3. Organization/team-ready IDs in the domain model.
4. Optional daemon process.
5. Sync-neutral storage interfaces.

Acceptance criteria:

- Hosted/team product can be added without rewriting core trace, eval, policy, or version models.
