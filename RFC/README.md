# RFC Process

OATP evolves through RFCs (Requests for Comments). An RFC is a design document describing a proposed change to the spec, schema, or conformance requirements.

## When an RFC is required

An RFC is required for:

- Changes to normative protocol semantics (`spec/01-protocol.md`)
- New or modified fields in the Toolset Registry schema (`spec/02-toolsets-schema.md`, `schemas/toolsets.schema.json`)
- Changes to adapter behavior or exit codes (`spec/03-adapter.md`)
- New or modified trace event types or fields (`spec/04-instrumentation.md`)
- New conformance levels or changes to existing level requirements (`spec/05-conformance.md`)
- Any change that would require existing conformant implementations to update

An RFC is not required for:

- Documentation fixes, typo corrections
- New or updated examples in `examples/`
- New conformance vectors that test existing behavior
- Non-normative clarifications that don't change behavior

## Process

1. **Copy the template**: `cp RFC/0000-template.md RFC/NNNN-short-title.md` using the next available number
2. **Fill it in**: complete all sections — Summary, Motivation, Detailed design, Drawbacks, Alternatives, Unresolved questions
3. **Open a PR**: title format `RFC NNNN: Short title`
4. **Discussion**: feedback happens in the PR. Update the RFC in response to comments.
5. **Decision**: maintainers merge (accepted) or close with rationale (rejected)

Merged RFCs are canonical. The corresponding spec changes MUST land in the same PR or a follow-up PR that references the RFC.

## RFC numbering

Use the next sequential 4-digit number (e.g. `0001`, `0002`). The template is `0000`.

## Filed RFCs

| RFC | Title | Status |
|---|---|---|
| [RFC 0001](0001-discipline-categories.md) | Discipline Categories | Draft |
| [RFC 0002](0002-gps-protocol.md) | GPS Protocol | Draft |
| [RFC 0003](0003-framework-adapters.md) | Framework Adapters | Draft |
| [RFC 0004](0004-formal-verification.md) | Formal Verification Conformance (L5 and L6) | Draft |

## Status labels

| Label | Meaning |
|---|---|
| `rfc: draft` | Open for discussion |
| `rfc: final-comment-period` | Last call before decision |
| `rfc: accepted` | Merged |
| `rfc: rejected` | Closed with rationale |
| `rfc: withdrawn` | Closed by author |
