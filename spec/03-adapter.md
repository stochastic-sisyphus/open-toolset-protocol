# Adapter Specification

This document specifies the behavior of a conformant OATP adapter. The reference implementation is the `oatp` binary in `adapter/`.

## Entry point

The canonical adapter invocation is:

```
oatp exec -- <cmd> [args...]
```

The `--` separator is REQUIRED to clearly delimit adapter flags from the command being validated. The Adapter MUST treat everything after `--` as the command and its arguments.

Future subcommands:

- `oatp check <toolsets.json>` — validate a registry against the schema (L1 conformance)
- `oatp trace` — tail the event sink

## Registry loading

The Adapter MUST locate the Toolset Registry in this order:

1. `$OATP_TOOLSET` environment variable (path to `toolsets.json`)
2. `./toolsets.json` (current working directory)
3. `~/.config/oatp/toolsets.json`

If the located file fails JSON Schema validation (against `schemas/toolsets.schema.json`), the Adapter MUST exit with code 3 and emit a `toolset.schema_error` trace event before any invocation is processed.

If no registry is found at any location, the Adapter MUST exit with code 4.

## Exec flow

```
1. Parse: extract cmd and args from argv after --
2. Load: read and validate toolsets.json (cached for process lifetime)
3. Validate: apply policy (see spec/01-protocol.md §2.2)
   → deny: emit tool.deny, exit 2
   → approval required: emit tool.approval_requested, await signal
   → allow: continue
4. Emit: tool.exec.start event
5. Execute: spawn subprocess, inherit env + apply overrides
6. Stream: pass stdout/stderr with optional redaction
7. Wait: collect exit code and wall-clock duration
8. Emit: tool.exec.end event
9. Exit: with subprocess exit code (or 1 if timeout)
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

The Adapter MUST return the subprocess's own exit code when execution succeeds. If the subprocess exits non-zero, the Adapter MUST return exit code 1 (not the subprocess code directly), and MUST include the actual subprocess exit code in the `tool.exec.end` trace event's `exit_code` field.

## Stdout and stderr passthrough

The Adapter MUST stream stdout and stderr from the subprocess to its own stdout and stderr respectively, preserving order as closely as possible. If redaction is enabled, the Adapter MUST apply patterns before writing to its output streams.

The Adapter MUST NOT buffer or suppress stdout/stderr on successful execution. For denied or errored invocations, the Adapter MUST write a human-readable rejection reason to stderr.

## Approval workflows

When a tool has `requires_approval: true`, the Adapter:

1. MUST emit `tool.approval_requested` with full invocation details
2. MUST pause execution
3. MUST wait for an external approval signal (mechanism is implementation-defined; the reference adapter polls `$OATP_APPROVAL_SOCKET`)
4. On approval: proceed with exec
5. On denial or timeout: emit `tool.deny`, exit 2

## Caching

The Adapter SHOULD cache the loaded and validated Toolset Registry for the duration of its process lifetime to avoid repeated I/O. If the registry file changes on disk, the behavior is implementation-defined (the reference adapter does not hot-reload).
