# Conformance

This directory contains the conformance level definitions and test vectors for the Open Toolset Protocol.

## Levels

See `spec/05-conformance.md` for the normative definitions. Summary:

| Level | Claim string | What it tests |
|---|---|---|
| L1 | `OATP-L1/0.x` | Schema validation of `toolsets.json` |
| L2 | `OATP-L2/0.x` | Policy enforcement and phase gating |
| L3 | `OATP-L3/0.x` | Full trace event emission |
| L4 | `OATP-L4/0.x` | Redaction and approval workflows |

## Vectors

Test vectors live in `conformance/vectors/`. See **[vectors/README.md](vectors/README.md)** for the normative vector format, file layout, and verdict schema.

Current coverage:

| Level | Cases | What they cover |
|---|---|---|
| L1 | 9 | Schema validation: required fields, version format, phase enum, `$ref` resolution, cycle detection |
| L2 | 10 | Policy order (deny → banned_patterns → allow → default), phase gating, required-tool tracking at transition, `requires_prior` preconditions |

## Running vectors

An implementation passes a vector if, for each event in `sequence` (processed in order), the adapter produces the verdict specified in `verdict.json`. For L1 vectors the sequence is empty - only registry loading is checked.

The test harness is implementation-defined. A conformant implementation MUST pass all vectors for its claimed level before advertising that conformance claim.

## Contributing vectors

To add a vector:

1. Create a directory: `conformance/vectors/<LEVEL>/<LEVEL>-NNN-descriptive-name/`
2. Add `toolsets.json`, `verdict.json`, and `notes.md` (see `vectors/README.md` for format)
3. Open a PR - no RFC required for vectors that test existing behavior

Vectors that expose gaps in the spec (behavior not currently defined) require an RFC.
