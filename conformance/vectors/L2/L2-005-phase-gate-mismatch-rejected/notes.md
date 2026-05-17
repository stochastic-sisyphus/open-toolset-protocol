`libcst` is declared `phase: "surgery"`. The active phase is `reconnaissance` (the session default). Policy would allow the call — phase gating fires afterward.
Confirms the phase gate check (step 6) is enforced even when policy passes, and that `surgery` tools cannot run during `reconnaissance`.
Spec: `spec/02-toolsets-schema.md` §"Policy evaluation" step 6, `spec/03-adapter.md` §"Phase gating algorithm".
