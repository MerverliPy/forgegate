# Policies

Policies constrain coding-agent behavior.

## Policy categories

1. Shell command policy.
2. File edit policy.
3. Run limit policy.
4. Promotion policy.

## Automatic improvement bounds

The improver may update policies only within safe limits.

Allowed:

- add stricter shell deny patterns
- add safer path deny rules
- reduce max tool calls
- add retry requirements after test failures
- require more inspection before writing files

Blocked:

- weakening deny rules without explicit user approval
- increasing cost limits above configured maximum
- allowing destructive shell commands
- modifying credentials or secrets
- modifying ForgeGate source code as part of self-improvement

## Policy decision output

Every policy decision should produce:

```json
{
  "allowed": false,
  "reason": "command contains denied pattern: rm -rf"
}
```
