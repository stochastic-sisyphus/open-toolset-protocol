# Governance

## Model

OATP uses a **BDFL-emeritus + RFC-based** governance model.

The initial author acts as BDFL (Benevolent Dictator For Life) during the bootstrap phase. Once the spec reaches v1.0 and a contributor community forms, the BDFL role transitions to emeritus and governance shifts fully to RFC consensus.

## Decision making

All normative spec changes - protocol semantics, schema fields, conformance requirements, adapter behavior - land via RFC merged to `main`.

Non-normative changes (documentation fixes, example updates, typos, conformance vector additions) may be merged directly by maintainers without a formal RFC.

## RFC process

See [`RFC/README.md`](RFC/README.md) for the full process.

In short: copy the template, open a pull request, discussion happens in the PR, merge = accepted. Rejected RFCs are closed with a rationale comment.

## Versioning

- **Patch** (`0.x.y`): backward-compatible clarifications, non-normative changes.
- **Minor** (`0.x`): backward-compatible additions (new optional fields, new conformance levels).
- **Major** (`x.0`): breaking changes to the spec. Requires an RFC and a deprecation window for existing implementations.

Breaking spec changes MUST bump the major version. Implementations MUST advertise which spec version they conform to.

## Maintainers

Maintainers are contributors with merge access. They are responsible for:

- Reviewing and merging non-RFC PRs
- Shepherding RFCs through discussion
- Releasing new spec versions

Maintainer list is in `MAINTAINERS.md` (to be created when the first external contributor joins).
