//! Command-line entrypoints for launcher workflows.
//!
//! This binary currently owns a real command surface and fake handlers. The
//! point of this stage is to make the Launcher vocabulary concrete enough to
//! discuss and test manually, without pretending the backend behavior exists.

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();
    dispatch::run(&cli);
}

mod cli {
    use clap::{Parser, Subcommand, ValueEnum};

    /// Parsed top-level Launcher invocation.
    #[derive(Parser)]
    #[command(name = "vapor-launcher")]
    #[command(version, about = "Launcher workflows for Vapor content.")]
    pub(crate) struct Cli {
        /// Show operation planning, diagnostics, historical context, and live detail.
        #[arg(long, global = true)]
        verbose: bool,

        /// Accept non-destructive prompts such as dependency-closure installation.
        #[arg(long, global = true)]
        yes: bool,

        /// Permit destructive or risk-bearing operations when that command supports it.
        #[arg(long, global = true)]
        force: bool,

        /// Reject local pack mutations that would leave the pack invalid.
        #[arg(long, global = true)]
        strict: bool,

        /// Keep old unused installed versions after update, lock, repair, or cleanup.
        #[arg(long, global = true)]
        keep_unused_versions: bool,

        #[command(subcommand)]
        command: Command,
    }

    impl Cli {
        pub(crate) fn command(&self) -> &Command {
            &self.command
        }

        pub(crate) fn globals(&self) -> GlobalOptions {
            GlobalOptions {
                verbose: self.verbose,
                yes: self.yes,
                force: self.force,
                strict: self.strict,
                keep_unused_versions: self.keep_unused_versions,
            }
        }
    }

    /// Global execution options after CLI parsing.
    #[derive(Clone, Copy, Debug)]
    pub(crate) struct GlobalOptions {
        pub(crate) verbose: bool,
        pub(crate) yes: bool,
        pub(crate) force: bool,
        pub(crate) strict: bool,
        pub(crate) keep_unused_versions: bool,
    }

    /// Root Launcher workflows.
    #[derive(Subcommand)]
    pub(crate) enum Command {
        /// Print Launcher version/build identity.
        Version,
        /// Summarize Launcher health, selected packagepack, and local state.
        Status,
        /// Inspect, plan, or apply repairs to Launcher-managed state.
        Repair {
            #[command(subcommand)]
            command: RepairCommand,
        },
        /// Work with packagepacks, including root selection and packagepack locking.
        Packagepack {
            #[command(subcommand)]
            command: PackagepackCommand,
        },
        /// Work with enginepacks as installed content or local mutable packs.
        Enginepack {
            #[command(subcommand)]
            command: PackCommand,
        },
        /// Work with gamepacks as installed content or local mutable packs.
        Gamepack {
            #[command(subcommand)]
            command: PackCommand,
        },
        /// Work with modpacks as installed content or local mutable packs.
        Modpack {
            #[command(subcommand)]
            command: PackCommand,
        },
        /// Work with installed engine content.
        Engine {
            #[command(subcommand)]
            command: LeafCommand,
        },
        /// Work with installed game content.
        Game {
            #[command(subcommand)]
            command: LeafCommand,
        },
        /// Work with installed engine mod content.
        #[command(name = "engine_mod")]
        EngineMod {
            #[command(subcommand)]
            command: LeafCommand,
        },
        /// Work with installed game mod content.
        #[command(name = "game_mod")]
        GameMod {
            #[command(subcommand)]
            command: LeafCommand,
        },
        /// Work with installed extension mod content.
        #[command(name = "extension_mod")]
        ExtensionMod {
            #[command(subcommand)]
            command: LeafCommand,
        },
        /// Lock the currently selected packagepack.
        Lock,
        /// Launch the currently selected packagepack.
        Launch,
    }

    /// Repair commands intentionally split planning from mutation.
    #[derive(Subcommand)]
    pub(crate) enum RepairCommand {
        /// Inspect repairable areas without proposing mutations.
        Status,
        /// Produce a repair plan without applying it.
        Plan { target: RepairTarget },
        /// Apply repairs for a target after the plan is accepted.
        Apply { target: RepairTarget },
    }

