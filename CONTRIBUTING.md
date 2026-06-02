# Contributing

ForgeGate is built phase-by-phase.

Before changing behavior, identify the phase and acceptance criteria in `IMPLEMENTATION_PHASES.md`.

## Local checks

```bash
cargo fmt
cargo check --workspace
cargo test --workspace
```

## Design rules

- Keep business logic out of the TUI.
- Trace every model call, tool call, policy decision, eval result, and promotion.
- Keep automatic improvement limited to prompt and policy artifacts.
- Preserve local-first operation.
