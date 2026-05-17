Tool entry omits `phase`, which is `required` in the `$defs.tool` schema object alongside `name`, `category`, and `verification_mode`.
Without a phase declaration the adapter cannot enforce phase gating — the schema makes this a hard error, not a default.
Spec: `spec/02-toolsets-schema.md` §"Tool definition fields", `schemas/toolsets.schema.json` `$defs.tool.required`.
