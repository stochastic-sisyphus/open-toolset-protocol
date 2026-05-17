# Conformance

This directory contains the conformance level definitions and test vectors for the Open Toolset Protocol.

## Levels

See `spec/05-conformance.md` for the normative definitions. Summary:

| Level | Claim string | What it tests |
|---|---|---|
| L1 | `OATP-L1/0.x` | Schema validation of `toolsets.json` |
| L2 | `OATP-L2/0.x` | Policy enforcement pre-execution |
| L3 | `OATP-L3/0.x` | Full trace event emission |
| L4 | `OATP-L4/0.x` | Redaction and approval workflows |

## Vector format

Each vector is a directory under `conformance/vectors/` named `<level>-<name>/`:

```
conformance/vectors/
  l1-valid-minimal/
    toolsets.json      # Registry under test
    cmd.txt            # Command to validate (one line: "cmd arg1 arg2 ...")
    expected.json      # Expected outcome
  l2-deny-banned-pattern/
    toolsets.json
    cmd.txt
    expected.json
```

### `expected.json` format

```json
{
  "verdict": "allow",
  "exit_code": 0
}
```

| Field | Values | Description |
|---|---|---|
| `verdict` | `"allow"`, `"deny"`, `"error"` | Expected policy decision |
| `exit_code` | integer | Expected adapter exit code |

`"error"` verdict covers schema errors (exit 3) and not-found (exit 4).

## Running vectors

An implementation passes a vector if it produces the expected `verdict` and `exit_code` for `cmd.txt` when `toolsets.json` is the active registry.

The test harness is implementation-defined. A conformant implementation MUST pass all vectors for its claimed level before advertising that conformance claim.

## Contributing vectors

To add a vector:

1. Create a directory: `conformance/vectors/<level>-<descriptive-name>/`
2. Add `toolsets.json`, `cmd.txt`, and `expected.json`
3. Open a PR — no RFC required for vectors that test existing behavior

Vectors that expose gaps in the spec (behavior not currently defined) require an RFC.
