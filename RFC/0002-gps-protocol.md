# RFC 0002 - Sequenced Phase Sub-Steps

- **RFC number**: 0002
- **Start date**: 2026-05-17
- **Status**: Draft
- **Target spec version**: 0.1

## Summary

An optional capability (`sequenced_phase`) for toolsets to declare a canonical sub-sequence within the `reconnaissance` phase. When this capability is enabled, adapters enforce that reconnaissance tools are invoked in a declared order. This prevents a class of agent failures caused by skipping structural orientation before acting on incomplete context.

One concrete sub-sequence (locate - map - instrument) is used as the running example throughout this RFC, but the mechanism is general and not tied to any specific naming convention.

## Motivation

The OATP three-phase discipline (Reconnaissance - Surgery - Instrumentation) prevents agents from mutating code without observing state first. But reconnaissance itself has internal structure that is frequently violated:

- Agents build a structural picture of the codebase without first identifying the relevant entry points, producing hallucinated structural models
- Agents skip baseline state capture within reconnaissance, so when surgery produces changes there is no pre-surgery baseline to diff against

A sequenced sub-phase mechanism codifies a reconnaissance order that produces reliable context. One common sequence:

1. **Locate step**: identify the relevant entry points, files, and symbols (e.g. call graph lookup, symbol resolution, definition search)
2. **Map step**: build a structural model of the relevant scope (e.g. AST traversal, cross-reference expansion, dependency graph construction)
3. **Instrument step**: capture baseline state (e.g. current test results, current lint output, current file hashes) - this is the pre-surgery baseline, not post-surgery verification

Without enforced ordering, agents skip the first step and begin mapping from an arbitrary starting point, or skip baseline capture and have no ground truth for instrumentation phase verification.

This capability is opt-in - not all reconnaissance workflows are sequential. Free-form reconnaissance remains the default.

## Detailed design

### New capability

```json
"capabilities": {
  "sequenced_phase": true
}
```

When `sequenced_phase: true` is declared in the toolset capabilities block, the adapter MUST enforce the declared sub-step sequence within the `reconnaissance` phase.

### New field on reconnaissance tools

```json
{
  "name": "symbol-locate",
  "phase": "reconnaissance",
  "category": "navigation",
  "verification_mode": "deterministic",
  "phase_step": "locate"
}
```

Field: `phase_step` (string, optional). Value is a declared step name from the toolset's step sequence. The step names themselves are not fixed by this RFC - toolset authors declare the ordered sequence.

Tools without `phase_step` are ungated - they may be called at any point during reconnaissance regardless of sequenced-phase enforcement.

### Step sequence declaration

The toolset declares the ordered sequence of steps as an array in the capabilities block:

```json
"capabilities": {
  "sequenced_phase": true,
  "phase_step_sequence": ["locate", "map", "instrument"]
}
```

The adapter enforces that tools are invoked in this declared order. Step names are open strings - toolset authors choose names that fit their domain.

### Adapter enforcement (when `sequenced_phase: true`)

```
on tool_invoke(tool_name, args):
    tool = registry.resolve(tool_name)
    if sequenced_phase_enabled and tool.phase_step is not None:
        current_step = trace.current_phase_step(active_phase)
        allowed_steps = get_allowed_steps(current_step, step_sequence)
        if tool.phase_step not in allowed_steps:
            reject(exit=2, reason="phase_step_sequence_violation",
                   expected=allowed_steps, got=tool.phase_step)
    ...

fn get_allowed_steps(current, sequence):
    if current is None: return [sequence[0]]     # must start at first step
    idx = sequence.index(current)
    if idx + 1 < len(sequence):
        return [sequence[idx], sequence[idx + 1]]  # current or advance
    return [sequence[idx]]                          # at last step
```

Transitions are forward-only within the sequence (no going back). The sequence advances when the agent first invokes a tool at the next step.

Phase transition out of `reconnaissance` requires that if any sequenced tools are declared, at least one tool at each step has been invoked (unless the step has no declared tools, in which case it is auto-satisfied).

### Clarification: recon baseline step vs. top-level instrumentation phase

These are distinct:

- **Baseline step within reconnaissance**: captures the *pre-surgery baseline* - test results, lint output, file hashes, structural state - before any mutations. This is observational.
- **Top-level `instrumentation` phase** (post-surgery): verifies that mutations produced the expected state delta by comparing current state against the pre-surgery baseline. This is verification.

Any tool used for baseline capture MUST have `phase: "reconnaissance"`. It is not in the `instrumentation` phase. Implementations MUST NOT conflate the two.

### Example toolset fragment

```json
{
  "toolset_name": "sequenced-recon",
  "version": "0.1.0",
  "capabilities": {
    "sequenced_phase": true,
    "phase_step_sequence": ["locate", "map", "instrument"]
  },
  "phases": ["reconnaissance"],
  "tools": [
    {
      "name": "symbol-locate",
      "phase": "reconnaissance",
      "category": "navigation",
      "verification_mode": "deterministic",
      "phase_step": "locate",
      "required": true
    },
    {
      "name": "ast-map",
      "phase": "reconnaissance",
      "category": "syntax-match-rewrite",
      "verification_mode": "deterministic",
      "phase_step": "map",
      "required": true
    },
    {
      "name": "baseline-capture",
      "phase": "reconnaissance",
      "category": "verification",
      "verification_mode": "deterministic",
      "phase_step": "instrument",
      "required": true
    }
  ]
}
```

## Drawbacks

**Not all recon workflows are sequential.** Some agents legitimately interleave locate and map steps (e.g. iterative deepening search). This capability is opt-in to avoid blocking those workflows.

**Step granularity.** Three steps may be too coarse for complex agents. A `locate-deep` or `cross-reference-expand` step might be warranted. Because step names are open strings and the sequence is declared per-toolset, this is extensible without a spec change.

## Alternatives

**More granular sub-phases** (e.g. separate phases for locate, map, instrument). This would require changes to the core phase model and is more invasive. The `phase_step` field achieves the ordering constraint without restructuring the phase model.

**Free-form reconnaissance (current default).** Retained as the default when `sequenced_phase: false` or absent. Sequenced sub-steps are strictly additive.

**Open string vs. fixed enum for step names.** A fixed enum would reduce configuration errors but prevent domain-specific naming. Open strings with a declared sequence (this RFC's approach) allow flexibility while still enforcing order.

## Unresolved questions

1. Should the baseline-capture step overlap with the top-level `instrumentation` phase - specifically, should baseline-capture tools be callable from both? Current position: no, they are distinct roles with distinct timing. But the community may find this confusing.
2. Should sequence violation be a hard error (current proposal) or a warning with configurable severity? Hard error is more disciplined; warning allows gradual adoption.
3. Should `phase_step_sequence` be declared per-toolset (this RFC's approach) or per-tool as a partial order graph? A graph would be more expressive but significantly more complex.
4. How does the sequenced phase interact with `for_each` loops over symbol sets? The adapter should record a step as satisfied after the first invocation - not require all iterations to complete before advancing.
