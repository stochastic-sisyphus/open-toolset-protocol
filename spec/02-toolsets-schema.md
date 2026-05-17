# Toolsets Schema

This document normatively describes the structure of `toolsets.json`, the Toolset Registry. The machine-readable JSON Schema is in `schemas/toolsets.schema.json`.

## Top-level fields

| Field | Type | Required | Description |
|---|---|---|---|
| `$schema` | string | RECOMMENDED | JSON Schema URI. SHOULD be `https://open-toolset-protocol.org/schemas/toolsets.schema.json` |
| `toolset_name` | string | REQUIRED | Human-readable name for this toolset (e.g. `"claude-code-default"`) |
| `version` | string | REQUIRED | Semver string for this registry. Format: `"0.x.y"` |
| `description` | string | OPTIONAL | Short description of the toolset's purpose |
| `manifesto_ref` | string (URI) | OPTIONAL | URI pointing to the manifesto or governing spec for this toolset |
| `capabilities` | object | OPTIONAL | Declared conformance capabilities (see below) |
| `phases` | array of strings | OPTIONAL | Phase names this toolset participates in. SHOULD be a subset of `["reconnaissance", "surgery", "instrumentation"]` |
| `tools` | array | OPTIONAL | Leaf tool definitions |
| `toolsets` | array | OPTIONAL | Nested toolset references (self-registry / composability) |

At least one of `tools` or `toolsets` MUST be present.

## Capabilities object

| Field | Type | Default | Description |
|---|---|---|---|
| `discovery` | boolean | `false` | Adapter supports tool discovery queries and well-known path serving |
| `phase_gating` | boolean | `false` | Adapter enforces `phase` per tool and tracks phase transitions |
| `capability_negotiation` | boolean | `false` | Adapter and agent exchange capability blocks at session start |
| `state_attestation` | boolean | `false` | Adapter enforces structured `instrumented_return` contracts |

## Tool definition fields

Each entry in `tools` is an object with the following fields:

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | string | REQUIRED | Unique name for this tool entry |
| `phase` | string (enum) | REQUIRED | When in the GPS sequence this tool runs: `reconnaissance`, `surgery`, `instrumentation`, or `any` |
| `category` | string | REQUIRED | What kind of tool this is. Open vocabulary — see canonical list below |
| `verification_mode` | string (enum) | REQUIRED | `deterministic`, `heuristic`, or `none` |
| `description` | string | OPTIONAL | Human-readable description of this tool's purpose |
| `binary_repo` | string | OPTIONAL | Upstream source URL for the tool binary |
| `operational_constraints` | array of strings | OPTIONAL | Normative constraints on usage (e.g. `"read-only"`, `"no network"`, `"single-file scope"`) |
| `instrumented_return` | object | OPTIONAL | Structured return contract (see below) |
| `required` | boolean | OPTIONAL | If `true`, this tool MUST be invoked at least once per phase before phase exit. Default: `false` |
| `requires_approval` | boolean | OPTIONAL | If `true`, invocation is paused pending external approval |

## Canonical category values

The `category` field uses an open vocabulary — any string is valid. OATP publishes a canonical list maintained in RFC 0001. Current canonical values:

| Category | Description |
|---|---|
| `navigation` | Codebase navigation and graph traversal |
| `syntax-match-rewrite` | Pattern-based structural search and rewrite (Comby, ast-grep, Semgrep, Amber) |
| `semantic-query` | Semantic code analysis and querying (CodeQL, Joern, weggli, Infer) |
| `transform-at-scale` | Large-scale codemod tools (Coccinelle, OpenRewrite, Rector, Bowler) |
| `merge-diff` | Structural diff and merge (difftastic, mergiraf, weave) |
| `index-search` | Full-text and trigram code search (Zoekt, Livegrep, OpenGrok) |
| `semantic-search` | Vector/embedding-based code search |
| `cross-reference` | Cross-reference and call graph tools (Kythe) |
| `config-substrate` | Configuration language parsers and transformers (pglast, sqlglot, yq, crossplane) |
| `logic-substrate` | Language-specific AST logic tools (ts-morph, LibCST) |
| `verification` | Formal verification tools (TLA+, Aeneas) |
| `synthesis-smt` | Synthesis and SMT solver tools |
| `datalog-logic` | Datalog and logic programming tools |
| `structural-edit` | AST-based structural editors |

