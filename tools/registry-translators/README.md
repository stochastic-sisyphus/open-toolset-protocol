# Registry translators

These are **one-shot translators**, not adapters. They read a framework's native toolset and emit a conformant `toolsets.json`. They do NOT intercept tool execution.

After translation, execution flows through the single universal adapter: the `oatp` binary at `adapter/`.

## Why translators, not adapters

OATP has exactly one adapter: `oatp` (Rust binary under `adapter/`). Every agent — regardless of framework — wraps its shell exec through `oatp`. The protocol's value is the universal contract layer. Per-framework runtime adapters would re-fragment what OATP unifies.

A translator's job ends when it has written a valid `toolsets.json`. The universal adapter takes it from there.

## Layout

Each translator subdirectory contains:
- A reference implementation that reads the framework's native registry
- An emitted-sample `toolsets.json` showing the output
- A `README.md` documenting the framework-specific input shape

## Translators

- `pydantic-ai/` — reads `Toolset` / `FunctionToolset` / `ExternalToolset` / `MCPToolset`
- `mcp/` — reads `tools/list` from MCP servers
- `langchain/` — reads `Tool` / `Toolkit`
- `openai-tools/` — reads OpenAI function-calling definitions

Status: stubs. See RFC 0003 for the design discussion.
