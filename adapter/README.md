# oatp — OATP Reference Adapter

Reference implementation of the Open Toolset Protocol adapter in Rust.

**Status: stub (v0.0.1).** Parses commands and prints what it would do. Full enforcement and instrumentation ships post-RFC.

## Usage

```bash
oatp exec -- <cmd> [args...]
```

Examples:

```bash
# Validate and run rg
oatp exec -- rg "pattern" src/

# Validate and run git log (read-only)
oatp exec -- git log --oneline -20

# Attempt a banned command (will be rejected at L2+)
oatp exec -- npm install -g typescript
```

## Registry loading

The adapter looks for `toolsets.json` in this order:

1. `$OATP_TOOLSET` — explicit path via environment variable
2. `./toolsets.json` — current working directory
3. `~/.config/oatp/toolsets.json` — user-global fallback

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
