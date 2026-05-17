# RFC 0003 — Framework Adapter Pattern

- **RFC number**: 0003
- **Start date**: 2026-05-17
- **Status**: Draft
- **Target spec version**: 0.1

## Summary

Define a normative pattern for wrapping existing agent toolset frameworks (Pydantic AI, MCP, LangChain, OpenAI, Anthropic, and others) in OATP-conformant surfaces. This RFC specifies the minimum contract a framework adapter must satisfy, the translation semantics for native tool definitions, and a roadmap for reference implementations.

## Motivation

Every major agent framework ships its own toolset abstraction:

- **Pydantic AI** — `FunctionToolset`, `ExternalToolset`, `CombinedToolset` (see [docs](https://pydantic.dev/docs/ai/tools-toolsets/toolsets/))
- **Model Context Protocol (MCP)** — `tools/list` / `tools/call` over JSON-RPC (see [spec](https://modelcontextprotocol.io))
- **LangChain** — `Tool` and `Toolkit`
- **OpenAI** — function calling tool definitions
- **Anthropic** — `tool_use` content blocks

None of these is an open protocol. Toolsets defined in one framework don't compose with toolsets defined in another. When an agent system uses multiple frameworks — which is common in production — discipline and policy must be re-encoded per framework, by hand. There is no shared contract.

OATP fills this gap by providing a vendor-neutral interop layer. A framework adapter wraps a native toolset to expose an OATP-conformant surface. The framework's internal execution model is unchanged. The adapter adds a declarative contract that makes the toolset discoverable, composable, and policy-enforced across frameworks.

## Detailed design

### Adapter contract

A conformant framework adapter MUST:

1. Accept a native framework toolset as input
2. Produce a `toolsets.json` document that validates against `schema.json`
3. Preserve framework-specific tool semantics — no behavior changes to the wrapped tools
4. Set `capabilities.discovery: true` in the generated registry
5. Document which capabilities it enables beyond discovery

A framework adapter SHOULD:
- Assign accurate `phase` and `category` values based on the tool's actual role
- Enable `capabilities.phase_gating: true` if it intercepts the framework's tool-call hook
- Enable `capabilities.state_attestation: true` if it can validate structured outputs before returning to the agent
- Publish the generated registry at `/.well-known/toolset.json` for remote discovery

### Translation table

| Native concept | OATP field | Notes |
|---|---|---|
| Tool `name` | `tool.name` | Direct mapping |
| Tool `description` | `tool.description` | Direct mapping |
| Tool input schema | (not directly mapped) | MAY inform `requires_prior` or `operational_constraints` |
| Tool output schema | `instrumented_return.schema_ref` | If present in native definition |
| Toolset group | nested `toolsets` entry | Each group becomes a sub-registry |
| Framework-level `requires_approval` | `tool.requires_approval` | Implementation-defined trigger |

Fields that have no native equivalent and MUST be assigned by the adapter author:
- `phase` — required; default to `"reconnaissance"` if unknown, document the assumption
- `category` — required; default to `"navigation"` if unknown, document the assumption
- `verification_mode` — required; default to `"heuristic"` for function tools, `"deterministic"` for pure query tools

### MCP-specific notes

MCP `tools/list` responses carry `inputSchema` per tool. If a tool also carries an output schema via `annotations.outputSchema`, map it to `instrumented_return.schema_ref`.

MCP server discovery composes with OATP's `$ref` mechanism: an MCP server's OATP registry (at `/.well-known/toolset.json`) can be referenced via `$ref: "https://<mcp-host>/.well-known/toolset.json"` in a parent registry. This allows multi-server MCP deployments to expose a unified OATP view.

### Pydantic AI-specific notes

Pydantic AI's `CombinedToolset` maps directly to OATP's nested `toolsets` array. Each constituent `FunctionToolset` or `ExternalToolset` becomes a sub-registry with its own `toolset_name`.

`ExternalToolset` tools — executed by an upstream service or frontend — map to `requires_approval: true` or to `phase: "surgery"` with explicit `instrumented_return` requirements, depending on their role.

### Registration

The OATP project will maintain a `ADAPTERS.md` registry listing known community-maintained framework adapters. Listing is non-normative and self-reported.

## Drawbacks

**Translation ambiguity.** `phase` and `category` have no native equivalent in most frameworks. Adapter authors make judgment calls. This means two adapters for the same framework may assign different values, reducing interoperability at the discipline level. Mitigation: publish canonical translation guides per framework alongside the reference implementations.

**Enforcement gap.** An adapter that generates `toolsets.json` but does not intercept the framework's tool-call hook provides spec without enforcement. This is useful for discovery and auditing but offers no runtime discipline. Clearly distinguish spec-only adapters from enforcement adapters in documentation.

## Alternatives

**OATP as a native framework feature.** Frameworks could natively support OATP fields (`phase`, `category`, `verification_mode`). This would eliminate the adapter layer. However, it requires buy-in from each framework maintainer and couples them to OATP's versioning. The shim pattern avoids this dependency.

**No adapter layer.** Require users to write `toolsets.json` from scratch. This is the current state. It works for greenfield deployments but abandons the large installed base of framework-based toolsets.

## Unresolved questions

1. Should the spec define a canonical transformation format (e.g. a YAML mapping file) for configuring `phase` and `category` assignments per native tool, without writing a custom adapter? This would let declarative configuration replace code for common cases.
2. Should enforcement adapters intercept at the framework level (e.g. wrapping `agent.run()`) or at the OS level (the `oatp exec` binary)? Both are valid; guidance is needed.
3. Should there be an OATP adapter SDK (Python, TypeScript, Rust) providing the translation scaffolding? Or is the pattern simple enough to implement from scratch each time?
