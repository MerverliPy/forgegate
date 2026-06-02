# P2 improver and TUI issues

## [improver] Generate prompt candidate from failed trace

Acceptance criteria:

- Reads failed trace.
- Produces candidate hypothesis.
- Writes candidate prompt file without mutating active version.

## [improver] Generate policy candidate from failed trace

Acceptance criteria:

- Proposes stricter policy when failure indicates unsafe or incorrect tool use.
- Does not weaken existing safety constraints automatically.

## [promotion] Implement eval-gated promotion

Acceptance criteria:

- Candidate runs against regression suite.
- Promotion is blocked on regression failure, new policy violation, excessive cost, or excessive latency.

## [tui] Create run list screen

Acceptance criteria:

- Shows recent runs, status, score, and agent version.

## [tui] Create trace detail screen

Acceptance criteria:

- Shows ordered event timeline.
- Highlights failed or blocked events.

## [tui] Create candidate comparison screen

Acceptance criteria:

- Shows base version vs candidate.
- Shows eval deltas.
- Shows promotion eligibility.
