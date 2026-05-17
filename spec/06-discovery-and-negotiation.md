# Discovery and Negotiation

This document specifies four protocol features that enable agents and adapters to agree on capabilities, enforce phase discipline, and produce verifiable state.

## 6.1 Tool discovery

Tool discovery is the process by which an agent learns what tools are available in the current session, in what phases they operate, and with what verification modes.

### Discovery sources

The adapter MUST resolve the active Toolset Registry at session start using the following precedence:

**Local discovery** (in order):

1. `$OATP_TOOLSET` environment variable - absolute path to `toolsets.json`
2. `./toolsets.json` - current working directory
3. `~/.config/oatp/toolsets.json` - user-global fallback

**Remote discovery**:

For remote or multi-agent scenarios, OATP standardizes a well-known path modeled on A2A agent card discovery:

```
GET https://<host>/.well-known/toolset.json
```

Response MUST:
- Return HTTP 200
- Set `Content-Type: application/json`
- Return a body that is a valid OATP registry per `schemas/toolsets.schema.json`

OATP discovery mirrors A2A's agent card discovery, applied to tool contracts rather than messaging. An A2A-compliant agent SHOULD advertise its OATP registry at `/.well-known/toolset.json` so peer agents can reason about which phases, categories, and verification modes the agent enforces internally before delegating work to it.

The adapter MAY cache the resolved registry for the session lifetime. The adapter MUST NOT cache across sessions without explicit configuration.

### Discovery response

After resolving the registry, the adapter:

1. Validates it against `schemas/toolsets.schema.json` (exit 3 on failure)
2. Resolves nested `toolsets` depth-first into a flattened tool manifest
3. Makes the manifest available for capability negotiation (§6.3) and phase gating (§6.2)

The discovery response emitted to the agent is the flattened manifest - a flat list of resolved tools with their `phase`, `category`, `verification_mode`, and `required` fields.

## 6.2 Phase gating

Phase gating enforces the three-phase discipline: **Reconnaissance → Surgery → Instrumentation**. Every tool declares a `phase`. The adapter tracks the current phase and rejects any tool call whose `phase` doesn't match.

### Phase state machine

```
session start
    |
    v
[reconnaissance] <-- default active phase
    |
    | oatp phase --set surgery
    v
[surgery]
    |
    | oatp phase --set instrumentation
    v
[instrumentation]
    |
    | (session end or explicit reset)
    v
[done]
```

Phase transitions are **explicit agent intents** - the agent calls `oatp phase --set <phase>`. The adapter logs every transition as a `phase.transition` trace event. Backward transitions (e.g. `instrumentation` → `surgery`) are permitted but MUST be logged as `phase.transition.backward` and MAY be rejected by policy.

### Phase gating algorithm

```
on session_start:
    active_phase = "reconnaissance"

on phase_transition(from, to):
    required_for_from = [t for t in registry.tools
                         if t.required == true and t.phase == from]
    satisfied = [t for t in required_for_from
                 if t.name in trace.invoked_in_phase(from)]
    missing = set(required_for_from) - set(satisfied)
    if missing:
        reject(exit=2, reason="required_tool_skipped",
               tools=[t.name for t in missing])
    active_phase = to
    emit("phase.transition", from=from, to=to)

on tool_invoke(tool_name, args):
    tool = registry.resolve(tool_name)
    if tool is None:
        reject(exit=2, reason="unknown_tool")
    if tool.phase != active_phase and tool.phase != "any":
        reject(exit=2, reason="phase_gate_violation",
               expected=active_phase, got=tool.phase)
    if tool.required:
        trace.record(active_phase, tool.name)
    validate_args(tool, args)
    return exec(tool, args)
```

`allowedCategories` is a derived view: the union of `tool.category` for all tools whose `tool.phase` matches `active_phase` or is `"any"`.

### Required tools

A tool with `required: true` MUST be invoked at least once in its declared phase before the agent may transition to the next phase. The adapter enforces this at **transition time, not invocation time** - it scans the phase trace log when `oatp phase --set` is called.

This mechanism prevents phase-skipping: an agent cannot proceed to surgery without completing mandatory reconnaissance, and cannot proceed to instrumentation without completing mandatory surgery steps.

## 6.3 Capability negotiation

At session start, the adapter and agent exchange `capabilities` blocks to agree on what the session will enforce.

### Negotiation protocol

```
1. Adapter emits capability offer:
   { "oatp_version": "0.1", "capabilities": { "discovery": true, "phase_gating": true, ... } }

2. Agent submits capability claim:
   { "capabilities": { "discovery": true, "phase_gating": true, ... } }

3. Adapter computes intersection:
   - For each capability in the toolset's capabilities block:
     - If toolset requires it (value: true) AND agent doesn't claim it:
       REFUSE session (exit 2, reason: capability_mismatch)
     - If toolset offers it AND agent doesn't claim it:
       Feature silently disabled for session

4. Adapter emits session.capabilities trace event with negotiated result
```

### Mismatch behavior

| Toolset claims | Agent claims | Result |
|---|---|---|
| `phase_gating: true` | `phase_gating: true` | Enabled |
| `phase_gating: true` | `phase_gating: false` | Session refused |
| `phase_gating: false` | `phase_gating: true` | Feature disabled (agent over-claiming; allowed) |
| `phase_gating: false` | `phase_gating: false` | Disabled |

## 6.4 State attestation

State attestation is the mechanism that turns "agent claims it changed X" into "agent presents verifiable state delta."

### The problem it solves

Surgery tools emit output. Without attestation, that output is free text - a narrative. The agent reports "I edited the file" and the system has no way to verify this without re-reading the file. When agents chain multiple mutations, the error accumulates silently.

State attestation solves this by requiring surgery and instrumentation tools to return a **structured state object** validated against a declared JSON Schema (`instrumented_return.schema_ref`). The adapter validates the return value before passing it to the agent. A result that doesn't match the schema is rejected - the tool call fails with exit code 1 and a `tool.attestation_failed` trace event.

### Enforcement

For any tool with `instrumented_return.required: true`:

1. Adapter executes the tool
2. Adapter parses stdout as JSON
3. Adapter validates parsed JSON against `schema_ref`
4. If validation fails: emit `tool.attestation_failed`, return exit code 1
5. If validation passes: emit `tool.exec.end` with attestation metadata, return to agent

The validated return value is the canonical output. The agent MUST base subsequent decisions on the attested state, not on narrative interpretation of raw stdout.

### Schema references

`schema_ref` is a URI. Implementations MUST support:
- `file://` - local schema file
- `https://` - remote schema (fetched at session start, cached)
- Fragment identifiers (`#/$defs/SurgeryResult`) within the toolset schema itself

Free-text tools (`verification_mode: "none"`) MAY omit `instrumented_return`. Deterministic tools (`verification_mode: "deterministic"`) SHOULD declare it. Heuristic tools (`verification_mode: "heuristic"`) MAY declare it with `required: false` to enable optional validation.
