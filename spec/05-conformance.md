# Conformance

This document defines how implementations claim OATP conformance and what each level requires.

## Conformance levels

OATP conformance is tiered. Each level is a strict superset of the previous.

### L1 — Schema Validation

The implementation can parse and validate a `toolsets.json` file against `schemas/toolsets.schema.json`.

Requirements:
- MUST accept all files that are valid per the JSON Schema
- MUST reject all files that are invalid per the JSON Schema
- MUST report validation errors with the path to the failing field
- MUST NOT modify valid registry files

Claim: `OATP-L1/0.x`

### L2 — Policy Enforcement

The implementation enforces toolset policies before executing any command.

Requirements:
- MUST satisfy all L1 requirements
- MUST deny commands matching `policies.banned_patterns`
- MUST deny commands not matching any `tools` entry when `default_action` is `"deny"`
- MUST allow commands not matching any `tools` entry when `default_action` is `"allow"`
- MUST enforce `args_pattern.allow` and `args_pattern.deny`
- MUST enforce `cwd_constraint`
- MUST enforce `path_denylist` and `path_allowlist`
- MUST return exit code 2 for all denied invocations
- MUST return exit code 3 for schema validation failures
- MUST return exit code 4 when no registry is found

Claim: `OATP-L2/0.x`

### L3 — Full Instrumentation

The implementation emits complete trace events for every invocation.

Requirements:
- MUST satisfy all L2 requirements
- MUST emit `tool.invoke` for every invocation received
- MUST emit `tool.allow` or `tool.deny` for every invocation (after validation)
- MUST emit `tool.exec.start` and `tool.exec.end` for every executed invocation
- MUST emit `toolset.schema_error` and `toolset.not_found` for registry errors
- MUST include all required fields for each event type (see `spec/04-instrumentation.md`)
- MUST write events to the configured `event_sink`
- MUST NOT suppress events when `trace_events` is `true`

Claim: `OATP-L3/0.x`

### L4 — Redaction and Approval

The implementation supports output redaction and approval workflows.

Requirements:
- MUST satisfy all L3 requirements
- MUST apply `redaction.patterns` to stdout and stderr before returning to caller
- MUST emit `tool.redact` events when redaction is applied
- MUST support `requires_approval: true` on tool entries
- MUST pause execution and emit `tool.approval_requested` for approval-gated tools
- MUST deny and exit 2 if approval is not granted (or times out)
- MUST NOT log or emit unredacted content matching redaction patterns

Claim: `OATP-L4/0.x`

## Conformance vectors

Conformance vectors are in `conformance/vectors/`. Each vector is a self-contained test case:

```
conformance/vectors/<level>-<name>/
  toolsets.json      # The registry under test
  cmd.txt            # The command being validated (one line: "cmd arg1 arg2")
  expected.json      # { "verdict": "allow"|"deny"|"error", "exit_code": N }
```

An implementation passes a vector if it produces the expected `verdict` and `exit_code` for the given `cmd.txt` when `toolsets.json` is the active registry.

## Claiming conformance

To claim OATP conformance:

1. Run all vectors for the target level from `conformance/vectors/`
2. All vectors MUST pass
3. Include the conformance claim string (`OATP-L2/0.x`) in your implementation's documentation and `--version` output

Conformance claims are self-reported. The OATP project does not operate a certification authority. Community members may challenge claims by adding failing vectors via RFC.

## Version compatibility

A conformance claim is valid for a specific spec version (`0.x`). When the spec version changes:

- **Patch** changes: existing conformance claims remain valid
- **Minor** changes: claims remain valid; new optional requirements may be added
- **Major** changes: all conformance claims must be re-validated against the new spec version
