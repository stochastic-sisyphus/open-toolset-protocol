`git-status` is declared `phase: "any"`. Active phase is set to `surgery` before invocation.
Confirms the phase gate exception: a tool with `phase: "any"` MUST be permitted regardless of the current active phase.
Spec: `spec/03-adapter.md` §"Phase gating algorithm" (`if tool.phase != active_phase and tool.phase != "any"`), `spec/05-conformance.md` §L2.
