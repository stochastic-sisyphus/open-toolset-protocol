Tool declares `"phase": "destruction"`, which is not in the schema enum `["reconnaissance", "surgery", "instrumentation", "any"]`.
Confirms the adapter rejects any tool with an unrecognised phase value; adapters MUST NOT invent or tolerate phase names outside the protocol's enumeration.
Spec: `spec/02-toolsets-schema.md` §"Tool definition fields", `schemas/toolsets.schema.json` `$defs.tool.properties.phase.enum`.
