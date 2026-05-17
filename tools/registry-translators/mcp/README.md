# MCP registry translator

Translator for the Model Context Protocol (MCP). Reads `tools/list` responses from MCP servers and emits a conformant `toolsets.json`. Not yet implemented. See [RFC 0003](../../RFC/0003-registry-translators.md).

This is a **one-shot translator**, not a runtime adapter. It does not intercept tool execution. Once it has written a valid `toolsets.json`, execution flows through the universal `oatp` binary at `adapter/`.
