# OATP adapter for LangChain

Reference adapter for LangChain. Not yet implemented. See [RFC 0003](../../../RFC/0003-framework-adapters.md).

This integration will wrap LangChain `BaseTool` and `Toolkit` instances in an OATP-conformant surface, translating tool metadata into `toolsets.json` registry entries and enforcing phase gating at invocation time.
