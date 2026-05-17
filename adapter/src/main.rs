use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "oatp", version = "0.0.1", about = "Open Toolset Protocol reference adapter")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate and execute a command against the active toolset
    Exec {
        /// The command and arguments to execute
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        cmd: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Exec { cmd } => {
            let cmd_str = cmd.join(" ");
            eprintln!(
                "OATP adapter v0.0.1 — stub. Would load toolsets.json from $OATP_TOOLSET and validate: {}",
                cmd_str
            );

            // TODO: load toolset registry
            //   1. Check $OATP_TOOLSET env var
            //   2. Fall back to ./toolsets.json
            //   3. Fall back to ~/.config/oatp/toolsets.json
            //   4. If not found: emit toolset.not_found trace event, exit 4

            // TODO: validate toolsets.json against schemas/toolsets.schema.json
            //   If invalid: emit toolset.schema_error trace event, exit 3

            // TODO: validate invocation against active toolset
            //   1. Check policies.banned_patterns against full cmd string
            //   2. Match cmd against tools[].command
            //   3. Check args_pattern.deny / args_pattern.allow
            //   4. Check cwd_constraint
            //   5. Check path_denylist / path_allowlist
            //   6. If denied: emit tool.deny trace event, exit 2

            // TODO: handle requires_approval: true
            //   Emit tool.approval_requested, await signal on $OATP_APPROVAL_SOCKET

            // TODO: emit tool.exec.start trace event

            // TODO: execute command as subprocess
            //   Inherit env, apply env overrides from tool definition
            //   Enforce timeout (terminate subprocess if exceeded)
            //   Stream stdout/stderr with optional redaction

            // TODO: emit tool.exec.end trace event (exit_code, duration_ms, timed_out, redactions)

            // TODO: exit with subprocess exit code (or 1 on exec failure / timeout)

            std::process::exit(0);
        }
    }
}
