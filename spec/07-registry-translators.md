# Registry Translators

OATP is not a replacement for existing agent toolset frameworks. It is the interop layer that makes those frameworks' toolsets composable, inspectable, and policy-enforced across implementations.

This document describes the **translator pattern** - how to convert an existing framework's native toolset into a conformant `toolsets.json` without changing the framework's internals and without creating a parallel runtime enforcement layer.

## 7.1 The translator pattern

A registry translator is a one-shot tool that:

1. **Reads** the framework's native toolset (Pydantic AI `FunctionToolset`, MCP `tools/list` response, LangChain `Toolkit`, OpenAI tool definitions, etc.)
2. **Translates** each native tool into an OATP `tool` entry, assigning `phase`, `category`, `verification_mode`, and any `requires_prior` constraints appropriate for the tool's role
3. **Emits** a `toolsets.json` document conforming to `schemas/toolsets.schema.json` and exits

Translators do NOT intercept tool execution. Once a `toolsets.json` has been emitted, runtime enforcement flows entirely through the universal `oatp` binary at `adapter/`. OATP has exactly one runtime adapter role - every agent, regardless of framework, wraps its shell exec through `oatp`. Per-framework runtime adapters would re-fragment what OATP unifies.

A registry translator MUST:
- Produce a `toolsets.json` that validates against `schema.json`
- Preserve framework-specific tool semantics - no behavior changes
- Emit and exit; it MUST NOT wrap execution or intercept tool-call hooks at runtime

## 7.2 Pydantic AI translator sketch

Pydantic AI ([docs](https://pydantic.dev/docs/ai/tools-toolsets/toolsets/)) provides `FunctionToolset`, `ExternalToolset`, and `CombinedToolset`. A `FunctionToolset` groups locally defined Python functions as agent tools. An `ExternalToolset` allows agents to call tools executed by an upstream service. `CombinedToolset` merges multiple toolsets.

Translation sketch (conceptual - not normative):

```python
# Conceptual - not normative
from pydantic_ai.tools import FunctionToolset
import json

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

# Emit and exit - do not intercept execution
print(json.dumps(to_oatp(my_toolset, phase="reconnaissance", category="navigation")))
```

Key translation decisions:
- `phase` and `category` must be assigned by the translator author - Pydantic AI tools carry no phase metadata
- `verification_mode` defaults to `"heuristic"` for function tools (output is not guaranteed deterministic)
- `ExternalToolset` tools - those executed by a frontend or upstream service - map well to `requires_approval: true`
- `CombinedToolset` maps to OATP's nested `toolsets` array: each constituent toolset becomes a nested registry entry

## 7.3 MCP translator sketch

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

Translation to OATP (emit and exit - not normative):

```python
# Conceptual - not normative
import json

def mcp_tools_to_oatp(tools_list_response: dict, *, phase: str) -> dict:
    """Translate MCP tools/list response to an OATP toolset registry."""
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

# Emit and exit - do not intercept tools/call
print(json.dumps(mcp_tools_to_oatp(response, phase="reconnaissance")))
```

OATP and MCP are complementary:
- MCP handles *how* tools are discovered and invoked between client and server
- OATP adds *which phase*, *what discipline*, *what preconditions*, and *what structured return* are expected
- An MCP server SHOULD advertise its OATP registry at `/.well-known/toolset.json` alongside its MCP endpoint so consumers can inspect tool discipline before connecting

## 7.4 LangChain translator sketch

LangChain `Tool` objects carry a `name`, `description`, and `func`. A `Toolkit` groups related tools. Translation is similar to Pydantic AI - phase and category must be assigned by the translator author. LangChain tools default to `"heuristic"` verification mode. The translator emits `toolsets.json` and exits; it does not wrap `agent.run()` or intercept tool-call hooks.

## 7.5 Conformance for translators

A registry translator MUST:
- Produce a `toolsets.json` that validates against `schema.json`
- Preserve framework-specific tool semantics (no behavior changes to the source tools)
- Document which OATP capabilities are reflected (at minimum: `discovery: true`)
- Emit and exit - no runtime interception

A registry translator SHOULD:
- Assign accurate `phase` and `category` values based on the tool's actual role
- Note `phase_gating: false` in the emitted registry (enforcement is the `oatp` binary's job, not the translator's)

## 7.6 Reference implementations roadmap (non-normative)

Planned translators. These are stubs - not yet implemented. See [RFC 0003](../RFC/0003-registry-translators.md) for design discussion.

| Translator | Path | Status |
|---|---|---|
| Pydantic AI | `tools/registry-translators/pydantic-ai/` | TODO |
| MCP | `tools/registry-translators/mcp/` | TODO |
| LangChain | `tools/registry-translators/langchain/` | TODO |
| OpenAI tools | `tools/registry-translators/openai-tools/` | TODO |
