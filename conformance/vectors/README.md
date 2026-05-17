# Conformance Vector Format

This document is the normative format specification for OATP conformance vectors.

## Directory layout

```
conformance/vectors/
  L1/
    L1-NNN-descriptive-name/
      toolsets.json       # registry under test (entry point)
      verdict.json        # expected outcome + invocation sequence
      notes.md            # 2-4 lines: spec section + why this case matters
      [child.toolsets.json]  # additional files when $ref is exercised
  L2/
    L2-NNN-descriptive-name/
      toolsets.json
      verdict.json
      notes.md
```

## `verdict.json` schema

```json
{
  "level": "L1",
  "case_id": "L1-001-valid-minimal",
  "expected": {
    "outcome": "accept",
    "exit_code": 0,
    "reason": "schema_valid"
  },
  "sequence": []
}
```

### Fields

| Field | Type | Description |
|---|---|---|
| `level` | `"L1"` \| `"L2"` \| `"L3"` \| `"L4"` | Conformance level this vector targets |
| `case_id` | string | Matches the directory name exactly |
| `expected.outcome` | `"accept"` \| `"reject"` | Policy decision |
| `expected.exit_code` | `0` \| `2` \| `3` \| `4` | Adapter exit code |
| `expected.reason` | string | Machine-readable reason code (see table below) |
| `sequence` | array | Ordered events the adapter receives. Empty for L1 (schema-only). |

### Exit codes

| Code | Meaning |
|---|---|
| 0 | Accepted - command executed successfully |
| 2 | Policy rejection: denylist, default_deny, phase gate, precondition, forbidden args |
| 3 | Schema error: registry failed JSON Schema validation |
| 4 | Registry not found |

### Reason codes

| Reason | Level | Description |
|---|---|---|
| `schema_valid` | L1 | Registry passes schema validation |
| `schema_invalid` | L1 | Registry fails schema validation |
| `registry_cycle` | L1 | `$ref` resolution detected a cycle |
| `denylist_match` | L2 | Tool name matched `policies.deny` |
| `default_deny` | L2 | No allow match and `default_action` is `"deny"` |
| `forbidden_args_match` | L2 | Full argv list matched `policies.forbidden_args` |
| `phase_gate_violation` | L2 | Tool phase does not match active phase |
| `required_tool_skipped` | L2 | Phase transition blocked: required tool not yet invoked |
| `precondition_unsatisfied` | L2 | `requires_prior` not satisfied by current phase trace |

### `sequence` event types

For L2+ vectors the sequence is the ordered list of events the adapter processes.

```json
{ "phase_set": "reconnaissance" }
```
Sets active phase. A failing phase transition uses `"expect_reject": true`:
```json
{ "phase_set": "surgery", "expect_reject": true }
```

```json
{ "invoke": "ast-grep", "args": ["--pattern", "$X"] }
```
Tool invocation. A rejected invocation uses `"expect_reject": true`:
```json
{ "invoke": "grep", "args": ["foo"], "expect_reject": true }
```

The `expected.outcome` and `expected.exit_code` describe the **final event** in the sequence that triggers a verdict. For sequences where the rejection happens mid-sequence, the rejected event carries `"expect_reject": true`.

## Passing a vector

An implementation passes a vector if, for each event in `sequence` (processed in order):
- Events without `"expect_reject"` are accepted (exit 0 or execution proceeds).
- Events with `"expect_reject": true` are rejected with the `exit_code` and `reason` in `expected`.

For L1 vectors (`sequence: []`), the implementation passes if loading `toolsets.json` produces the `expected.outcome` and `exit_code`.
