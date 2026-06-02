# OpenCode execution guide

## Start here

Paste the contents of `OPENCODE_SESSION_PROMPT.md` into your OpenCode terminal session.

## First commands

```bash
cargo fmt
cargo check --workspace
cargo test --workspace
cargo run -p forgegate-cli -- init
cargo run -p forgegate-cli -- run "Fix the failing parser tests"
cargo run -p forgegate-cli -- inspect last
```

## Expected result

The scaffold should create `.forgegate/` and write a mock trace.

## Then implement

Proceed through `IMPLEMENTATION_PHASES.md` in order.

Do not jump to TUI implementation until the runner, trace schema, policy engine, and eval contracts are stable.
