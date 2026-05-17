# OATP adapter for Pydantic AI

Reference adapter for Pydantic AI. Not yet implemented. See [RFC 0003](../../../RFC/0003-framework-adapters.md).

This integration will wrap Pydantic AI `FunctionToolset`, `ExternalToolset`, and `CombinedToolset` instances in an OATP-conformant surface, translating tool metadata into `toolsets.json` registry entries and enforcing phase gating at invocation time.
