# OATP Manifesto: Predictable Physics for AI Agent Tools

## The problem

Today's AI agent workflows are built on a shared fantasy: that the agent will do roughly the right thing if the prompt is good enough.

In practice, agents thrash. They call `grep` when `sg` exists. They run `npm install -g` inside sandboxes that don't want it. They spawn shell commands with unescaped interpolations. They hallucinate tool flags, retry forever, and cascade failures silently. The loop ends when the context window runs out or the bill arrives.

The root cause is structural, not prompting. There is no shared contract between the agent and its tools. The agent doesn't know what it's allowed to call. The runtime doesn't enforce what it should reject. There's no instrumentation on what actually ran, so post-hoc debugging is archaeology.

Every team that runs AI agents in production hand-rolls their own guardrails. Deny lists in bash. Regex on stdout. Wrappers that half-work. None of it composes. None of it audits cleanly. None of it transfers when you swap the agent runtime.

## The thesis

**Predictable physics.** A tool invocation is a fact, not a hope. The agent declares intent. The adapter validates against policy. The adapter executes or rejects — before the shell sees it. The adapter returns an instrumented result. Every step is observable.

**Declarative tool contracts.** What tools an agent may use, under what conditions, with what constraints — declared in a machine-readable registry (`toolsets.json`), not sprinkled across prompts and bash wrappers.

**Mandatory instrumented returns.** Every tool invocation produces a structured trace event: what was called, what policy matched, what was redacted, how long it took, what it exited with. Instrumentation is not optional telemetry — it is the return value of the protocol.

**Kill the hallucination cascade.** When an agent calls a banned tool, the adapter rejects it at the gate — exit code 2, trace event emitted, stdout clean. The cascade never starts because the bad call never reaches the shell.

## Design principles

1. **Vendor-neutral.** The spec works with any agent runtime: Claude Code, Codex, Cursor, Aider, OpenHands, Cline, or one you wrote last week. The adapter is one reference implementation, not the only valid one.

2. **Declarative-first.** Policy is data, not code. A `toolsets.json` file fully describes what is allowed, what is banned, and what instrumentation to emit. No imperative logic required to read it.

3. **Surgical tool surface.** Default deny is the correct starting position. An agent that can call anything is an agent that will call everything. Explicit allowlists, not blocklists of edge cases.

4. **Fail-closed.** When the toolset registry is missing, malformed, or ambiguous, the adapter rejects the call. Uncertainty resolves to denial, not permission.

5. **Instrumentation as first-class output.** Trace events are not side effects. They are the protocol's answer to "what happened." Conformant implementations emit them fully and durably.

## Non-goals

- **Not a runtime.** OATP does not manage agent loops, model calls, or context windows. It governs the tool boundary only.
- **Not a model trainer.** Trace events are for operational observability, not training data pipelines. What you do with them is out of scope.
- **Not opinionated about agent architecture.** Single-agent, multi-agent, hierarchical, flat — OATP is silent on topology. It only cares about the tool invocation boundary.
- **Not a replacement for sandboxing.** OATP is a policy layer. It does not replace OS-level sandboxing (seccomp, namespaces, VM isolation). Use both.

## The shift

Old model: prompt the agent to behave, grep the output to see if it did, patch the prompt when it didn't.

OATP model: declare the contract, enforce it structurally, instrument every invocation, make failure impossible to hide.

From "trying to fix code" to "following a protocol that makes failure observable and policy violations impossible to complete."

That's the shift. That's OATP.
