Registry uses `"version": "1.0"` — two-component semver, which fails the schema's `pattern: "^\d+\.\d+\.\d+$"` (requires three components).
Confirms the adapter enforces the exact semver pattern and rejects the registry with exit 3.
Spec: `spec/02-toolsets-schema.md` §"Top-level fields" (version field), `schemas/toolsets.schema.json` `properties.version.pattern`.
