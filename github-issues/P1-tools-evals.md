# P1 tools and eval issues

## [tools] Implement `read_file`

Acceptance criteria:

- Reads UTF-8 files.
- Applies path policy.
- Records structured error on failure.

## [tools] Implement `write_file`

Acceptance criteria:

- Applies path policy.
- Supports dry-run mode.
- Captures diff after write.

## [tools] Implement `git_diff`

Acceptance criteria:

- Captures current repo diff.
- Fails cleanly outside git repositories.

## [tools] Implement `run_tests`

Acceptance criteria:

- Runs allowed test command.
- Captures exit code, stdout, stderr, duration.
- Applies command policy.

## [eval] Define eval YAML schema

Acceptance criteria:

- Supports command, git_diff, and policy checks.
- Supports weighted scoring.

## [eval] Implement isolated fixture runner

Acceptance criteria:

- Copies fixture to temp working directory.
- Runs eval without mutating source fixture.
