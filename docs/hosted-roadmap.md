# Hosted/team roadmap

Hosted mode is not part of the MVP, but the architecture should not block it.

## Future paid/team features

1. Shared run history.
2. Team agent registry.
3. Organization policy controls.
4. Hosted trace retention.
5. Eval dashboards.
6. Approval workflows.
7. Cloud candidate review.
8. Cross-repo agent benchmarks.

## Local-first preservation

Hosted mode should be optional. Local ForgeGate must remain useful without an account.

## Future sync model

Potential sync unit:

```text
workspace
  agents
  versions
  traces
  eval_results
  candidates
  promotions
```

Local SQLite can act as the source of truth until explicit sync is configured.
