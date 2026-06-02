# Evals

ForgeGate evals are regression checks for coding-agent behavior.

## Eval case goals

A good eval case is:

- repeatable
- isolated
- task-specific
- scored
- policy-aware
- version comparable

## Eval checks

Initial check types:

1. `command`
2. `git_diff`
3. `policy`

Future check types:

1. `file_contains`
2. `file_not_contains`
3. `snapshot_diff`
4. `llm_judge`
5. `test_coverage`

## Scoring rule

Eval scores should be weighted and inspectable. Avoid opaque pass/fail only.

Example:

```yaml
scoring:
  tests_passed: 0.50
  minimal_diff: 0.20
  policy_compliance: 0.20
  explanation_quality: 0.10
```
