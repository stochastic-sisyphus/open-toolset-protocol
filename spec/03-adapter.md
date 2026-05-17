# Adapter Specification

This document specifies the behavior of a conformant OATP adapter. The reference implementation is the `oatp` binary in `adapter/`.

OATP defines exactly one adapter role: the universal `oatp` binary. Framework-specific runtime adapters are out of scope - frameworks contribute registry translators (see spec/07-registry-translators.md), not parallel adapters.

## Binary shape

The adapter is one binary with many subcommands:

- `oatp validate <toolsets.json>` - validate a registry against the schema
- `oatp resolve [<toolsets.json>]` - resolve nested registries, apply priority-based category substitution, and print the flattened manifest
- `oatp exec -- <cmd> [args...]` - validate and execute a command
- `oatp trace` - read or tail the event sink

`exec` MUST use `--` to separate adapter flags from the command being executed. The adapter MUST treat everything after `--` as the command and its arguments.

## Crate-binding rule

Any matcher, validator, parser, resolver, serializer, execution engine, or trace sink that already exists as a mature crate MUST be bound directly rather than reimplemented. Glue code that adapts those crates to the OATP binary shape MUST remain at or below 150 LOC per integration boundary.

The binding rule exists so that OATP implementations inherit the maturity of the underlying ecosystem instead of reconstructing it with bespoke shell glue.

## Registry loading

The adapter MUST locate the Toolset Registry in this order:

1. `$OATP_TOOLSET` environment variable (path to `toolsets.json`)
2. `./toolsets.json` (current working directory)
3. `~/.config/oatp/toolsets.json`

If the located file fails JSON Schema validation against `schemas/toolsets.schema.json`, the adapter MUST exit with code 3 and emit a `toolset.schema_error` trace event before any invocation is processed.

If no registry is found at any location, the adapter MUST exit with code 4.

## Resolve flow

`oatp resolve` MUST flatten nested registries, apply `$ref` resolution depth-first in declaration order, and produce a deterministic manifest. When multiple tools satisfy the same category, the adapter MUST prefer the tool with the highest numeric `priority`; ties MUST break deterministically by tool name.

## Exec flow

```
1. Parse: extract cmd and args from argv after --
2. Load: read and validate toolsets.json (cached for process lifetime)
3. Resolve: determine the candidate tool and apply priority-based category substitution
4. Validate: apply policy
   → deny: emit tool.deny, exit 2
   → approval required: emit tool.approval_requested, await signal
   → allow: continue
5. Emit: tool.exec.start event
6. Execute: spawn subprocess, inherit env + apply overrides
7. Stream: pass stdout/stderr with optional redaction
8. Wait: collect exit code and wall-clock duration
9. Emit: tool.exec.end event
10. Exit: with subprocess exit code (or 1 if timeout)
```

## Environment variables

| Variable | Description |
|---|---|
| `OATP_TOOLSET` | Path to the Toolset Registry. Highest precedence |
| `OATP_TRACE_SINK` | Override `instrumentation.event_sink` at runtime |
| `OATP_LOG_LEVEL` | Adapter log verbosity: `error`, `warn`, `info`, `debug`. Default: `warn` |

## Exit codes

| Code | Meaning |
|---|---|
| 0 | Command executed successfully (subprocess exit 0) |
| 1 | Exec failure: subprocess returned non-zero, or timeout exceeded |
| 2 | Policy rejection: call denied by toolset policy |
| 3 | Schema error: Toolset Registry failed validation |
| 4 | Toolset not found: no registry located |

The adapter MUST return the subprocess's own exit code when execution succeeds. If the subprocess exits non-zero, the adapter MUST return exit code 1 (not the subprocess code directly), and MUST include the actual subprocess exit code in the `tool.exec.end` trace event's `exit_code` field.

## Stdout and stderr passthrough

The adapter MUST stream stdout and stderr from the subprocess to its own stdout and stderr respectively, preserving order as closely as possible. If redaction is enabled, the adapter MUST apply patterns before writing to its output streams.

The adapter MUST NOT buffer or suppress stdout/stderr on successful execution. For denied or errored invocations, the adapter MUST write a human-readable rejection reason to stderr.

## Approval workflows

When a tool has `requires_approval: true`, the adapter:

1. MUST emit `tool.approval_requested` with full invocation details
2. MUST pause execution
3. MUST wait for an external approval signal (mechanism is implementation-defined; the reference adapter polls `$OATP_APPROVAL_SOCKET`)
4. On approval: proceed with exec
5. On denial or timeout: emit `tool.deny`, exit 2

## Phase gating algorithm

The adapter MUST track `active_phase`, initialized to `reconnaissance` at session start.

Phase transition is an explicit agent intent: `oatp phase --set <phase>`. The adapter MUST process phase transitions as follows:

```
on phase_transition(from, to):
    required_for_from = [t for t in registry.tools
                         if t.required == true and t.phase == from]
    satisfied = [t for t in required_for_from
                 if t.name in trace.invoked_in_phase(from)]
    missing = set(required_for_from) - set(satisfied)
    if missing:
        reject(exit=2, reason="required_tool_skipped",
               tools=[t.name for t in missing])
    active_phase = to
    emit("phase.transition", from=from, to=to)
```

On each tool invocation:

```
on tool_invoke(tool_name, args):
    tool = registry.resolve(tool_name)
    if tool is None:
        reject(exit=2, reason="unknown_tool")
    if tool.phase != active_phase and tool.phase != "any":
        reject(exit=2, reason="phase_gate_violation",
               expected=active_phase, got=tool.phase)
    if tool.required:
        trace.record(active_phase, tool.name)
    validate_args(tool, args)
    return exec(tool, args)
```

`allowedDisciplineCategories` is a derived view: the union of `tool.category` for all tools whose `tool.phase` matches `active_phase` or is `"any"`.

## Required tool enforcement

A tool with `required: true` MUST be invoked at least once in its declared phase before the agent may transition to the next phase. Enforcement happens at **transition time** - when `oatp phase --set` is called, the adapter scans the phase trace log.

If any required tool for the exiting phase has not been invoked, the adapter MUST:
1. Reject the transition with exit code 2
2. Emit reason `required_tool_skipped` with the list of missing tool names
3. Leave `active_phase` unchanged

The agent may then invoke the missing tools and retry the transition.

## Caching

The adapter SHOULD cache the loaded and validated Toolset Registry for the duration of its process lifetime to avoid repeated I/O. If the registry file changes on disk, the behavior is implementation-defined (the reference adapter does not hot-reload).
