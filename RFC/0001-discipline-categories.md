# RFC 0001 - Standardized Discipline Categories

- **RFC number**: 0001
- **Start date**: 2026-05-17
- **Status**: Draft
- **Target spec version**: 0.1

## Summary

Define an authoritative-but-open vocabulary for the tool `category` field in OATP toolset registries. "Open" means any implementer may use any string value; "authoritative" means OATP publishes and maintains a canonical list. This RFC specifies the initial 14 canonical categories and establishes the process for adding new ones.

## Motivation

The `category` field is the mechanism by which agents reason about tool substitution and toolset coverage gaps. Without a shared vocabulary, agents cannot answer the question "do I have a `semantic-query` tool available?" - they would need to compare names or descriptions.

Categories enable:

- **Substitution reasoning**: an agent that needs a `syntax-match-rewrite` tool can select from Comby, ast-grep, Semgrep, or Amber - any conformant implementation will do for the class of tasks. The agent doesn't need to hardcode tool names.
- **Coverage gap detection**: a toolset can be analyzed for missing categories at each phase - for example, a `surgery` phase with no `merge-diff` tool is a gap an agent can surface before attempting mutations.
- **Registry composition**: when composing toolsets via `$ref`, the merged set can be inspected for category overlap and conflicts without reading tool descriptions.

The risk of a closed enum is rigidity - new tool classes (e.g. a new SMT-backed synthesis approach, a new cross-language semantic engine) need an RFC to register, slowing adoption. An open vocabulary avoids this while the canonical list provides consistency.

## Detailed design

### Canonical categories (v0.1)

| Category | Description | Example tools |
|---|---|---|
| `navigation` | Codebase navigation and graph traversal | Kythe (call graphs), graph-sitter, language server protocol clients |
| `syntax-match-rewrite` | Pattern-based structural search and rewrite | Comby, ast-grep, Semgrep, Amber |
| `semantic-query` | Semantic code analysis and querying | CodeQL, Joern, weggli, Infer |
| `transform-at-scale` | Large-scale codemod tools | Coccinelle, OpenRewrite, Rector, Bowler |
| `merge-diff` | Structural diff and merge | difftastic, mergiraf, weave |
| `index-search` | Full-text and trigram code search | Zoekt, Livegrep, OpenGrok |
| `semantic-search` | Vector/embedding-based code search | (various) |
| `cross-reference` | Cross-reference and call graph tools | Kythe |
| `config-substrate` | Configuration language parsers and transformers | pglast, sqlglot, yq, crossplane |
| `logic-substrate` | Language-specific AST logic tools | ts-morph, LibCST |
| `verification` | Formal verification tools | TLA+, Aeneas |
| `synthesis-smt` | Synthesis and SMT solver tools | (various) |
| `datalog-logic` | Datalog and logic programming tools | (various) |
| `structural-edit` | AST-based structural editors | (various) |

### Category string format

Category values are lowercase, hyphen-separated strings. No spaces, no underscores, no uppercase. New categories proposed via this RFC MUST follow this format.

### Process for adding new categories

1. Open a PR adding the new category to this RFC document (the canonical list above)
2. Include: name, description, at least one example tool
3. No separate RFC number required - additions to RFC 0001 are minor
4. Breaking changes (renaming or removing categories) require a new RFC

### Backward compatibility

Category is an open vocabulary - adapters MUST NOT reject tool entries with unknown `category` values. Unknown categories are valid; they simply don't appear in the canonical list. Conformance tooling SHOULD warn on unrecognized category values but MUST NOT fail.

## Drawbacks

**Taxonomy debates.** Some tools span multiple categories (e.g. Semgrep does both `syntax-match-rewrite` and `semantic-query`). The spec position is: choose the category that describes the tool's primary use in the declaring toolset. A tool may appear multiple times in a registry under different names with different categories if needed.

**Category sprawl.** Open vocabulary means anyone can add custom values. This reduces interoperability for uncommon categories. The canonical list mitigates this for common cases.

## Alternatives

**Closed enum in JSON Schema.** Maximizes interoperability but blocks new tool classes without a spec change. Too rigid for a rapidly evolving tooling landscape.

**No taxonomy.** Agents compare by tool name. This requires name-coupling and prevents substitution reasoning. Rejected as insufficient for the substitution use case.

**Hierarchical DAG of categories.** For example: `syntax-match-rewrite` is-a `transform`. This is appealing but adds resolution complexity and the inheritance semantics are unclear when composing toolsets. Deferred to a future RFC.

## Unresolved questions

1. Should categories form a DAG? (e.g. `syntax-match-rewrite` is-a `transform`, `semantic-query` is-a `analysis`) This would allow substitution at multiple levels of specificity.
2. Should toolset composition track category coverage and warn on gaps? This might belong in adapter spec rather than here.
3. Should there be a `general` or `utility` catch-all category? Current position: no - encourage explicit categorization.
