# RFC 0004: Formal Verification Conformance (L5 and L6)

**Status**: Draft
**Created**: 2026-05-17
**Authors**: OATP contributors

---

## Summary

This RFC defines two optional conformance levels — L5 and L6 — layered on top of the existing L1–L4 levels. L5 requires an adapter to implement phase state machines as SCXML statecharts; L6 requires the adapter to provide machine-checked TLA+/PlusCal property proofs for the core safety and liveness properties of the OATP phase gating algorithm.

---

## Motivation

L1–L4 conformance (spec/05-conformance.md) covers runtime enforcement: discovery, phase gating, capability negotiation, and state attestation. But enforcement alone cannot guarantee protocol-level properties such as:

- **Safety**: a tool with `phase: surgery` is never executed while `active_phase` is `reconnaissance` — not just in the happy path, but under any interleaving of concurrent requests.
- **Liveness**: if a required tool has been invoked, a phase transition to the next phase always eventually succeeds.
- **Cycle absence**: the `$ref` resolution algorithm terminates for any finite, acyclic registry graph.

Formal verification addresses these at the design level rather than the test level. An L5 or L6 adapter provides mathematical evidence that the algorithm is correct.

This RFC does not require all adapters to pursue L5 or L6. These levels are opt-in. They exist to provide a vocabulary for research implementations, security-critical deployments, and protocol auditors.

---

## Detailed Design

### Conformance Levels (L5 and L6)

**L5 — SCXML Phase Machines**

