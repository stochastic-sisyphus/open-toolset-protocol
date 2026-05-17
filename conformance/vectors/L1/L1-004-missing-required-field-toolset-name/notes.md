Registry omits `toolset_name`, which is `required` in the JSON Schema root object.
Confirms the adapter rejects the registry and exits 3 before processing any invocation.
Spec: `spec/02-toolsets-schema.md` §"Top-level fields", `schemas/toolsets.schema.json` `required: ["toolset_name", "version"]`, `spec/01-protocol.md` §4.