    /// Packagepack commands include both root-level user selection and local mutable pack editing.
    #[derive(Subcommand)]
    pub(crate) enum PackagepackCommand {
        /// List packagepacks from one content source.
        List { source: ContentSource },
        /// Show local status for one packagepack.
        Status { packagepack_id: String },
        /// Compute or display a deterministic packagepack fingerprint.
        Fingerprint { packagepack_id: String },
        /// Inspect packagepack metadata, installation state, and known graph state.
        Inspect { packagepack_id: String },
        /// Validate packagepack metadata and graph invariants.
        Validate { packagepack_id: String },
        /// Lock an explicit packagepack, selected or not.
        Lock { packagepack_id: String },
        /// Install a packagepack and its required dependency closure.
        Install { packagepack_id: String },
        /// Remove an installed packagepack when dependency safety permits it.
        Uninstall { packagepack_id: String },
        /// Update an installed packagepack, which is the normal user update boundary.
        Update { packagepack_id: String },
        /// Create a blank local mutable packagepack.
        New { packagepack_id: String },
        /// Create a local mutable packagepack from an existing source packagepack.
        Fork {
            source_pack_id: String,
            new_pack_id: String,
        },
        /// Select the root packagepack, or select child content inside a local mutable packagepack.
        Select {
            packagepack_id: String,
            child_content_type: Option<ContentType>,
            child_content_id: Option<String>,
        },
        /// Add child content to a local mutable packagepack.
        Add {
            packagepack_id: String,
            child_content_type: ContentType,
            child_content_id: String,
        },
        /// Remove child content from a local mutable packagepack.
        Remove {
            packagepack_id: String,
            child_content_type: ContentType,
            child_content_id: String,
        },
        /// Keep child content present but inactive inside a local mutable packagepack.
        Unselect {
            packagepack_id: String,
            child_content_type: ContentType,
            child_content_id: String,
        },
    }

    /// Commands shared by non-root pack types.
    #[derive(Subcommand)]
    pub(crate) enum PackCommand {
        /// List content from one source.
        List { source: ContentSource },
        /// Show local status for one pack.
        Status { pack_id: String },
        /// Compute or display a deterministic pack fingerprint.
        Fingerprint { pack_id: String },
        /// Inspect pack metadata, installation state, and known graph state.
        Inspect { pack_id: String },
        /// Validate pack metadata and graph invariants.
        Validate { pack_id: String },
        /// Install a pack and its required dependency closure.
        Install { pack_id: String },
        /// Remove an installed pack when dependency safety permits it.
        Uninstall { pack_id: String },
        /// Update an installed pack. This is usually an author/modpack workflow, not a normal player workflow.
        Update { pack_id: String },
        /// Create a blank local mutable pack.
        New { pack_id: String },
        /// Create a local mutable pack from an existing source pack.
        Fork {
            source_pack_id: String,
            new_pack_id: String,
        },
        /// Add child content to a local mutable pack.
        Add {
            pack_id: String,
            child_content_type: ContentType,
            child_content_id: String,
        },
        /// Remove child content from a local mutable pack.
        Remove {
            pack_id: String,
            child_content_type: ContentType,
            child_content_id: String,
        },
        /// Select active child content inside a local mutable pack.
        Select {
            pack_id: String,
            child_content_type: ContentType,
            child_content_id: String,
        },
        /// Keep child content present but inactive inside a local mutable pack.
        Unselect {
            pack_id: String,
            child_content_type: ContentType,
            child_content_id: String,
        },
    }

    /// Commands shared by leaf content types that cannot contain child content.
    #[derive(Subcommand)]
    pub(crate) enum LeafCommand {
        /// List content from one source.
        List { source: ContentSource },
        /// Show local status for one content identity.
        Status { content_id: String },
        /// Compute or display a deterministic content fingerprint.
        Fingerprint { content_id: String },
        /// Inspect metadata and installation state.
        Inspect { content_id: String },
        /// Validate metadata and compatibility requirements.
        Validate { content_id: String },
        /// Install content and its required dependency closure.
        Install { content_id: String },
        /// Remove installed content when dependency safety permits it.
        Uninstall { content_id: String },
        /// Update installed content. This may fork packagepack state outside normal player workflows.
        Update { content_id: String },
    }

    /// Places the Launcher can list or discover content from.
    #[derive(Clone, Copy, Debug, ValueEnum)]
    pub(crate) enum ContentSource {
        /// Already discovered through configured sources.
        Discovered,
        /// Local filesystem or local user library.
        Local,
        /// Git-backed sources.
        Git,
        /// Steam Workshop-backed sources.
        Workshop,
        /// Every configured source.
        All,
    }

