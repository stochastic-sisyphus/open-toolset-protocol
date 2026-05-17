# Open Agent Toolset Protocol (OATP)

**Code is instrumented, not narrated.**

The Open Agent Toolset Protocol (OATP) is a universal specification for AI agent tool discipline, structural instrumentation, and deterministic execution. OATP codifies how agents interact with internal and CLI tools to prevent hallucination cascades and ensure high-fidelity state management. By enforcing a strict separation between reconnaissance, surgery, and instrumentation, OATP makes agentic errors visible and prevents the grep-thrashing loops common in unconstrained LLM execution.

**Status: v0 draft**

---

## Prior art and positioning

Every modern agent framework has a "toolset" concept:

- **Pydantic AI** — `FunctionToolset`, `ExternalToolset`, `CombinedToolset` ([docs](https://pydantic.dev/docs/ai/tools-toolsets/toolsets/))
- **Model Context Protocol (MCP)** — `tools/list` and `tools/call` ([spec](https://modelcontextprotocol.io))
- **LangChain** — `Tool` and `Toolkit` abstractions
- **OpenAI** — function calling and tool definitions
- **Anthropic** — `tool_use` content blocks
- **Cursor / Claude Code / Codex / Aider / OpenHands** — each ships proprietary tool registries

Every framework reinvents the wheel. **None of them is an open protocol.** Toolsets defined in one framework don't compose with toolsets defined in another. Discipline (which tool, when, with what verification) is re-encoded per agent loop, by hand.

OATP is the interop layer. It is **not a competing toolset framework**. It is the vendor-neutral specification that:

1. Defines a single declarative format (`toolsets.json`) for advertising tools, disciplines, phases, and policies
2. Lets existing framework toolsets be wrapped by a lightweight adapter so they expose an OATP-conformant surface
3. Makes "this agent's tool discipline" a portable, inspectable artifact instead of an implementation secret

If you already use Pydantic AI, MCP, LangChain, or any other framework — OATP doesn't replace it. OATP adds an external contract on top.

## Why

Every team building AI agent workflows hand-rolls its own guardrails. Deny lists in bash. Regex on stdout. Wrappers that half-work. None of it composes. None of it audits cleanly. None of it transfers when you swap the agent runtime.

OATP fixes this by defining the contract between an agent and its tools. Declarative. Auditable. Vendor-neutral. Any agent that follows the protocol gets predictable, inspectable tool behavior without framework lock-in.

OATP can enforce **causal preconditions**: a tool MAY declare that it requires another tool (or category of tool) to have already run in the current phase. This makes "grep without first tracing the symbol" structurally impossible. The discipline isn't a guideline — it's a precondition check.

OATP toolset discovery is layered, deterministic, and composable — using a layered multi-source resolution chain that is a pattern common to modern toolchain managers. An adapter resolves the active registry from explicit env, project files, user config, system config, advertised well-known endpoint, and a built-in canonical fallback (in that order). Registries can compose via `$ref`, allowing teams to publish reusable toolset fragments without forking.

## What's here

| Path | Purpose |
|---|---|
| [`manifesto.md`](./manifesto.md) | The operational laws of tool discipline |
| [`schema.json`](./schema.json) | Formal JSON Schema for `toolsets.json` (symlink to `schemas/`) |
| [`toolsets.json`](./toolsets.json) | Starter configuration to copy and modify |
| [`spec/`](spec/) | Full normative specification (01-protocol through 06-discovery) |
| [`examples/`](examples/) | Reference toolset configurations |
| [`adapter/`](adapter/) | Reference adapter implementation (Rust) |
| [`RFC/`](RFC/) | Active and accepted RFCs |

## How it works

1. **Define a toolset registry** (`toolsets.json`) — declare which tools an agent may use, in which phase, with what instrumentation requirements and capability claims.
2. **Negotiate capabilities** — at session start, the adapter and agent exchange capability blocks. Mismatches are refused before execution begins.
3. **Run with phase gating** (`oatp exec -- <cmd>`) — the adapter validates each call against the active toolset and the current phase (reconnaissance, surgery, or instrumentation). Out-of-phase calls and unsatisfied preconditions are rejected before they reach the shell.
4. **Get instrumented returns** — surgery and instrumentation tools return structured state objects, not free text. The adapter rejects results that don't match the declared return schema.

The spec is the contract. The adapter is one reference implementation. Any runtime that implements the spec can claim OATP conformance.

## Quick start

```bash
# Install the reference adapter (once it ships)
# oatp exec -- sg "pattern" src/

# List resolved tools from the active toolset
# oatp ls

# List all available registries in resolution order
# oatp registry ls

# Validate a toolset registry
# oatp check toolsets.json

# See manifesto.md for the full picture
```

Copy [`toolsets.json`](toolsets.json) at repo root to start with a working configuration. See [`examples/`](examples/) for more.

## Conformance levels

| Level | Capability | Claim |
|---|---|---|
| L1 | Tool discovery — validates and resolves the toolset registry | `OATP-L1/0.x` |
| L2 | Phase gating — enforces `phase` per tool, required-tool satisfaction, explicit allow/deny | `OATP-L2/0.x` |
| L3 | Capability negotiation — adapter/agent exchange and validate `capabilities` blocks | `OATP-L3/0.x` |
| L4 | State attestation — enforces structured `instrumented_return` and `requires_prior` contracts | `OATP-L4/0.x` |

Details in [`spec/05-conformance.md`](spec/05-conformance.md) and [`conformance/README.md`](conformance/README.md).

## Participating

OATP evolves through RFCs. See [`RFC/README.md`](RFC/README.md) for the process.

- File a bug or question: open an issue
- Propose a spec change: copy [`RFC/0000-template.md`](RFC/0000-template.md), open a PR
- Add conformance vectors: contribute to [`conformance/vectors/`](conformance/vectors/)

## License

Apache-2.0. See [`LICENSE`](LICENSE).
