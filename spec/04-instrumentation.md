# Instrumentation

Every tool invocation produces one or more trace events. Trace events are the protocol's answer to "what happened." They are emitted as JSONL (one JSON object per line) to the configured event sink.

## Event sink

The event sink is configured via `instrumentation.event_sink` in the Toolset Registry, or overridden at runtime via `$OATP_TRACE_SINK`.

Supported sink values:

| Value | Behavior |
|---|---|
| `"stdout"` | Events written to the Adapter's stdout, interleaved with subprocess output |
| `"stderr"` | Events written to the Adapter's stderr |
| A file path | Events appended to the file (created if absent) |
| An HTTP URL | Events POSTed as `application/x-ndjson` |

If `instrumentation.trace_events` is `false`, no events are emitted regardless of sink configuration. The Adapter MUST still enforce policy; it MUST NOT skip enforcement because instrumentation is disabled.

## Common fields

All trace events share these fields:

| Field | Type | Description |
|---|---|---|
| `ts` | string | RFC 3339 timestamp with millisecond precision (e.g. `"2026-05-17T14:23:01.042Z"`) |
| `event` | string | Event type (see below) |
| `tool_id` | string | The `id` from the matched tool entry, or `null` if no entry matched |
| `cmd` | string | The command as submitted (not redacted) |
| `args` | array of strings | Arguments as submitted |
| `cwd` | string | Working directory at invocation time |
| `oatp_version` | string | Spec version the adapter implements |

## Event types

### `tool.invoke`

Emitted immediately when the Adapter receives an invocation request, before validation.

Additional fields: none beyond common fields.

### `tool.allow`

Emitted when an invocation passes all policy checks and is permitted to execute.

Additional fields:

| Field | Type | Description |
|---|---|---|
| `policy_match` | string | Which rule permitted this call (`"tool_entry"`, `"default_allow"`) |

### `tool.deny`

Emitted when an invocation is rejected by policy.

Additional fields:

| Field | Type | Description |
|---|---|---|
| `reason` | string | Human-readable rejection reason |
| `policy_match` | string | Which rule triggered the denial (`"banned_pattern"`, `"args_pattern.deny"`, `"path_denylist"`, `"no_tool_entry"`, `"default_deny"`, `"approval_denied"`) |
| `pattern` | string | The specific pattern that matched, if applicable |

### `tool.exec.start`

Emitted immediately before subprocess spawn.

Additional fields:

| Field | Type | Description |
|---|---|---|
| `pid` | integer | Subprocess PID |
| `timeout_secs` | integer or null | Effective timeout for this invocation |

### `tool.exec.end`

Emitted after subprocess termination.

Additional fields:

| Field | Type | Description |
|---|---|---|
| `exit_code` | integer | Subprocess exit code |
| `duration_ms` | integer | Wall-clock execution time in milliseconds |
| `timed_out` | boolean | `true` if the subprocess was terminated due to timeout |
| `redactions` | integer | Number of redaction substitutions applied to output |

### `tool.redact`

Emitted once per stream (stdout/stderr) when redaction is applied.

Additional fields:

| Field | Type | Description |
|---|---|---|
| `stream` | string | `"stdout"` or `"stderr"` |
| `rule_name` | string | The `redaction.patterns[].name` that matched |
| `count` | integer | Number of substitutions in this stream |

### `policy.violation`

Emitted when a banned pattern matches. MAY be emitted in addition to `tool.deny`.

Additional fields:

| Field | Type | Description |
|---|---|---|
| `pattern` | string | The banned pattern that matched |
| `matched` | string | The substring that matched (redacted if it contains secret-like content) |

### `toolset.schema_error`

Emitted when the Toolset Registry fails JSON Schema validation.

Additional fields:

| Field | Type | Description |
|---|---|---|
| `path` | string | Path to the invalid registry file |
| `errors` | array of strings | Validation error messages |

### `toolset.not_found`

Emitted when no Toolset Registry is located.

Additional fields:

| Field | Type | Description |
|---|---|---|
| `searched_paths` | array of strings | Paths checked, in order |

## Example event sequence

```jsonl
{"ts":"2026-05-17T14:23:01.001Z","event":"tool.invoke","tool_id":null,"cmd":"rg","args":["pattern","src/"],"cwd":"/home/user/project","oatp_version":"0.1"}
{"ts":"2026-05-17T14:23:01.002Z","event":"tool.allow","tool_id":"rg-search","cmd":"rg","args":["pattern","src/"],"cwd":"/home/user/project","oatp_version":"0.1","policy_match":"tool_entry"}
{"ts":"2026-05-17T14:23:01.003Z","event":"tool.exec.start","tool_id":"rg-search","cmd":"rg","args":["pattern","src/"],"cwd":"/home/user/project","oatp_version":"0.1","pid":12345,"timeout_secs":30}
{"ts":"2026-05-17T14:23:01.187Z","event":"tool.exec.end","tool_id":"rg-search","cmd":"rg","args":["pattern","src/"],"cwd":"/home/user/project","oatp_version":"0.1","exit_code":0,"duration_ms":184,"timed_out":false,"redactions":0}
```
