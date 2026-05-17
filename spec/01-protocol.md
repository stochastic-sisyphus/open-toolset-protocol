# OATP Protocol Semantics

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119).

## 1. Roles

### 1.1 Agent

The Agent is any AI-driven process that invokes tools to accomplish a task. The Agent is not assumed to be well-behaved. It MAY attempt to call banned tools, pass disallowed arguments, or exceed policy constraints. The protocol is designed to handle this safely.

### 1.2 Adapter

The Adapter is the enforcement boundary between the Agent and the operating system. The Adapter:

- MUST load the active Toolset Registry before processing any invocation
- MUST validate each invocation against the active toolset before execution
- MUST emit a trace event for every invocation (allowed, denied, or errored)
- MUST NOT execute a call that violates policy
- MUST pass stdout and stderr of allowed invocations back to the Agent, with optional redaction applied

The reference adapter is the `oatp` binary. Any conformant implementation of this role MUST satisfy all MUST requirements in this document.

### 1.3 Toolset Registry

The Toolset Registry is the machine-readable policy document (`toolsets.json`). It declares which tools are allowed, under what conditions, and what instrumentation to emit. See `spec/02-toolsets-schema.md` for the normative schema.

## 2. Lifecycle

Every tool invocation follows this lifecycle:

```
Agent → Adapter: invocation request (cmd + args + cwd)
Adapter: load registry (if not cached)
Adapter: validate invocation against active toolset
  → if policy denies: emit tool.deny event, return exit code 2, stop
  → if requires_approval: pause, emit tool.approval_requested, wait for external signal
  → if policy allows: continue
Adapter: emit tool.exec.start event
Adapter: execute command
Adapter: apply redaction to stdout/stderr (if configured)
Adapter: emit tool.exec.end event (with exit code, duration)
Adapter → Agent: stdout, stderr, exit code
```

### 2.1 Discovery

The Adapter MUST locate the active Toolset Registry in the following order of precedence:

1. Path specified by the `OATP_TOOLSET` environment variable
2. `toolsets.json` in the current working directory
3. `~/.config/oatp/toolsets.json`

If no registry is found, the Adapter MUST reject all invocations with exit code 4 and emit a `toolset.not_found` trace event.

### 2.2 Validation

For each invocation, the Adapter MUST:

1. Identify the tool by matching `cmd` against the `tools[].command` field in the registry
2. Check `policies.banned_patterns` — if the full command string matches any pattern, deny
3. If the tool is found: check `args_pattern.allow` and `args_pattern.deny` regexes
4. Check `cwd_constraint` against the actual working directory
5. Check `path_allowlist` and `path_denylist` in policies
6. If all checks pass: allow

If no matching tool entry is found in the registry and `policies.default_action` is `"deny"`, the Adapter MUST deny the invocation. If `default_action` is `"allow"`, the Adapter MUST allow it (and SHOULD emit a `tool.allow` event with `policy_match: "default_allow"`).

### 2.3 Execution

The Adapter MUST execute allowed commands as a subprocess, inheriting the caller's environment unless `env` overrides are specified in the tool definition. The Adapter MUST enforce the `timeout` field if present, terminating the subprocess and returning exit code 1 if the timeout is exceeded.

### 2.4 Instrumentation

The Adapter MUST emit trace events to the configured `instrumentation.event_sink`. See `spec/04-instrumentation.md` for event format and field requirements.

## 3. Contract

The protocol establishes a contract between three parties:

| Party | Obligation |
|---|---|
| Agent | Submits invocations to the Adapter; does not bypass it |
| Adapter | Enforces policy; instruments every invocation; returns deterministic exit codes |
| Toolset Registry | Accurately reflects intended policy; is valid per `toolsets.schema.json` |

A system claiming OATP conformance MUST NOT allow the Agent to bypass the Adapter and invoke tools directly. The Adapter is the single choke point.

## 4. Failure modes

| Failure | Exit code | Trace event |
|---|---|---|
| Command rejected by policy | 2 | `tool.deny` |
| Command requires approval, approval not granted | 2 | `tool.deny` |
| Exec failure (subprocess error) | 1 | `tool.exec.end` with non-zero exit |
| Timeout exceeded | 1 | `tool.exec.end` with `timed_out: true` |
| Registry schema invalid | 3 | `toolset.schema_error` |
| Registry not found | 4 | `toolset.not_found` |

The Adapter MUST NOT return exit code 0 for any denied or errored invocation.

## 5. Redaction

If `instrumentation.redact_secrets` is `true`, the Adapter MUST apply redaction to stdout and stderr before returning them to the Agent. Redaction patterns are defined in the `redaction` block of the registry. The Adapter MUST substitute matched content with `[REDACTED]` and MUST increment the `redactions` counter in the emitted trace event. The Adapter MUST NOT log or emit the unredacted content.

## 6. Relation to A2A

OATP is complementary to Agent-to-Agent (A2A) protocols. A2A standardizes how agents discover and message each other; it deliberately leaves agent-internal tool discipline unspecified. OATP fills that gap.

An A2A-compliant agent SHOULD advertise its OATP registry at `/.well-known/toolset.json` so peer agents can reason about which disciplines, phases, and verification modes the agent enforces internally before delegating work to it. This is the same well-known discovery pattern A2A uses for agent cards, applied to tool contracts rather than messaging capabilities.

OATP does not require A2A, and A2A does not require OATP. When used together, OATP governs intra-agent tool discipline; A2A governs inter-agent communication. The boundary is the agent's own tool invocation layer.
