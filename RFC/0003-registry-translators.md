# RFC 0003 - Registry Translator Pattern

- **RFC number**: 0003
- **Start date**: 2026-05-17
- **Status**: Draft
- **Target spec version**: 0.1

## Summary

Define a normative pattern for translating existing agent toolset frameworks (Pydantic AI, MCP, LangChain, OpenAI, Anthropic, and others) into OATP-conformant `toolsets.json` registries. This RFC specifies the minimum contract a registry translator must satisfy, the translation semantics for native tool definitions, and a roadmap for reference implementations.

## Motivation

Every major agent framework ships its own toolset abstraction:

- **Pydantic AI** - `FunctionToolset`, `ExternalToolset`, `CombinedToolset` ([docs](https://ai.pydantic.dev/tools/))
- **Model Context Protocol (MCP)** - `tools/list` / `tools/call` over JSON-RPC ([spec](https://modelcontextprotocol.io))
- **LangChain** - `Tool` and `Toolkit` ([docs](https://python.langchain.com/docs/concepts/tools/))
- **OpenAI** - function calling tool definitions ([docs](https://platform.openai.com/docs/guides/function-calling))
- **Anthropic** - `tool_use` content blocks ([docs](https://docs.anthropic.com/en/docs/build-with-claude/tool-use))

None of these is an open protocol. Toolsets defined in one framework don't compose with toolsets defined in another. When an agent system uses multiple frameworks - which is common in production - discipline and policy must be re-encoded per framework, by hand. There is no shared contract.

OATP fills this gap by providing a vendor-neutral interop layer. A **registry translator** reads a native framework toolset and emits a conformant `toolsets.json`. The framework's internal execution model is unchanged. Runtime enforcement is the sole responsibility of the universal `oatp` binary - not the translator.

The core argument: framework-native toolsets are diverse in shape, but OATP runtime enforcement must remain singular. Translators are the bridge from diverse to uniform. Introducing per-framework runtime enforcement layers would reconstitute exactly the fragmentation OATP exists to eliminate.

## Detailed design

### Translator contract

A conformant registry translator MUST:

1. Accept a native framework toolset as input
2. Produce a `toolsets.json` document that validates against `schema.json`
3. Preserve framework-specific tool semantics - no behavior changes to the source tools
4. Set `capabilities.discovery: true` in the generated registry
5. Emit and exit - no interception of tool-call hooks at runtime

A registry translator MUST NOT:
- Wrap tool-call execution or intercept framework-level tool hooks
- Claim `phase_gating: true` in the emitted registry (phase gating is enforced by `oatp`, not the translator)
- Position itself as an alternative to the universal `oatp` binary

A registry translator SHOULD:
- Assign accurate `phase` and `category` values based on the tool's actual role
- Document which capabilities are reflected beyond discovery
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

Fields that have no native equivalent and MUST be assigned by the translator author:
- `phase` - required; default to `"reconnaissance"` if unknown, document the assumption
- `category` - required; default to `"navigation"` if unknown, document the assumption
- `verification_mode` - required; default to `"heuristic"` for function tools, `"deterministic"` for pure query tools

### MCP-specific notes

MCP `tools/list` responses carry `inputSchema` per tool. If a tool also carries an output schema via `annotations.outputSchema`, map it to `instrumented_return.schema_ref`.

MCP server discovery composes with OATP's `$ref` mechanism: an MCP server's OATP registry (at `/.well-known/toolset.json`) can be referenced via `$ref: "https://<mcp-host>/.well-known/toolset.json"` in a parent registry. This allows multi-server MCP deployments to expose a unified OATP view.

### Pydantic AI-specific notes

Pydantic AI's `CombinedToolset` maps directly to OATP's nested `toolsets` array. Each constituent `FunctionToolset` or `ExternalToolset` becomes a sub-registry with its own `toolset_name`.

`ExternalToolset` tools - executed by an upstream service or frontend - map to `requires_approval: true` or to `phase: "surgery"` with explicit `instrumented_return` requirements, depending on their role.

### Registration

The OATP project will maintain a `TRANSLATORS.md` registry listing known community-maintained framework translators. Listing is non-normative and self-reported.

## Drawbacks

**Translation ambiguity.** `phase` and `category` have no native equivalent in most frameworks. Translator authors make judgment calls. Two translators for the same framework may assign different values, reducing interoperability at the discipline level. Mitigation: publish canonical translation guides per framework alongside the reference implementations.

**Spec without enforcement.** A translator that generates `toolsets.json` but is not paired with the `oatp` binary at runtime provides schema without enforcement. This is useful for discovery and auditing but provides no runtime discipline. Clearly distinguish spec-only deployments from `oatp`-enforced deployments in documentation.

## Alternatives

**OATP as a native framework feature.** Frameworks could natively support OATP fields (`phase`, `category`, `verification_mode`). This would eliminate the translator layer. However, it requires buy-in from each framework maintainer and couples them to OATP's versioning. The translator pattern avoids this dependency.

**Per-framework runtime adapters.** Each framework could ship its own OATP runtime enforcement layer. This re-fragments the enforcement surface and reconstitutes the problem OATP solves. The universal `oatp` binary must remain the single enforcement point.

**No translator layer.** Require users to write `toolsets.json` from scratch. This is the current state. It works for greenfield deployments but abandons the large installed base of framework-based toolsets.

## Unresolved questions

1. Should the spec define a canonical transformation format (e.g. a YAML mapping file) for configuring `phase` and `category` assignments per native tool, without writing a custom translator? This would let declarative configuration replace code for common cases.
2. Should there be an OATP translator SDK (Python, TypeScript, Rust) providing the translation scaffolding? Or is the pattern simple enough to implement from scratch each time?
3. Should translators be runnable as MCP tools themselves, allowing agents to trigger registry translation at session start?
