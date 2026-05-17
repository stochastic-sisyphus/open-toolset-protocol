# OATP adapter for OpenAI Tools

Reference adapter for the OpenAI tool-calling API. Not yet implemented. See [RFC 0003](../../../RFC/0003-framework-adapters.md).

This integration will translate OpenAI `tools` array entries (JSON Schema function definitions) into `toolsets.json` registry entries and enforce OATP policy evaluation before forwarding tool-call responses to the model.