Categories let agents reason about substitution: "I need a `semantic-query` tool — any of CodeQL, Joern, or weggli will do." They also let toolsets advertise coverage gaps.

## Instrumented return object

| Field | Type | Required | Description |
|---|---|---|---|
| `required` | boolean | REQUIRED | If `true`, the adapter MUST reject results that don't match `schema_ref` |
| `schema_ref` | string (URI) | OPTIONAL | URI pointing to the JSON Schema the tool's return value MUST satisfy |

## Nested toolsets (self-registry)

A toolset is recursive. `toolsets` is an array of objects that are themselves toolset definitions (same schema as the parent). This enables composition — a parent toolset can include sub-toolsets for organizational or phase separation.

**Disambiguation**: if an entry has `phase` and `category`, it is a tool. If it has `toolset_name`, it is a nested toolset.

**Inheritance**: nested toolsets inherit the parent's `capabilities` unless they declare their own. When capabilities conflict, the parent's value takes precedence (most restrictive wins).

**Resolution**: the adapter resolves the registry depth-first, flattening the tool tree into a single validated manifest at session start. Parent constraints are applied over child constraints.

Example structure:
```json
{
  "toolset_name": "full-agent",
  "version": "0.1.0",
  "toolsets": [
    {
      "toolset_name": "read-only",
      "phases": ["reconnaissance"],
      "tools": [...]
    },
    {
      "toolset_name": "git-ops",
      "phases": ["surgery"],
      "tools": [...]
    }
  ]
}
```

## Policy evaluation

The `policies` block applies globally to all tool invocations. For each invocation, the adapter evaluates policies in this order:

1. If tool name matches any pattern in `policies.deny` → reject (exit 2, reason `denylist_match`). **Deny always wins.**
2. If full command line matches any regex in `policies.banned_patterns` → reject (exit 2, reason `banned_pattern_match`).
3. If tool name matches any pattern in `policies.allow` → continue to phase/precondition/required gates.
4. Else if `policies.default_action == "allow"` → continue.
5. Else → reject (exit 2, reason `default_deny`).

Then, after policies pass:

6. `phase` gate: if `tool.phase` does not match `active_phase` and is not `"any"` → reject (exit 2, reason `phase_gate_violation`).
7. `requires_prior`: if non-empty, scan phase trace for prior invocations matching any entry (name or `category:` prefix). If none match → reject (exit 2, reason `precondition_unsatisfied`).
8. `requires_approval` check — pause and emit `tool.approval_requested` if true.
9. If all pass, allow.

**Glob semantics**: `policies.allow` and `policies.deny` use git-style globbing. Patterns match against tool `name`, not against the command line. For command-line pattern matching, use `banned_patterns` (regex).

## Registry composition via `$ref`

A `toolsets.json` may reference other registries using `$ref` entries in the `toolsets` array:

```json
{
  "toolset_name": "my-agent",
  "version": "1.0.0",
  "toolsets": [
    { "$ref": "./toolsets/recon.toolsets.json" },
    { "$ref": "https://example.com/.well-known/toolset.json" },
    { "$ref": "oatp:builtin/safe-defaults" }
  ]
}
```

`$ref` value forms:
- **Relative path** — resolved against the referring file's directory
- **Absolute path** — resolved directly
- **`https://` URL** — fetched; MUST be HTTPS; MUST be a valid OATP registry
- **`oatp:builtin/<name>`** — resolves against the adapter's embedded canonical registries (see `adapter/builtin/`)

Resolution is **transitive** — `$ref`'d registries can themselves reference others. The adapter MUST detect cycles by tracking the resolution stack. A cycle causes exit 3, reason `registry_cycle`, with `cycle_path` listing the stack.

Merge semantics: each resolved registry's tools are flattened into the parent tool list. The parent's `capabilities` win on conflict. Each tool's `phase` and `category` are preserved from its source registry.

Resolution is **depth-first, in declaration order** — identical inputs always produce identical resolved registries (deterministic).

## Evaluation order (full)

## Example (minimal)

```json
{
  "$schema": "https://open-toolset-protocol.org/schemas/toolsets.schema.json",
  "toolset_name": "minimal",
  "version": "0.1.0",
  "tools": [
    {
      "name": "rg-search",
      "phase": "reconnaissance",
      "category": "index-search",
      "verification_mode": "deterministic",
      "description": "Read-only file search",
      "operational_constraints": ["read-only"]
    }
  ]
}
```

See `examples/` for full working registries.
