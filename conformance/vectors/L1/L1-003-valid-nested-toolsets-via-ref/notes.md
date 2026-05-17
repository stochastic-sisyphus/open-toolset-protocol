Parent registry with no inline tools - it composes entirely via a `$ref` to a child file that contains the tool definitions.
Confirms the adapter can resolve relative-path `$ref` entries and that the resolved manifest is valid.
Spec: `spec/02-toolsets-schema.md` §"Registry composition via `$ref`", §"Nested toolsets (self-registry)".
