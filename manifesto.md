# OATP Manifesto: Predictable Physics for AI Agent Tools

## The problem

Today's AI agent workflows are built on a shared fantasy: that the agent will do roughly the right thing if the prompt is good enough.

In practice, agents thrash. They grep when structural search tools exist. They install packages in sandboxes that don't want them. They spawn shell commands with unescaped interpolations. They hallucinate tool flags, retry forever, and cascade failures silently. The loop ends when the context window runs out or the bill arrives.

The root cause is structural, not prompting. There is no shared contract between the agent and its tools. The agent doesn't know what it's allowed to call. The runtime doesn't enforce what it should reject. There's no instrumentation on what actually ran, so post-hoc debugging is archaeology.

Every team running AI agents in production hand-rolls its own guardrails. Deny lists in bash. Regex on stdout. Wrappers that half-work. None of it composes. None of it audits cleanly. None of it transfers when you swap the agent runtime.

## The thesis

**Predictable physics.** A tool invocation is a fact, not a hope. The agent declares intent. The adapter validates against policy. The adapter executes or rejects — before the shell sees it. The adapter returns an instrumented result. Every step is observable.

**Declarative tool contracts.** What tools an agent may use, under what conditions, with what constraints — declared in a machine-readable registry (`toolsets.json`), not sprinkled across prompts and bash wrappers.

**Mandatory instrumented returns.** Every tool invocation produces a structured trace event: what was called, what policy matched, what was redacted, how long it took, what it exited with. Instrumentation is not optional telemetry — it is the return value of the protocol.

**Kill the hallucination cascade.** When an agent calls a banned tool, the adapter rejects it at the gate — exit code 2, trace event emitted, stdout clean. The cascade never starts because the bad call never reaches the shell.

## Design principles

1. **Vendor-neutral.** The spec works with any agent runtime: Claude Code, Codex, Cursor, Aider, OpenHands, Cline, or one you wrote last week. The adapter is one reference implementation, not the only valid one.

2. **Declarative-first.** Policy is data, not code. A `toolsets.json` file fully describes what is allowed, what is banned, and what instrumentation to emit. No imperative logic required to read it.

3. **Reconnaissance → Surgery → Instrumentation.** Tools belong to exactly one phase. Phase boundaries are enforced, not suggested. An agent in reconnaissance cannot call a surgery tool. An agent in surgery cannot skip instrumentation. The three-phase discipline is the structural core of OATP — it makes tool misuse mechanically impossible, not merely inadvisable.

4. **Fail-closed.** When the toolset registry is missing, malformed, or ambiguous, the adapter rejects the call. Uncertainty resolves to denial, not permission.

5. **Instrumentation as first-class output.** Trace events are not side effects. They are the protocol's answer to "what happened." Surgery and instrumentation tools return structured state objects — not free text. "I changed X" is not a return value. A verifiable state delta is.

## The three phases

### Reconnaissance
Read-only exploration. The agent observes the environment, queries structure, and builds a map. No mutations. No side effects. Tools in this phase MUST be `verification_mode: deterministic` — the same call returns the same observable state.

### Surgery
Targeted mutation. The agent applies changes to the narrowest possible scope. Tools in this phase MUST return a structured `instrumented_return` describing exactly what changed. Narrative output is rejected by the adapter.

### Instrumentation
Verification and state emission. The agent confirms the mutation produced the expected state and emits the structured delta. No further mutations. This phase closes the loop: the system now has verifiable evidence of what changed, not a narrative claim.

Phase transitions are explicit. The agent declares the transition intent; the adapter logs it. Skipping a phase or transitioning backward is a protocol violation.

## Non-goals

- **Not a runtime.** OATP does not manage agent loops, model calls, or context windows. It governs the tool boundary only.
- **Not a model trainer.** Trace events are for operational observability, not training data pipelines. What you do with them is out of scope.
- **Not opinionated about agent architecture.** Single-agent, multi-agent, hierarchical, flat — OATP is silent on topology. It only cares about the tool invocation boundary.
- **Not a replacement for sandboxing.** OATP is a policy layer. It does not replace OS-level sandboxing (seccomp, namespaces, VM isolation). Use both.

## The shift

Old model: prompt the agent to behave, grep the output to see if it did, patch the prompt when it didn't.

OATP model: declare the contract, enforce it structurally, instrument every invocation, make failure impossible to hide.

From "hoping the agent behaves" to "following a protocol that makes non-compliance mechanically impossible."

Code is instrumented, not narrated.
