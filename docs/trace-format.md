# Trace format

Every run produces a structured trace.

## Required top-level fields

| Field | Description |
|---|---|
| `trace_id` | Unique trace identifier |
| `run_id` | Unique run identifier |
| `agent_id` | Agent that performed the task |
| `agent_version` | Immutable agent version used for the run |
| `task` | User task |
| `started_at` | UTC timestamp |
| `finished_at` | UTC timestamp or null |
| `status` | queued, running, passed, failed, blocked_by_policy |
| `score` | optional run score |
| `events` | ordered event list |
| `outcome` | final summary and failure metadata |

## Event types

### `model_call`

Fields:

- `model`
- `input_tokens`
- `output_tokens`
- `latency_ms`

### `tool_call`

Fields:

- `tool`
- `args`
- `status`
- `latency_ms`
- `error`

### `policy_decision`

Fields:

- `allowed`
- `reason`

## Compatibility target

The local trace schema should stay compatible with later export to OpenTelemetry-style spans, but the MVP should use simple JSON/SQLite storage.