An L5-conformant adapter provides an SCXML document (W3C SCXML, https://www.w3.org/TR/scxml/) that defines the phase state machine. The SCXML document MUST:

1. Model the three top-level states: `reconnaissance`, `surgery`, and `instrumentation`.
2. Define transitions between states that correspond to `oatp phase --set <phase>` invocations.
3. Guard each transition with a condition that the required-tool satisfaction constraint has been met for the exiting state.
4. Define a `denied` sink state reachable from any state on policy violation (exit 2).
5. Define a `schema_error` sink state reachable from `initial` on registry load failure (exit 3).
6. Be valid against the W3C SCXML schema.

The SCXML document MUST be stored at `formal/phase-machine.scxml` in the adapter repository.

The SCXML document MAY be executable — i.e., the adapter MAY use an SCXML runtime (Apache Commons SCXML, Qt SCXML, or equivalent) as the authoritative phase state machine. If the adapter does not execute SCXML at runtime, the document is informational but still normative for conformance testing purposes.

**L6 — TLA+/PlusCal Property Proofs**

An L6-conformant adapter provides a TLA+ (https://lamport.azurewebsites.net/tla/tla.html) specification and accompanying model-checked proofs for the following properties:

**Safety properties** (must hold in all reachable states):

- `PhaseGateSafety`: No tool with `tool.phase = P` is executed when `active_phase != P` and `tool.phase != "any"`.
- `RequiredToolSafety`: For any phase `P`, if phase exit to `P+1` is granted, then every tool with `required: true` and `phase = P` appears in the phase trace.
- `CycleFreedom`: The `$ref` resolution algorithm does not visit any registry node more than once in a single resolution chain.

**Liveness properties** (must hold for all fair executions):

- `PhaseProgressLiveness`: If all required tools for phase `P` have been invoked, a `phase --set <P+1>` request eventually succeeds (assuming no intervening policy violations).

The TLA+ specification MUST be stored at `formal/oatp.tla`. The PlusCal translation (if used) MUST be embedded in the same file. The TLC model-checker configuration MUST be stored at `formal/oatp.cfg`.

The model checker MUST verify all four properties above with no violated invariants.

### Canonical Properties (normative names)

Adapters claiming L5 or L6 MUST use these canonical names for properties and states. This allows protocol auditors to cross-reference claims across implementations.

**SCXML state names**

| State | Meaning |
|---|---|
| `recon` | `active_phase = reconnaissance` |
| `surgery` | `active_phase = surgery` |
| `instrumentation` | `active_phase = instrumentation` |
| `denied` | Policy or phase gate violation (terminal) |
| `schema_error` | Registry load or schema validation failure (terminal) |

**TLA+ variable names (RECOMMENDED)**

| Variable | Meaning |
|---|---|
| `active_phase` | Current active phase |
| `phase_trace` | Sequence of tool invocations in current phase |
| `registry` | Resolved flat tool manifest |
| `resolution_stack` | Stack of registry URIs being resolved (cycle detection) |

### Relation to L1–L4

L5 and L6 are additive. An adapter claiming L5 or L6 MUST also be L1–L4 conformant. The SCXML and TLA+ artifacts describe the same algorithm that the runtime adapter implements.

### Prior Art

- **W3C SCXML** (https://www.w3.org/TR/scxml/) — state chart XML; W3C Recommendation 2015. Production-grade runtimes exist for Java (Apache Commons SCXML), C++ (Qt SCXML), and JavaScript (xstate with SCXML import).
- **TLA+ reference** (https://lamport.azurewebsites.net/tla/tla.html) — Lamport's specification language. TLC (the TLA+ model checker) is open source and bundled with the TLA+ Toolbox. PlusCal (https://lamport.azurewebsites.net/pubs/pluscal.pdf) is a higher-level algorithm language that compiles to TLA+.
- **Aeneas** (https://github.com/AeneasVerifier/aeneas) — Rust-to-Lean4 extraction tool; relevant if a Rust adapter implementation seeks proof of implementation-level correctness beyond specification-level TLA+ proofs. Out of scope for L6, but noted for L7+ future work.
- **AWS TLA+ examples** (https://github.com/tlaplus/Examples) — reference library of TLA+ specifications for distributed protocols, useful for modeling the registry resolution cycle detection algorithm.

---

## Artifacts Required for Conformance Claim

An adapter claiming L5 or L6 MUST include in its repository:

| Level | Required Artifact | Path |
|---|---|---|
| L5 | SCXML phase machine | `formal/phase-machine.scxml` |
| L5 | Conformance declaration | `CONFORMANCE.md` (L5 claim) |
| L6 | TLA+ specification | `formal/oatp.tla` |
| L6 | TLC configuration | `formal/oatp.cfg` |
| L6 | TLC output log (CI-generated) | `formal/oatp.tlc.out` |
| L6 | Conformance declaration | `CONFORMANCE.md` (L6 claim) |

---

## Drawbacks

- L5/L6 add implementation burden for adapter authors who want to claim these levels. This is intentional — these levels exist for deployments where the burden is justified.
- SCXML execution at runtime adds a dependency not present in simpler implementations. The L5 spec explicitly allows non-executed SCXML (informational), which reduces this burden.
- TLC model checking time is bounded by the state space of the model. Authors MUST bound all unbounded sequences (phase trace, resolution stack) in the TLC configuration to ensure termination.

---

## Alternatives

- **Alloy** instead of TLA+: Alloy 6 supports temporal properties and is arguably more accessible. Rejected for now; TLA+ has broader adoption in distributed systems and is better documented for these property types.
- **Property-based testing** instead of formal proofs: `proptest` (Rust) or `hypothesis` (Python) can cover many invariants. Useful at L3–L4 but insufficient for safety proofs under all interleavings. Not a substitute for L6.

---

## Unresolved Questions

- Should `formal/oatp.tlc.out` be a committed artifact or a CI-generated artifact verified on each PR? Current proposal: CI-generated and committed, verified by the CI check diffing against a known-good baseline.
- Is there a formal relationship between the SCXML document (L5) and the TLA+ specification (L6)? For example, could the SCXML be mechanically extracted from the TLA+? This is an open research question; not required for conformance.
- Should L7 be reserved for implementation-level proofs via Aeneas/Lean4? Placeholder suggested, not yet specified.
