use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "oatp",
    version = "0.0.1",
    about = "Open Agent Toolset Protocol reference adapter"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate and execute a command against the active toolset
    Exec {
        /// The command and arguments to execute (everything after --)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        cmd: Vec<String>,
    },

    /// List resolved tools from the active toolset
    Ls {
        /// Filter by phase (reconnaissance, surgery, instrumentation)
        #[arg(long)]
        phase: Option<String>,

        /// Filter by category
        #[arg(long)]
        category: Option<String>,

        /// Show only required tools
        #[arg(long)]
        required: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Registry introspection subcommands
    Registry {
        #[command(subcommand)]
        action: RegistryCommands,
    },

    /// Validate a toolsets.json file against the OATP schema
    Check {
        /// Path to toolsets.json to validate
        path: String,
    },

    /// Set the active phase
    Phase {
        /// Phase to activate (reconnaissance, surgery, instrumentation)
        #[arg(long = "set")]
        set: String,
    },
}

#[derive(Subcommand)]
enum RegistryCommands {
    /// List all available registries in resolution order
    Ls {
        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Include tool counts per registry
        #[arg(long)]
        show_tools: bool,

        /// Perform discovery and mark the winning registry
        #[arg(long)]
        resolve: bool,
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

            // TODO: load toolset registry (multi-source resolution order)
            //   1. $OATP_TOOLSET env var (absolute path or URL)
            //   2. ./toolsets.json (project local)
            //   3. ./.oatp/toolsets.json (project dotdir)
            //   4. $XDG_CONFIG_HOME/oatp/toolsets.json or ~/.config/oatp/toolsets.json
            //   5. /etc/oatp/toolsets.json (system config)
            //   6. https://<host>/.well-known/toolset.json (if $OATP_REMOTE is set)
            //   7. oatp:builtin/safe-defaults (embedded fallback)
            //   Emit: discovery.resolved trace event with source, path_or_url, version

            // TODO: resolve $ref entries in toolsets array (recursive, cycle-detected)
            //   Emit: discovery.compose per resolved nested registry

            // TODO: validate toolsets.json against schemas/toolsets.schema.json
            //   If invalid: emit toolset.schema_error trace event, exit 3

            // TODO: capability negotiation
            //   Emit adapter capability offer, accept agent claim, compute intersection

            // TODO: check active_phase (default: reconnaissance on session start) and priority-based category substitution and resolve priority-ranked category substitutions

            // TODO: validate invocation against active toolset
            //   1. policies.deny glob match -> reject exit 2, denylist_match
            //   2. policies.forbidden_args literal argv match -> reject exit 2, forbidden_args_match
            //   3. policies.allow glob match -> continue
            //   4. policies.default_action -> allow or deny
            //   5. tool.phase vs active_phase -> reject exit 2, phase_gate_violation
            //   6. tool.requires_prior -> scan phase trace -> reject exit 2, precondition_unsatisfied
            //   7. gps_protocol enforcement if capabilities.gps_protocol == true
            //   8. tool.requires_approval -> emit tool.approval_requested, await signal

            // TODO: emit tool.exec.start trace event

            // TODO: execute command as subprocess
            //   Inherit env, enforce timeout, stream stdout/stderr with optional redaction

            // TODO: validate instrumented_return if required == true
            //   Parse stdout as JSON, validate against schema_ref
            //   On failure: emit tool.attestation_failed, exit 1

            // TODO: emit tool.exec.end trace event (exit_code, duration_ms, timed_out, redactions)

            // TODO: exit with subprocess exit code (or 1 on exec failure / timeout)

            std::process::exit(0);
        }

        Commands::Ls { phase, category, required, json } => {
            // TODO: load and resolve registry (same as Exec)
            // TODO: filter tools by --phase, --category, --required flags
            // TODO: output table or JSON

            eprintln!(
                "OATP adapter v0.0.1 — stub. Would list resolved tools (phase={:?}, category={:?}, required={}, json={}).",
                phase, category, required, json
            );
            println!("PHASE           CATEGORY              TOOL                    VERIFICATION    REQUIRED");
            println!("(stub — no registry loaded)");

            std::process::exit(0);
        }

        Commands::Registry { action } => match action {
            RegistryCommands::Ls { json, show_tools, resolve } => {
                // TODO: enumerate all registry sources in resolution order
                // TODO: check each source for existence / validity
                // TODO: mark which would be active (first hit wins)

                eprintln!(
                    "OATP adapter v0.0.1 — stub. Would list registries (json={}, show_tools={}, resolve={}).",
                    json, show_tools, resolve
                );
                println!("ORDER  SOURCE                                 STATUS    TOOLS");
                println!("1      $OATP_TOOLSET                          (check env)");
                println!("2      ./toolsets.json                        (check cwd)");
                println!("3      ./.oatp/toolsets.json                  (check cwd/.oatp)");
                println!("4      ~/.config/oatp/toolsets.json           (check user config)");
                println!("5      /etc/oatp/toolsets.json                (check system config)");
                println!("6      https://<host>/.well-known/toolset.json (only if $OATP_REMOTE set)");
                println!("7      oatp:builtin/safe-defaults             available");

                std::process::exit(0);
            }
        },

        Commands::Check { path } => {
            eprintln!(
                "OATP adapter v0.0.1 — stub. Would validate {} against OATP schema.",
                path
            );
            // TODO: load path, parse JSON, validate against embedded schema
            std::process::exit(0);
        }

        Commands::Phase { set } => {
            eprintln!(
                "OATP adapter v0.0.1 — stub. Would set active phase to '{}'.",
                set
            );
            // TODO: validate phase value (reconnaissance | surgery | instrumentation)
            // TODO: check required-tool satisfaction for exiting phase
            // TODO: emit phase.transition trace event
            // TODO: persist active_phase in session state
            std::process::exit(0);
        }
    }
}
