# CLI specification

## Commands

### `forgegate init`

Creates `.forgegate/` in the current repository.

Expected output:

```text
Initialized ForgeGate project at /path/to/repo
```

### `forgegate agent new <name>`

Planned. Creates a new agent config, prompt, and policy.

### `forgegate run <task>`

Runs the default coding agent on a task.

MVP behavior:

- Load `.forgegate/config.yaml`.
- Load default agent.
- Load prompt and policy.
- Create run trace.
- Execute mocked model/tool loop in Phase 1.
- Execute real coding tools in Phase 2.
- Save trace.

### `forgegate inspect last`

Prints a readable summary of the latest trace.

### `forgegate eval <agent-id>`

Planned for Phase 3. Runs the agent's eval suite.

### `forgegate improve <agent-id> --auto`

Planned for Phase 5. Generates prompt/policy candidates, evaluates them, and promotes eligible improvements.

### `forgegate tui`

Planned for Phase 6. Launches the Rust TUI.
