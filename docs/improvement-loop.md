# Improvement loop

ForgeGate improves coding agents through eval-gated prompt and policy changes.

## Loop

```text
Run agent
  ↓
Record trace
  ↓
Detect failure or weak performance
  ↓
Generate candidate prompt/policy change
  ↓
Run regression evals
  ↓
Compare against active version
  ↓
Promote only if gates pass
```

## Candidate types

| Type | Description |
|---|---|
| `prompt` | updates system/developer prompt |
| `policy` | updates tool/path/command/run policy |
| `prompt_and_policy` | updates both |

## Promotion gates

Candidate promotion requires:

1. no regression eval failures
2. minimum score improvement
3. no new policy violations
4. acceptable cost delta
5. acceptable latency delta

## Promotion record

A promotion should record:

- candidate ID
- base version
- promoted version
- eval result
- score delta
- cost delta
- latency delta
- changed files
- rationale
