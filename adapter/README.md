# oatp — OATP Reference Adapter

Reference implementation of the Open Toolset Protocol adapter in Rust.

**Status: stub (v0.0.1).** Parses commands and prints what it would do. Full enforcement and instrumentation ships post-RFC.

## Usage

```bash
oatp exec -- <cmd> [args...]
oatp ls [--phase <phase>] [--category <category>] [--required] [--json]
oatp registry ls [--json] [--show-tools] [--resolve]
oatp check <path>
oatp phase --set <phase>
```

### `oatp exec`

Validate and execute a command against the active toolset.

```bash
# Validate and run rg
oatp exec -- rg "pattern" src/

# Validate and run git log (read-only)
oatp exec -- git log --oneline -20

# Attempt a banned command (rejected at L2+)
oatp exec -- npm install -g typescript
```

### `oatp ls`

List resolved tools from the active toolset. Output is a table (default) or JSON.

```
PHASE           CATEGORY              TOOL                    VERIFICATION    REQUIRED
reconnaissance  index-search          rg                      deterministic   false
reconnaissance  syntax-match-rewrite  sg                      deterministic   true
surgery         structural-edit       edit                    deterministic   false
instrumentation merge-diff            difftastic              deterministic   true
```

Flags:

| Flag | Description |
|---|---|
| `--phase <phase>` | Filter by phase (`reconnaissance`, `surgery`, `instrumentation`) |
| `--category <category>` | Filter by category (e.g. `index-search`) |
| `--required` | Show only tools with `required: true` |
| `--json` | Output as JSON array |

### `oatp registry ls`

List all registry sources in resolution order, showing which is active (first hit wins).

```
ORDER  SOURCE                                  STATUS    TOOLS
1      $OATP_TOOLSET                           (not set)
2      ./toolsets.json                         found     7
3      ./.oatp/toolsets.json                   (not found)
4      ~/.config/oatp/toolsets.json            (not found)
5      /etc/oatp/toolsets.json                 (not found)
6      https://<host>/.well-known/toolset.json  (no $OATP_REMOTE)
7      oatp:builtin/safe-defaults              available  5
```

Flags:

| Flag | Description |
|---|---|
| `--json` | Output as JSON array |
| `--show-tools` | Include tool counts per registry |
| `--resolve` | Perform discovery and mark the winning registry |

### `oatp check`

Validate a `toolsets.json` file against the OATP schema.

```bash
oatp check ./toolsets.json
oatp check ./examples/claude-code.toolsets.json
```

### `oatp phase`

Set the active phase for the current session.

```bash
oatp phase --set reconnaissance
oatp phase --set surgery
oatp phase --set instrumentation
```

Phase transitions are gated: all tools with `required: true` in the exiting phase must have been invoked before the transition is allowed.

### Built-in registries

The adapter embeds two canonical registries accessible via the `oatp:builtin/<name>` URI scheme:

| URI | Description |
|---|---|
| `oatp:builtin/safe-defaults` | 5 read-only reconnaissance tools (ls, cat, git-status, git-log, git-diff). Default fallback. |
| `oatp:builtin/empty` | No tools, default deny. Used for policy enforcement testing. |

Reference these from any `toolsets.json` via `$ref`:

```json
{ "$ref": "oatp:builtin/safe-defaults" }
```

## Registry loading

The adapter looks for `toolsets.json` in this order:

1. `$OATP_TOOLSET` — explicit path or URL via environment variable
2. `./toolsets.json` — project root
3. `./.oatp/toolsets.json` — project dotdir
4. `$XDG_CONFIG_HOME/oatp/toolsets.json` or `~/.config/oatp/toolsets.json` — user config
5. `/etc/oatp/toolsets.json` — system config
6. `https://<host>/.well-known/toolset.json` — remote discovery (only if `$OATP_REMOTE` is set)
7. `oatp:builtin/safe-defaults` — embedded fallback (always available)

## Environment variables

| Variable | Description |
|---|---|
| `OATP_TOOLSET` | Path to the active `toolsets.json`. Highest precedence. |
| `OATP_TRACE_SINK` | Override `instrumentation.event_sink` at runtime. |
| `OATP_LOG_LEVEL` | Adapter verbosity: `error`, `warn`, `info`, `debug`. Default: `warn`. |

## Exit codes

| Code | Meaning |
|---|---|
| 0 | Command executed successfully |
| 1 | Exec failure or timeout |
| 2 | Policy rejection |
| 3 | Registry schema error |
| 4 | Registry not found |

## Building

```bash
cargo build --release
```

Requires Rust 1.83+ (see `../mise.toml`).

## Conformance

This adapter targets OATP-L2/0.1 when complete. Current stub passes no conformance vectors.
