Tool entry omits `verification_mode`, which is `required` in `$defs.tool`. The field drives L4 attestation contracts - omitting it must be a schema error at load time.
Confirms the adapter cannot silently default `verification_mode`; every tool must declare its verification contract explicitly.
Spec: `spec/02-toolsets-schema.md` §"Tool definition fields", `schemas/toolsets.schema.json` `$defs.tool.required`.
