# ForgeGate

ForgeGate is a local-first Rust CLI/TUI for coding agents.

It runs coding agents, records structured traces, detects failures, converts failures into regression evals, generates prompt and policy improvement candidates, and promotes improved agent versions only after eval gates pass.

## Status

This repository is a starter scaffold for implementation. The first target is a traceable local runner; the full TUI and self-improvement loop follow after the trace, policy, and eval foundations are stable.

## Core thesis

Coding agents are easy to demo and hard to trust. ForgeGate treats agent behavior like software:

- versioned
- traceable
- testable
- measurable
- promotable

## Initial scope

ForgeGate starts with coding agents only.

Allowed automatic improvements:

- prompt changes
- policy changes
- context/prompt inclusion rules

Out of scope for automatic improvement:

- modifying ForgeGate's own source code
- modifying credentials or secrets
- changing model provider credentials
- destructive shell behavior
- hosted sync in the MVP

## Workspace layout

```text
forgegate/
  crates/
    forgegate-cli/          # CLI entrypoint, later launches TUI
    forgegate-core/         # agent/config/domain model
    forgegate-trace/        # trace schema and trace serialization
    forgegate-policy/       # command/path/tool policy evaluation
    forgegate-tools/        # coding tools: file/git/test/shell
    forgegate-store/        # local persistence; JSON first, SQLite next
    forgegate-eval/         # eval case format and scoring
    forgegate-improver/     # prompt/policy candidate generation
  docs/
  .forgegate.example/
  github-issues/
```

## First working commands

```bash
cargo run -p forgegate-cli -- init
cargo run -p forgegate-cli -- run "Fix the failing parser tests"
cargo run -p forgegate-cli -- inspect last
```

The scaffold implements a mock trace writer so OpenCode can begin by replacing mocked behavior with real policy, tool, eval, and agent-runtime implementation.

## Implementation phases

See [`IMPLEMENTATION_PHASES.md`](./IMPLEMENTATION_PHASES.md).

## OpenCode starting point

Use [`OPENCODE_SESSION_PROMPT.md`](./OPENCODE_SESSION_PROMPT.md) as the initial terminal-session prompt.
