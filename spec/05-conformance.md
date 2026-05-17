# Conformance

This document defines how implementations claim OATP conformance and what each level requires.

## Conformance levels

OATP conformance is tiered around the four declared capabilities. Each level is a strict superset of the previous.

### L1 - Discovery

The implementation can parse, validate, and resolve the toolset registry, and can respond to tool discovery queries.

Requirements:
- MUST accept all `toolsets.json` files that are valid per `schemas/toolsets.schema.json`
- MUST reject all files that are invalid per the schema, with the path to the failing field
- MUST resolve nested toolsets depth-first into a flattened tool manifest
- MUST respond to discovery queries with the resolved manifest
- MUST support local discovery: `$OATP_TOOLSET` → `./toolsets.json` → `~/.config/oatp/toolsets.json`
- MUST support remote discovery: HTTP GET `https://<host>/.well-known/toolset.json`
- MAY cache discovery results for session lifetime

Claim: `OATP-L1/0.x`

### L2 - Phase Gating

The implementation enforces `phase_gate` per tool and tracks phase transitions.

Requirements:
- MUST satisfy all L1 requirements
- MUST track `active_phase`, defaulting to `reconnaissance` on session start
- MUST reject tool calls where `tool.phase_gate` does not match `active_phase` and is not `"any"`, with exit code 2 and reason `phase_gate_violation`
- MUST support explicit phase transition via `oatp phase --set <phase>`
- MUST log phase transitions as `phase.transition` trace events
- MUST enforce required-tool satisfaction at phase transition:
  - Scan trace log for all tools with `required: true` and `phase_gate` matching the exiting phase
  - If any required tool has not been invoked in the current phase, reject the transition with exit code 2 and reason `required_tool_skipped`
- MUST return exit code 2 for denied invocations
- MUST return exit code 3 for schema validation failures
- MUST return exit code 4 when no registry is found

Claim: `OATP-L2/0.x`

### L3 - Capability Negotiation

The implementation exchanges and validates capability blocks with the agent at session start.

Requirements:
- MUST satisfy all L2 requirements
- MUST emit a capability offer at session start, advertising which capabilities the adapter supports
- MUST accept a capability claim from the agent
- MUST refuse the session if the agent claims a capability that the toolset marks as required but the adapter does not support
- MUST silently disable features the toolset offers but the agent does not claim
- MUST log the negotiation result as a `session.capabilities` trace event
- MUST emit `tool.invoke`, `tool.allow`, `tool.deny` for every invocation
- MUST emit `tool.exec.start` and `tool.exec.end` for every executed invocation
- MUST emit `toolset.schema_error` and `toolset.not_found` for registry errors
- MUST include all required fields per `spec/04-instrumentation.md`
- MUST write events to the configured `event_sink`

Claim: `OATP-L3/0.x`

### L4 - State Attestation

The implementation enforces structured `instrumented_return` contracts. This is full OATP compliance.

Requirements:
- MUST satisfy all L3 requirements
- MUST enforce `instrumented_return.required: true` - reject tool results that do not satisfy `schema_ref`
- MUST emit `tool.attestation_failed` when a return value fails schema validation
- MUST apply `redaction.patterns` to stdout and stderr before returning to the caller
- MUST emit `tool.redact` events when redaction is applied
- MUST support `requires_approval: true` on tool entries: pause, emit `tool.approval_requested`, await signal
- MUST deny and exit 2 if approval is not granted or times out
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
3. Include the conformance claim string (e.g. `OATP-L2/0.x`) in your implementation's documentation and `--version` output

Conformance claims are self-reported. The OATP project does not operate a certification authority. Community members may challenge claims by adding failing vectors via RFC.

## Version compatibility

A conformance claim is valid for a specific spec version (`0.x`). When the spec version changes:

- **Patch** changes: existing conformance claims remain valid
- **Minor** changes: claims remain valid; new optional requirements may be added
- **Major** changes: all conformance claims must be re-validated against the new spec version
