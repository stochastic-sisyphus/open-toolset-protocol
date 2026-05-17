# RFC 0002 — GPS Protocol (Locate → Map → Instrument)

- **RFC number**: 0002
- **Start date**: 2026-05-17
- **Status**: Draft
- **Target spec version**: 0.1

## Summary

An optional capability (`gps_protocol`) for toolsets to declare a canonical sub-sequence within the `reconnaissance` phase: `locate → map → instrument`. When this capability is enabled, adapters enforce that reconnaissance tools are invoked in this order. This prevents a class of agent failures caused by skipping structural orientation before acting on incomplete context.

## Motivation

The OATP three-phase discipline (Reconnaissance → Surgery → Instrumentation) prevents agents from mutating code without observing state first. But reconnaissance itself has internal structure that is frequently violated:

- Agents `map` (build a structural picture of the codebase) without first `locate`-ing the relevant entry points, producing hallucinated structural models
- Agents skip `instrument` (capturing baseline state) within reconnaissance, so when surgery produces changes there is no pre-surgery baseline to diff against

The GPS sub-sequence codifies the reconnaissance order that produces reliable context:

1. **Locate**: identify the relevant entry points, files, and symbols (e.g. call graph lookup, symbol resolution, grep-for-definition)
2. **Map**: build a structural model of the relevant scope (e.g. AST traversal, cross-reference expansion, dependency graph construction)
3. **Instrument**: capture baseline state (e.g. current test results, current lint output, current file hashes) — this is the pre-surgery baseline, not post-surgery verification

Without GPS enforcement, agents skip locate and begin mapping from an arbitrary starting point, or skip the baseline capture and have no ground truth for instrumentation phase verification.

This capability is opt-in — not all reconnaissance workflows are sequential. Free-form reconnaissance remains the default.

## Detailed design

### New capability

```json
"capabilities": {
  "gps_protocol": true
}
```

When `gps_protocol: true` is declared in the toolset capabilities block, the adapter MUST enforce the GPS sequence within the `reconnaissance` phase.

### New field on reconnaissance tools

```json
{
  "name": "cymbal-locate",
  "phase": "reconnaissance",
  "category": "navigation",
  "verification_mode": "deterministic",
  "gps_step": "locate"
}
```

Field: `gps_step` (string, optional, enum: `"locate"`, `"map"`, `"instrument"`).

Tools without `gps_step` are ungated — they may be called at any point during reconnaissance regardless of GPS enforcement.

### Adapter enforcement (when `gps_protocol: true`)

```
on tool_invoke(tool_name, args):
    tool = registry.resolve(tool_name)
    if gps_enabled and tool.gps_step is not None:
        current_gps_step = trace.current_gps_step(active_phase)
        allowed_steps = get_allowed_gps_steps(current_gps_step)
        if tool.gps_step not in allowed_steps:
            reject(exit=2, reason="gps_sequence_violation",
                   expected=allowed_steps, got=tool.gps_step)
    ...

fn get_allowed_gps_steps(current):
    if current is None: return ["locate"]     # must start with locate
    if current == "locate": return ["locate", "map"]
    if current == "map": return ["map", "instrument"]
    if current == "instrument": return ["instrument"]
```

Transitions are forward-only within GPS (no going back from `map` to `locate`). The sequence advances when the agent first invokes a tool at the next step.

Phase transition out of `reconnaissance` requires that if any GPS tools are declared, at least one tool at each step has been invoked (unless the step has no declared tools, in which case it is auto-satisfied).

### Clarification: GPS `instrument` vs. top-level `instrumentation` phase

These are distinct:

- **GPS `instrument` step** (within reconnaissance): captures the *pre-surgery baseline* — test results, lint output, file hashes, structural state — before any mutations. This is observational.
- **Top-level `instrumentation` phase** (post-surgery): verifies that mutations produced the expected state delta by comparing current state against the pre-surgery baseline. This is verification.

GPS `instrument` tools MUST have `phase: "reconnaissance"`. They are not in the `instrumentation` phase. Implementations MUST NOT conflate the two.

### Example toolset fragment

```json
{
  "toolset_name": "gps-recon",
  "version": "0.1.0",
  "capabilities": { "gps_protocol": true },
  "phases": ["reconnaissance"],
  "tools": [
    {
      "name": "symbol-locate",
      "phase": "reconnaissance",
      "category": "navigation",
      "verification_mode": "deterministic",
      "gps_step": "locate",
      "required": true
    },
    {
      "name": "ast-map",
      "phase": "reconnaissance",
      "category": "syntax-match-rewrite",
      "verification_mode": "deterministic",
      "gps_step": "map",
      "required": true
    },
    {
      "name": "baseline-capture",
      "phase": "reconnaissance",
      "category": "verification",
      "verification_mode": "deterministic",
      "gps_step": "instrument",
      "required": true
    }
  ]
}
```

## Drawbacks

**Not all recon workflows are sequential.** Some agents legitimately interleave locate and map steps (e.g. iterative deepening search). This capability is opt-in to avoid blocking those workflows.

**Step granularity.** Three steps may be too coarse for complex agents. A `locate-deep` or `cross-reference-expand` step might be warranted. This RFC does not preclude extending the enum in a future RFC.

## Alternatives

**More granular sub-phases** (e.g. separate phases for locate, map, instrument). This would require changes to the core phase model and is more invasive. The `gps_step` field achieves the ordering constraint without restructuring the phase model.

**Free-form reconnaissance (current default).** Retained as the default when `gps_protocol: false` or absent. GPS is strictly additive.

**Naming the sequence differently.** "GPS" (Locate → Map → Instrument) is a memorable mnemonic. Alternative: "recon sequence", "pre-flight". GPS is preferred for its navigational connotations — you locate position, build a map, then take a reading.

## Unresolved questions

1. Should GPS `instrument` step overlap with the top-level `instrumentation` phase — specifically, should baseline-capture tools be callable from both? Current position: no, they are distinct roles with distinct timing. But the community may find this confusing.
2. Should GPS sequence violation be a hard error (current proposal) or a warning with configurable severity? Hard error is more disciplined; warning allows gradual adoption.
3. Should `gps_step` be extensible (open string) or fixed enum? Fixed enum (`locate`, `map`, `instrument`) is current proposal. Open string would allow `locate-deep`, `cross-reference`, etc.
4. How does GPS interact with `for_each` loops over symbol sets during map? The adapter should record the GPS step as satisfied after the first invocation — not require all iterations to complete before advancing.
