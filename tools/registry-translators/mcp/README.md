# OATP adapter for MCP

Reference adapter for the Model Context Protocol (MCP). Not yet implemented. See [RFC 0003](../../../RFC/0003-framework-adapters.md).

This integration will translate MCP `tools/list` responses into `toolsets.json` registry entries and intercept `tools/call` requests to apply OATP policy evaluation and phase gating before forwarding to the MCP server.
