# Open Toolset Protocol (OATP)

Universal spec for AI agent tool discipline and instrumentation. Move from agentic vibes to engineering. Predictable physics for how agents handle tools. Kill the hallucination cascade before it starts.

**Status: v0 draft**

---

## Why

Every dev hand-rolls their agent loop differently. Grep plus hope. No shared contract. When something goes wrong — and it always goes wrong — there's no instrumentation to tell you what the agent actually called, what policy it violated, or which tool invocation triggered the cascade.

OATP fixes this by defining the contract between an agent and its tools. Declarative. Auditable. Vendor-neutral. Any agent that follows the protocol gets predictable, inspectable tool behavior without framework lock-in.

## What's in this repo

| Path | Contents |
|---|---|
| [`spec/`](spec/) | Normative specification (00-manifesto through 05-conformance) |
| [`schemas/`](schemas/) | JSON Schema 2020-12 for `toolsets.json` |
| [`examples/`](examples/) | Ready-to-use toolset registries for Claude Code, Codex, and minimal setups |
| [`adapter/`](adapter/) | Reference adapter stub (`oatp` binary) in Rust |
| [`conformance/`](conformance/) | Conformance level definitions and test vectors |
| [`RFC/`](RFC/) | RFC process for evolving the spec |

## How it works

1. **Define a toolset registry** (`toolsets.json`) — declare which tools an agent may use, under what constraints, and what to instrument.
2. **Run the adapter** (`oatp exec -- <cmd>`) — the adapter validates each call against the active toolset before execution, emits trace events, and rejects policy violations before they reach the shell.
3. **Get instrumented returns** — every tool invocation produces a structured trace event: what was called, what policy matched, what was redacted, how long it took, what it exited with.

The spec is the contract. The adapter is one reference implementation. Any runtime that implements the spec can claim OATP conformance.

## Quick start

```bash
# Install the reference adapter (once it ships)
# oatp exec -- rg "pattern" src/

# Validate a toolset registry
# oatp check toolsets.json

# See spec/00-manifesto.md for the full picture
```

See [`examples/minimal.toolsets.json`](examples/minimal.toolsets.json) to start with the smallest valid registry.

## Conformance levels

| Level | Requirement |
|---|---|
| L1 | Validates `toolsets.json` against the JSON Schema |
| L2 | Enforces policies pre-execution (allow/deny) |
| L3 | Emits full instrumentation trace events |
| L4 | Supports redaction and approval workflows |

Details in [`spec/05-conformance.md`](spec/05-conformance.md) and [`conformance/README.md`](conformance/README.md).

## Participating

OATP evolves through RFCs. See [`RFC/README.md`](RFC/README.md) for the process.

- File a bug or question: open an issue
- Propose a spec change: copy [`RFC/0000-template.md`](RFC/0000-template.md), open a PR
- Add conformance vectors: contribute to [`conformance/vectors/`](conformance/vectors/)

## License

Apache-2.0. See [`LICENSE`](LICENSE).
