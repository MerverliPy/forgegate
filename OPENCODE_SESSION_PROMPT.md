# OpenCode session prompt

You are working inside the ForgeGate repository.

ForgeGate is a local-first Rust CLI/TUI for coding agents. It runs coding agents, records structured traces, evaluates behavior, generates automatic prompt/policy improvements, and promotes improved agent versions only after eval gates pass.

## Critical constraints

- Build a CLI/TUI app, not a library-only project.
- Domain is coding agents.
- Use Rust for the CLI/TUI and core local product surface.
- Automatic improvement is limited to prompts and policies only.
- Do not implement automatic modification of ForgeGate's own source code.
- Local-first architecture is mandatory.
- Hosted/team mode must remain possible later, but do not implement cloud sync now.
- Do not introduce hidden network calls.
- Do not add a hosted dependency for the MVP.
- Do not remove traceability to simplify implementation.

## Immediate objective

Execute Phase 0, then Phase 1 from `IMPLEMENTATION_PHASES.md`.

## Phase 0 checklist

1. Run `cargo fmt`.
2. Run `cargo check --workspace`.
3. Run `cargo test --workspace`.
4. Run `cargo run -p forgegate-cli -- init`.
5. Run `cargo run -p forgegate-cli -- run "Fix the failing parser tests"`.
6. Run `cargo run -p forgegate-cli -- inspect last`.
7. Fix any scaffold issues before proceeding.

## Phase 1 implementation requirements

Implement a traceable local runner.

Required behavior:

- `forgegate init` creates `.forgegate/` with agents, prompts, policies, evals, traces, and config files.
- `forgegate run "task"` loads the default coding-agent config.
- The runner records a complete trace for each run.
- The trace includes:
  - trace ID
  - run ID
  - agent ID
  - agent version
  - task
  - timestamps
  - status
  - score placeholder
  - model-call events
  - tool-call events
  - policy-decision events
  - final outcome
- `forgegate inspect last` prints the latest trace in readable form.
- Implement clean module boundaries between CLI, core, trace, policy, tools, and store crates.

## Preferred implementation order

1. Make the existing scaffold compile.
2. Strengthen `forgegate-core` domain types.
3. Strengthen `forgegate-trace` event types.
4. Replace ad hoc trace writing with store abstractions.
5. Implement project path resolution.
6. Add config loading from `.forgegate/agents/coding-agent.yaml`.
7. Add policy loading from `.forgegate/policies/coding-agent.yaml`.
8. Add tests for trace serialization and project initialization.
9. Keep the model runner mocked until trace, policy, and tool contracts are stable.

## Definition of done for this session

- `cargo check --workspace` passes.
- `cargo test --workspace` passes.
- `forgegate init` works in an empty repo.
- `forgegate run "..."` creates a valid trace.
- `forgegate inspect last` prints the latest trace.
- At least one unit test exists for trace serialization.
- At least one unit test exists for policy path denial.
- The README remains accurate.

## Do not do yet

- Do not build the full TUI before the runner works.
- Do not implement hosted sync.
- Do not add a PWA.
- Do not add real model-provider calls before the trace and policy contract are stable.
- Do not make promotion modify source code.
