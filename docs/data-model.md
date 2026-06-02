# Data model

## Entities

| Entity | Purpose |
|---|---|
| Workspace | local or future team boundary |
| Agent | named coding agent |
| AgentVersion | immutable prompt/policy snapshot |
| Run | execution of an agent version on a task |
| Trace | ordered event record for a run |
| EvalSuite | collection of eval cases |
| EvalCase | one repeatable coding task/check |
| EvalResult | score for agent version on eval case/suite |
| Candidate | proposed prompt/policy change |
| Promotion | activation of candidate as new version |

## Versioning rule

Runs must point to immutable agent versions, not mutable agent configs.

## Storage progression

Phase 1:

- JSON trace files

Phase 2-3:

- SQLite index
- JSON blobs for trace details

Later:

- normalized SQLite tables
- exportable sync bundles
