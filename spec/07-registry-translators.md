# Framework Adapters

OATP is not a replacement for existing agent toolset frameworks. It is the interop layer that makes those frameworks' toolsets composable, inspectable, and policy-enforced across implementations.

This document describes the **shim pattern** — how to wrap an existing framework's native toolset in an OATP-conformant surface without changing the framework's internals.

## 7.1 The shim pattern

A framework adapter is a thin wrapper that:

1. **Reads** the framework's native toolset (Pydantic AI `FunctionToolset`, MCP `tools/list` response, LangChain `Toolkit`, OpenAI tool definitions, etc.)
2. **Translates** each native tool into an OATP `tool` entry, assigning `phase`, `category`, `verification_mode`, and any `requires_prior` constraints appropriate for the tool's role
3. **Generates** a `toolsets.json` document conforming to `schemas/toolsets.schema.json`
4. **Optionally enforces** OATP policies at invocation time by intercepting the framework's tool-call hook

The framework keeps its execution model. OATP adds a declarative contract layer. A framework adapter MUST:
- Produce a `toolsets.json` that validates against `schema.json`
- Preserve framework-specific tool semantics — no behavior changes
- Enforce OATP policies if it intercepts execution; if it does not intercept execution, the generated `toolsets.json` is spec-only (useful for discovery and auditing)

## 7.2 Pydantic AI adapter sketch

Pydantic AI ([docs](https://pydantic.dev/docs/ai/tools-toolsets/toolsets/)) provides `FunctionToolset`, `ExternalToolset`, and `CombinedToolset`. A `FunctionToolset` groups locally defined Python functions as agent tools. An `ExternalToolset` allows agents to call tools executed by an upstream service. `CombinedToolset` merges multiple toolsets.

Translation sketch (conceptual — not normative):

```python
# Conceptual — not normative
from pydantic_ai.tools import FunctionToolset

def to_oatp(toolset: FunctionToolset, *, phase: str, category: str) -> dict:
    """Translate a Pydantic AI FunctionToolset to an OATP toolset registry."""
    return {
        "$schema": "https://open-toolset-protocol.org/schemas/toolsets.schema.json",
        "toolset_name": getattr(toolset, "id", None) or "pydantic-ai-toolset",
        "version": "0.0.1",
        "capabilities": {
            "discovery": True,
            "phase_gating": False,
            "capability_negotiation": False,
            "state_attestation": False,
        },
        "tools": [
            {
                "name": t.name,
                "description": t.description or "",
                "phase": phase,
                "category": category,
                "verification_mode": "heuristic",
            }
            for t in toolset.tools
        ],
    }
```

Key translation decisions:
- `phase` and `category` must be assigned by the adapter author — Pydantic AI tools carry no phase metadata
- `verification_mode` defaults to `"heuristic"` for function tools (output is not guaranteed deterministic)
- `ExternalToolset` tools — those executed by a frontend or upstream service — map well to `requires_approval: true`
- `CombinedToolset` maps to OATP's nested `toolsets` array: each constituent toolset becomes a nested registry entry

## 7.3 MCP adapter sketch

MCP ([spec](https://modelcontextprotocol.io)) defines `tools/list` (discover available tools and their schemas) and `tools/call` (execute a specific tool with arguments).

An MCP tools/list response looks like:

```json
{
  "tools": [
    {
      "name": "get_weather",
      "description": "Get current weather information for a location",
      "inputSchema": {
        "type": "object",
        "properties": { "location": { "type": "string" } },
        "required": ["location"]
      }
    }
  ]
}
```

Translation to OATP:

```python
# Conceptual — not normative
def mcp_tools_to_oatp(tools_list_response: dict, *, phase: str) -> dict:
    """Translate MCP tools/list response to OATP toolset registry."""
    tools = []
    for tool in tools_list_response["tools"]:
        oatp_tool = {
            "name": tool["name"],
            "description": tool.get("description", ""),
            "phase": phase,
            "category": "navigation",     # default; override per-tool as appropriate
            "verification_mode": "heuristic",
        }
        # MCP inputSchema can inform instrumented_return.schema_ref if the
        # tool also declares an output schema via annotations
        if "outputSchema" in tool.get("annotations", {}):
            oatp_tool["instrumented_return"] = {
                "required": False,
                "schema_ref": tool["annotations"]["outputSchema"],
            }
        tools.append(oatp_tool)
    return {
        "$schema": "https://open-toolset-protocol.org/schemas/toolsets.schema.json",
        "toolset_name": "mcp-toolset",
        "version": "0.0.1",
        "tools": tools,
    }
```

OATP and MCP are complementary:
- MCP handles *how* tools are discovered and invoked between client and server
- OATP adds *which phase*, *what discipline*, *what preconditions*, and *what structured return* are expected
- An MCP server SHOULD advertise its OATP registry at `/.well-known/toolset.json` alongside its MCP endpoint so consumers can inspect tool discipline before connecting

## 7.4 LangChain adapter sketch

LangChain `Tool` objects carry a `name`, `description`, and `func`. A `Toolkit` groups related tools. Translation is similar to Pydantic AI — phase and category must be assigned by the adapter author. LangChain tools default to `"heuristic"` verification mode.

## 7.5 Conformance for adapters

A framework adapter MUST:
- Produce a `toolsets.json` that validates against `schema.json`
- Preserve framework-specific tool semantics (no behavior changes to the wrapped tools)
- Document which OATP capabilities it enables (at minimum: `discovery: true`)

A framework adapter SHOULD:
- Assign accurate `phase` and `category` values based on the tool's actual role
- Enable `phase_gating` if it intercepts the framework's tool-call hook
- Enable `state_attestation` if it can validate structured outputs before returning to the agent

## 7.6 Reference implementations roadmap (non-normative)

Planned reference adapters. These are stubs — not yet implemented. See [RFC 0003](../RFC/0003-framework-adapters.md) for design discussion.

| Integration | Path | Status |
|---|---|---|
| Pydantic AI | `adapter/integrations/pydantic-ai/` | TODO |
| MCP | `adapter/integrations/mcp/` | TODO |
| LangChain | `adapter/integrations/langchain/` | TODO |
| OpenAI tools | `adapter/integrations/openai-tools/` | TODO |
