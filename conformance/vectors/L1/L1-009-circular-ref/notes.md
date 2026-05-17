`toolsets.json` (cycle-a) `$ref`s `cycle-b.toolsets.json`, which `$ref`s back to `toolsets.json` - a two-node cycle.
The adapter MUST detect cycles by tracking its resolution stack during depth-first traversal and exit 3 with reason `registry_cycle`.
Spec: `spec/02-toolsets-schema.md` §"Registry composition via `$ref`" ("The adapter MUST detect cycles by tracking the resolution stack").
