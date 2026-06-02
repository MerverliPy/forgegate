# P0 foundation issues

## [core] Create `.forgegate/` project structure

Acceptance criteria:

- `forgegate init` creates config, agents, prompts, policies, evals, and traces directories.
- Existing files are not overwritten unless a force flag exists.

## [core] Define agent config schema

Acceptance criteria:

- Agent config loads from YAML.
- Missing required fields produce actionable errors.
- Config includes agent ID, active version, model, prompt, policy, tools, and eval suite.

## [trace] Define run trace schema

Acceptance criteria:

- Trace captures run ID, trace ID, agent ID, version, task, status, events, score, and outcome.
- Trace serializes to valid JSON.
- Unit tests cover serialization.

## [policy] Define policy config schema

Acceptance criteria:

- Policy config supports shell, file edit, run limit, and promotion rules.
- Denied paths and denied commands are enforced.
- Unit tests cover path and command denials.

## [cli] Implement `forgegate run`

Acceptance criteria:

- Loads default agent.
- Creates a run trace.
- Emits model-call and tool-call events.
- Saves trace locally.

## [cli] Implement `forgegate inspect last`

Acceptance criteria:

- Finds latest trace.
- Prints readable summary.
- Fails cleanly if no traces exist.
