# Contributing

## Ways to contribute

- **File an RFC**: propose a spec change, new schema field, or conformance requirement change
- **Add conformance vectors**: contribute test cases to `conformance/vectors/`
- **Fix documentation**: improve clarity, fix typos, add examples
- **Report issues**: open a GitHub issue for bugs, ambiguities, or gaps in the spec

## Filing an RFC

1. Copy [`RFC/0000-template.md`](RFC/0000-template.md) to `RFC/NNNN-short-title.md` (use the next available number)
2. Fill in all sections: Summary, Motivation, Detailed design, Drawbacks, Alternatives, Unresolved questions
3. Open a pull request
4. Discussion happens in the PR — address feedback, revise the RFC
5. Maintainers merge (accepted) or close with rationale (rejected)

RFCs that change normative protocol semantics, JSON Schema fields, conformance requirements, or adapter exit codes MUST go through this process. Documentation-only changes do not require an RFC.

## Adding conformance vectors

Conformance vectors live in `conformance/vectors/`. Each vector is a directory containing:

- `toolsets.json`: the registry under test
- `cmd.txt`: the command being validated
- `expected.json`: `{ "verdict": "allow" | "deny" | "error", "exit_code": N }`

See [`conformance/README.md`](conformance/README.md) for vector naming conventions and the full format spec.

## Proposing schema changes

Schema changes (`schemas/toolsets.schema.json`) require a corresponding spec change in `spec/02-toolsets-schema.md`. Open both in the same PR. Schema changes that add required fields are breaking and require a major version bump per the governance policy.

## Code style (adapter)

The reference adapter is in Rust. Follow standard `rustfmt` formatting. Run `cargo clippy` before submitting. No `unsafe` blocks without explicit justification in the PR.

## Code of conduct

All contributors are expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md).
