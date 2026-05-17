`cymbal` is `required: true` and `phase: "reconnaissance"`. The sequence runs `git-status` but never invokes `cymbal`, then attempts a phase transition to `surgery`.
Confirms the adapter scans the phase trace at transition time and blocks the move with `required_tool_skipped` when any required tool has not been called.
Spec: `spec/03-adapter.md` §"Required tool enforcement", §"Phase gating algorithm" (on phase_transition pseudocode).