    /// Content kind used when a pack command targets a child.
    #[derive(Clone, Copy, Debug, ValueEnum)]
    pub(crate) enum ContentType {
        Packagepack,
        Enginepack,
        Gamepack,
        Modpack,
        Engine,
        Game,
        #[value(name = "engine_mod")]
        EngineMod,
        #[value(name = "game_mod")]
        GameMod,
        #[value(name = "extension_mod")]
        ExtensionMod,
    }

    /// Repair target used by `repair plan` and `repair apply`.
    #[derive(Clone, Copy, Debug, ValueEnum)]
    pub(crate) enum RepairTarget {
        /// Launcher-owned local state and configuration.
        #[value(name = "core_state")]
        CoreState,
        /// Pinned Rust/Cargo/toolchain state needed by Vapor workflows.
        Toolchain,
        /// Steam identity, ownership, Workshop, and Steam-facing cache state.
        Steam,
        /// Source indexes and discovered-content catalogs.
        #[value(name = "content_catalog")]
        ContentCatalog,
        /// Installed artifacts and local content records.
        #[value(name = "content_library")]
        ContentLibrary,
        /// The selected packagepack composition, lock, and derived state.
        #[value(name = "active_composition")]
        ActiveComposition,
        /// Every repair target in dependency order.
        All,
    }
}

mod dispatch {
    use super::cli::*;

    /// Broad state surface a future implementation may read or mutate.
    #[derive(Debug)]
    enum StateSurface {
        ReadOnly,
        RepairPlan,
        RepairApply,
        InstalledLibrary,
        LocalMutablePack,
        ActiveComposition,
        RuntimeLaunch,
    }

