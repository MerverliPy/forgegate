# Architecture

ForgeGate is a local-first product for coding-agent reliability.

## System boundary

```text
User terminal
  ↓
forgegate CLI/TUI
  ↓
core domain model
  ↓
agent runner
  ↓
policy engine + coding tools
  ↓
trace store + eval store + version store
```

## Principles

1. Local-first by default.
2. Every meaningful action is traceable.
3. CLI and TUI share core logic.
4. Prompt and policy are versioned artifacts.
5. Automatic promotion requires eval gates.
6. Hosted/team mode must be additive, not a rewrite.

## Crate responsibilities

| Crate | Responsibility |
|---|---|
| `forgegate-cli` | CLI entrypoint; later TUI launch command |
| `forgegate-core` | domain types, config loading, project paths |
| `forgegate-trace` | run trace schema, event schema, serialization |
| `forgegate-policy` | command/path/tool policy decisions |
| `forgegate-tools` | local coding tools |
| `forgegate-store` | local persistence; JSON first, SQLite later |
| `forgegate-eval` | eval cases, checks, scoring |
| `forgegate-improver` | prompt/policy candidates and promotion contracts |

## Hosted-ready notes

Use IDs now even before cloud exists:

- `workspace_id`
- `agent_id`
- `agent_version_id`
- `run_id`
- `trace_id`
- `candidate_id`
- `eval_suite_id`

The MVP can default `workspace_id` to `local`.
