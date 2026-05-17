# Toolsets Schema

This document normatively describes the structure of `toolsets.json`, the Toolset Registry. The machine-readable JSON Schema is in `schemas/toolsets.schema.json`.

## Top-level fields

| Field | Type | Required | Description |
|---|---|---|---|
| `$schema` | string | RECOMMENDED | JSON Schema URI for this document. SHOULD be `https://open-toolset-protocol.org/schemas/toolsets.schema.json` |
| `version` | string | REQUIRED | OATP spec version this registry targets. Format: `"0.x"` |
| `name` | string | REQUIRED | Human-readable name for this toolset (e.g. `"claude-code-default"`) |
| `description` | string | OPTIONAL | Short description of the toolset's purpose |
| `tools` | array | REQUIRED | Array of tool definitions. MAY be empty (all tools denied when `default_action` is `"deny"`) |
| `policies` | object | OPTIONAL | Global policy rules applied to all invocations |
| `instrumentation` | object | OPTIONAL | Instrumentation configuration |
| `redaction` | object | OPTIONAL | Redaction rules for stdout/stderr |

## Tool definition fields

Each entry in `tools` is an object with the following fields:

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | string | REQUIRED | Unique identifier for this tool entry (e.g. `"rg-read-only"`) |
| `command` | string | REQUIRED | The binary or command name (e.g. `"rg"`, `"git"`) |
| `description` | string | OPTIONAL | Human-readable description of this tool entry's purpose |
| `args_pattern` | object | OPTIONAL | Allow/deny patterns for arguments |
| `args_pattern.allow` | string | OPTIONAL | Regex. If present, the full args string MUST match for the call to be allowed |
| `args_pattern.deny` | string | OPTIONAL | Regex. If the full args string matches, the call is denied regardless of `allow` |
| `cwd_constraint` | string | OPTIONAL | Regex. The invocation's working directory MUST match |
| `timeout` | integer | OPTIONAL | Maximum execution time in seconds. Overrides `policies.default_timeout` |
| `requires_approval` | boolean | OPTIONAL | If `true`, invocation is paused and emits `tool.approval_requested` before executing |
| `instrumentation_overrides` | object | OPTIONAL | Per-tool overrides for `instrumentation` fields |

## Policies fields

| Field | Type | Required | Description |
|---|---|---|---|
| `default_action` | string | OPTIONAL | `"allow"` or `"deny"`. Default: `"deny"` |
| `default_timeout` | integer | OPTIONAL | Default timeout in seconds for all tools unless overridden |
| `banned_patterns` | array of strings | OPTIONAL | Regex patterns matched against the full command string (cmd + args). Any match causes immediate denial |
| `path_allowlist` | array of strings | OPTIONAL | Glob patterns. File path arguments MUST match at least one entry if this list is non-empty |
| `path_denylist` | array of strings | OPTIONAL | Glob patterns. File path arguments MUST NOT match any entry |

### Evaluation order

1. `banned_patterns` is checked first. A match denies immediately.
2. `path_denylist` is checked. A match denies immediately.
3. `path_allowlist` is checked. If non-empty and no match, deny.
4. Tool-level `args_pattern.deny` is checked. A match denies.
5. Tool-level `args_pattern.allow` is checked. If present and no match, deny.
6. If all pass, allow.

## Instrumentation fields

| Field | Type | Required | Description |
|---|---|---|---|
| `trace_events` | boolean | OPTIONAL | Emit trace events. Default: `true` |
| `event_sink` | string | OPTIONAL | Where to emit events: `"stdout"`, a file path, or an HTTP URL. Default: `"stdout"` |
| `redact_secrets` | boolean | OPTIONAL | Apply redaction rules before returning stdout/stderr. Default: `false` |

## Redaction fields

| Field | Type | Required | Description |
|---|---|---|---|
| `patterns` | array of objects | OPTIONAL | List of redaction rules |
| `patterns[].name` | string | REQUIRED | Identifier for this redaction rule (used in trace events) |
| `patterns[].regex` | string | REQUIRED | Pattern to match in stdout/stderr |
| `patterns[].replacement` | string | OPTIONAL | Replacement text. Default: `"[REDACTED]"` |

## Example (minimal)

```json
{
  "$schema": "https://open-toolset-protocol.org/schemas/toolsets.schema.json",
  "version": "0.1",
  "name": "minimal",
  "tools": [
    {
      "id": "rg-search",
      "command": "rg",
      "description": "Read-only file search"
    }
  ],
  "policies": {
    "default_action": "deny"
  }
}
```

See `examples/` for full working registries.
