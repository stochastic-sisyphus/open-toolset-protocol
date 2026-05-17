# Open Agent Toolset Protocol (OATP)

**Code is instrumented, not narrated.**

The Open Agent Toolset Protocol (OATP) is a universal specification for AI agent tool discipline, structural instrumentation, and deterministic execution. OATP codifies how agents interact with internal and CLI tools to prevent hallucination cascades and ensure high-fidelity state management. By enforcing a strict separation between reconnaissance, surgery, and instrumentation, OATP makes agentic errors visible and prevents the grep-thrashing loops common in unconstrained LLM execution.

**Status: v0 draft**

---

## Prior art

### Open protocols in the neighborhood

These are open, vendor-neutral specifications for adjacent concerns. OATP is designed to compose with them, not compete with them.

- [Model Context Protocol (MCP)](https://modelcontextprotocol.io) - standard for exposing tools/resources to LLMs via JSON-RPC (`tools/list`, `tools/call`)
- [Agent-to-Agent (A2A)](https://github.com/google/A2A) - inter-agent communication and task delegation protocol
- [OpenAPI](https://www.openapis.org/) - HTTP API description standard; tool schemas often reference OpenAPI definitions
- [AsyncAPI](https://www.asyncapi.com/) - event-driven API description; complements OpenAPI for async tool interfaces
- [OpenTelemetry](https://opentelemetry.io/) - observability standard; OATP trace events are designed to align with OTel semantics
- [Language Server Protocol (LSP)](https://microsoft.github.io/language-server-protocol/) - editor-to-language-server protocol; informs OATP's structural navigation tool category
- [Debug Adapter Protocol (DAP)](https://microsoft.github.io/debug-adapter-protocol/) - debugger integration protocol; companion to LSP

### Framework toolset implementations

These are framework-internal toolset abstractions. None is an open protocol - toolsets defined in one don't compose with toolsets in another. OATP adds an interop layer on top.

- [Pydantic AI](https://ai.pydantic.dev/tools/) - `FunctionToolset`, `ExternalToolset`, `CombinedToolset`
- [LangChain](https://python.langchain.com/docs/concepts/tools/) - `Tool` and `Toolkit`
- [OpenAI function calling](https://platform.openai.com/docs/guides/function-calling) - `tools` array with JSON Schema definitions
- [Anthropic tool use](https://docs.anthropic.com/en/docs/build-with-claude/tool-use) - `tool_use` content blocks
- Agent products (Cursor, Claude Code, Codex, Aider, OpenHands) - each ships proprietary internal tool registries

Every framework reinvents the wheel. **None of them is an open protocol.** Toolsets defined in one framework don't compose with toolsets defined in another. Discipline (which tool, when, with what verification) is re-encoded per agent loop, by hand.

OATP is the interop layer. It is **not a competing toolset framework**. It is the vendor-neutral specification that:

1. Defines a single declarative format (`toolsets.json`) for advertising tools, disciplines, phases, and policies
2. Provides a universal adapter (`oatp` binary) that enforces policy for any agent regardless of framework - OATP has one adapter, not one per framework
3. Lets existing framework toolsets be translated into OATP-conformant registries via one-shot registry translators, which emit `toolsets.json` and exit without intercepting runtime execution
4. Makes "this agent's tool discipline" a portable, inspectable artifact instead of an implementation secret

If you already use Pydantic AI, MCP, LangChain, or any other framework - OATP doesn't replace it. OATP adds an external contract on top.

## Rationale

Every team building AI agent workflows hand-rolls its own guardrails. Deny lists in bash. Regex on stdout. Wrappers that half-work. None of it composes. None of it audits cleanly. None of it transfers when you swap the agent runtime.

OATP fixes this by defining the contract between an agent and its tools. Declarative. Auditable. Vendor-neutral. Any agent that follows the protocol gets predictable, inspectable tool behavior without framework lock-in.

OATP can enforce **causal preconditions**: a tool MAY declare that it requires another tool (or category of tool) to have already run in the current phase. This makes "grep without first tracing the symbol" structurally impossible. The discipline isn't a guideline - it's a precondition check.

OATP toolset discovery is layered, deterministic, and composable - using a layered multi-source resolution chain that is a pattern common to modern toolchain managers. An adapter resolves the active registry from explicit env, project files, user config, system config, advertised well-known endpoint, and a built-in canonical fallback (in that order). Registries can compose via `$ref`, allowing teams to publish reusable toolset fragments without forking.

## Repository contents

| Path | Purpose |
|---|---|
| [`manifesto.md`](./manifesto.md) | The operational laws of tool discipline |
| [`schema.json`](./schema.json) | Formal JSON Schema for `toolsets.json` (symlink to `schemas/`) |
| [`toolsets.json`](./toolsets.json) | Starter configuration to copy and modify |
| [`spec/`](spec/) | Full normative specification (01-protocol through 06-discovery) |
| [`examples/`](examples/) | Reference toolset configurations |
| [`adapter/`](adapter/) | The sole reference adapter implementation (`oatp` binary, Rust) |
| [`tools/registry-translators/`](tools/registry-translators/) | One-shot translators that read framework-native toolsets and emit `toolsets.json` (not runtime adapters) |
| [`RFC/`](RFC/) | Active and accepted RFCs |

## Protocol overview

1. **Define a toolset registry** (`toolsets.json`) - declare which tools an agent may use, in which phase, with what instrumentation requirements and capability claims.
2. **Negotiate capabilities** - at session start, the adapter and agent exchange capability blocks. Mismatches are refused before execution begins.
3. **Run with phase gating** (`oatp exec -- <cmd>`) - the adapter validates each call against the active toolset and the current phase (reconnaissance, surgery, or instrumentation). Out-of-phase calls and unsatisfied preconditions are rejected before they reach the shell.
4. **Get instrumented returns** - surgery and instrumentation tools return structured state objects, not free text. The adapter rejects results that don't match the declared return schema.

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
| L1 | Tool discovery - validates and resolves the toolset registry | `OATP-L1/0.x` |
| L2 | Phase gating - enforces `phase` per tool, required-tool satisfaction, explicit allow/deny | `OATP-L2/0.x` |
| L3 | Capability negotiation - adapter/agent exchange and validate `capabilities` blocks | `OATP-L3/0.x` |
| L4 | State attestation - enforces structured `instrumented_return` and `requires_prior` contracts | `OATP-L4/0.x` |

Details in [`spec/05-conformance.md`](spec/05-conformance.md) and [`conformance/README.md`](conformance/README.md).

## Participating

OATP evolves through RFCs. See [`RFC/README.md`](RFC/README.md) for the process.

- File a bug or question: open an issue
- Propose a spec change: copy [`RFC/0000-template.md`](RFC/0000-template.md), open a PR
- Add conformance vectors: contribute to [`conformance/vectors/`](conformance/vectors/)

## License

Apache-2.0. See [`LICENSE`](LICENSE).