    /// Local command contract used by the placeholder dispatcher.
    struct CommandSpec {
        action: String,
        summary: &'static str,
        surface: StateSurface,
        preconditions: &'static [&'static str],
        future_effects: &'static [&'static str],
    }

    pub(crate) fn run(cli: &Cli) {
        let globals = cli.globals();
        match describe_command(cli.command()) {
            Ok(spec) => print_stub(globals, spec),
            Err(message) => eprintln!("{message}"),
        }
    }

    fn describe_command(command: &Command) -> Result<CommandSpec, &'static str> {
        match command {
            Command::Version => Ok(spec(
                "launcher version",
                "Print Launcher version and build identity.",
                StateSurface::ReadOnly,
                &[],
                &["display version metadata"],
            )),
            Command::Status => Ok(spec(
                "launcher status",
                "Summarize Launcher health, selected packagepack, and local state.",
                StateSurface::ReadOnly,
                &[],
                &["display local state and repair hints"],
            )),
            Command::Repair { command } => Ok(describe_repair(command)),
            Command::Packagepack { command } => describe_packagepack(command),
            Command::Enginepack { command } => Ok(describe_pack("enginepack", command)),
            Command::Gamepack { command } => Ok(describe_pack("gamepack", command)),
            Command::Modpack { command } => Ok(describe_pack("modpack", command)),
            Command::Engine { command } => Ok(describe_leaf("engine", command)),
            Command::Game { command } => Ok(describe_leaf("game", command)),
            Command::EngineMod { command } => Ok(describe_leaf("engine_mod", command)),
            Command::GameMod { command } => Ok(describe_leaf("game_mod", command)),
            Command::ExtensionMod { command } => Ok(describe_leaf("extension_mod", command)),
            Command::Lock => Ok(spec(
                "launcher lock selected packagepack",
                "Write a persistent lock artifact for the selected packagepack.",
                StateSurface::ActiveComposition,
                &["one packagepack is selected", "the selected packagepack can be resolved"],
                &["resolve the selected packagepack", "write or update its lock artifact"],
            )),
            Command::Launch => Ok(spec(
                "launcher launch selected packagepack",
                "Launch the selected packagepack.",
                StateSurface::RuntimeLaunch,
                &["one packagepack is selected", "the selected packagepack has a usable locked graph"],
                &["prepare runtime inputs", "start the selected packagepack experience"],
            )),
        }
    }

    fn describe_repair(command: &RepairCommand) -> CommandSpec {
        match command {
            RepairCommand::Status => spec(
                "launcher repair status",
                "Inspect repairable Launcher targets without proposing mutation.",
                StateSurface::ReadOnly,
                &[],
                &["display repair target health"],
            ),
            RepairCommand::Plan { .. } => spec(
                "launcher repair plan",
                "Prepare a repair plan without applying it.",
                StateSurface::RepairPlan,
                &["repair target is known"],
                &["compute proposed repair operations"],
            ),
            RepairCommand::Apply { .. } => spec(
                "launcher repair apply",
                "Apply repairs for a Launcher target.",
                StateSurface::RepairApply,
                &["repair target is known", "planned mutations are acceptable"],
                &["repair selected Launcher-managed state"],
            ),
        }
    }

    fn describe_packagepack(command: &PackagepackCommand) -> Result<CommandSpec, &'static str> {
        Ok(match command {
            PackagepackCommand::List { .. } => read_spec("launcher packagepack list", "List packagepacks from a content source."),
            PackagepackCommand::Status { .. } => read_spec("launcher packagepack status", "Show local packagepack status."),
            PackagepackCommand::Fingerprint { .. } => read_spec("launcher packagepack fingerprint", "Compute or display a deterministic packagepack fingerprint."),
            PackagepackCommand::Inspect { .. } => read_spec("launcher packagepack inspect", "Inspect packagepack metadata and local state."),
            PackagepackCommand::Validate { .. } => read_spec("launcher packagepack validate", "Validate packagepack metadata and resolved graph invariants."),
            PackagepackCommand::Lock { .. } => spec(
                "launcher packagepack lock",
                "Write a persistent lock artifact for an explicit packagepack.",
                StateSurface::ActiveComposition,
                &["packagepack is locally available", "packagepack can be resolved"],
                &["resolve explicit packagepack", "write or update its lock artifact"],
            ),
            PackagepackCommand::Install { .. } => installed_spec("launcher packagepack install", "Install a packagepack and its required dependency closure."),
            PackagepackCommand::Uninstall { .. } => installed_spec("launcher packagepack uninstall", "Remove an installed packagepack when dependency safety permits it."),
            PackagepackCommand::Update { .. } => installed_spec("launcher packagepack update", "Update an installed packagepack, the normal user update boundary."),
            PackagepackCommand::New { .. } => local_pack_spec("launcher packagepack new", "Create a blank local mutable packagepack."),
            PackagepackCommand::Fork { .. } => local_pack_spec("launcher packagepack fork", "Create a local mutable packagepack from an existing source packagepack."),
            PackagepackCommand::Select {
                child_content_type,
                child_content_id,
                ..
            } => match (child_content_type, child_content_id) {
                (None, None) => spec(
                    "launcher packagepack select",
                    "Select the active root packagepack used by launcher lock and launch.",
                    StateSurface::ActiveComposition,
                    &["packagepack is installed or otherwise locally available"],
                    &["record selected root packagepack"],
                ),
                (Some(_), Some(_)) => composition_spec("launcher packagepack select", "Select active child content inside a local mutable packagepack."),
                _ => return Err("packagepack select needs either only <PACKAGEPACK_ID> or both child arguments"),
            },
            PackagepackCommand::Add { .. } => composition_spec("launcher packagepack add", "Add child content to a local mutable packagepack."),
            PackagepackCommand::Remove { .. } => composition_spec("launcher packagepack remove", "Remove child content from a local mutable packagepack."),
            PackagepackCommand::Unselect { .. } => composition_spec("launcher packagepack unselect", "Keep child content present but inactive inside a local mutable packagepack."),
        })
    }

    fn describe_pack(content_type: &str, command: &PackCommand) -> CommandSpec {
        let action = match command {
            PackCommand::List { .. } => return read_spec(format!("launcher {content_type} list"), "List packs from a content source."),
            PackCommand::Status { .. } => return read_spec(format!("launcher {content_type} status"), "Show local pack status."),
            PackCommand::Fingerprint { .. } => return read_spec(format!("launcher {content_type} fingerprint"), "Compute or display a deterministic pack fingerprint."),
            PackCommand::Inspect { .. } => return read_spec(format!("launcher {content_type} inspect"), "Inspect pack metadata and local state."),
            PackCommand::Validate { .. } => return read_spec(format!("launcher {content_type} validate"), "Validate pack metadata and graph invariants."),
            PackCommand::Install { .. } => return installed_spec(format!("launcher {content_type} install"), "Install a pack and its dependency closure."),
            PackCommand::Uninstall { .. } => return installed_spec(format!("launcher {content_type} uninstall"), "Remove an installed pack when dependency safety permits it."),
            PackCommand::Update { .. } => return installed_spec(format!("launcher {content_type} update"), "Update installed pack content outside the normal packagepack update boundary."),
            PackCommand::New { .. } => return local_pack_spec(format!("launcher {content_type} new"), "Create a blank local mutable pack."),
            PackCommand::Fork { .. } => return local_pack_spec(format!("launcher {content_type} fork"), "Create a local mutable pack from an existing source pack."),
            PackCommand::Add { .. } => "add",
            PackCommand::Remove { .. } => "remove",
            PackCommand::Select { .. } => "select",
            PackCommand::Unselect { .. } => "unselect",
        };
        composition_spec(
            format!("launcher {content_type} {action}"),
            "Mutate child membership or active child selection inside a local mutable pack.",
        )
    }

    fn describe_leaf(content_type: &str, command: &LeafCommand) -> CommandSpec {
        match command {
            LeafCommand::List { .. } => read_spec(format!("launcher {content_type} list"), "List content from a content source."),
            LeafCommand::Status { .. } => read_spec(format!("launcher {content_type} status"), "Show local content status."),
            LeafCommand::Fingerprint { .. } => read_spec(format!("launcher {content_type} fingerprint"), "Compute or display a deterministic content fingerprint."),
            LeafCommand::Inspect { .. } => read_spec(format!("launcher {content_type} inspect"), "Inspect content metadata and local state."),
            LeafCommand::Validate { .. } => read_spec(format!("launcher {content_type} validate"), "Validate metadata and compatibility requirements."),
            LeafCommand::Install { .. } => installed_spec(format!("launcher {content_type} install"), "Install content and its dependency closure."),
            LeafCommand::Uninstall { .. } => installed_spec(format!("launcher {content_type} uninstall"), "Remove installed content when dependency safety permits it."),
            LeafCommand::Update { .. } => installed_spec(format!("launcher {content_type} update"), "Update installed content outside the normal packagepack update boundary."),
        }
    }

    fn read_spec(action: impl Into<String>, summary: &'static str) -> CommandSpec {
        spec(action, summary, StateSurface::ReadOnly, &[], &["display requested information"])
    }

    fn installed_spec(action: impl Into<String>, summary: &'static str) -> CommandSpec {
        spec(
            action,
            summary,
            StateSurface::InstalledLibrary,
            &["content identity is known", "required source is available"],
            &["update installed content records and artifacts when implemented"],
        )
    }

    fn local_pack_spec(action: impl Into<String>, summary: &'static str) -> CommandSpec {
        spec(
            action,
            summary,
            StateSurface::LocalMutablePack,
            &["target pack identity is available for local mutation"],
            &["create or update local mutable pack state when implemented"],
        )
    }

    fn composition_spec(action: impl Into<String>, summary: &'static str) -> CommandSpec {
        spec(
            action,
            summary,
            StateSurface::LocalMutablePack,
            &["target pack is a local mutable pack", "child type is allowed by the parent pack type"],
            &["update local mutable pack membership or active selection when implemented"],
        )
    }

    fn spec(
        action: impl Into<String>,
        summary: &'static str,
        surface: StateSurface,
        preconditions: &'static [&'static str],
        future_effects: &'static [&'static str],
    ) -> CommandSpec {
        CommandSpec {
            action: action.into(),
            summary,
            surface,
            preconditions,
            future_effects,
        }
    }

    fn print_stub(globals: GlobalOptions, spec: CommandSpec) {
        println!(
            "Doing {}! Trust me, I am definitely doing it and not just a placeholder message.",
            spec.action
        );

        if globals.verbose {
            println!("summary: {}", spec.summary);
            println!("state_surface: {:?}", spec.surface);
            print_lines("preconditions", spec.preconditions);
            print_lines("future_effects", spec.future_effects);
            println!("yes: {}", globals.yes);
            println!("force: {}", globals.force);
            println!("strict: {}", globals.strict);
            println!("keep_unused_versions: {}", globals.keep_unused_versions);
        }
    }

    fn print_lines(label: &str, lines: &[&str]) {
        println!("{label}:");
        if lines.is_empty() {
            println!("  none");
        } else {
            for line in lines {
                println!("  {line}");
            }
        }
    }
}
