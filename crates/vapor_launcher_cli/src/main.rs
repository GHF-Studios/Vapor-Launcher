//! Command-line entrypoints for launcher workflows.

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "vapor-launcher")]
#[command(version, about = "Launcher workflows for Vapor content.")]
struct Cli {
    #[arg(long, global = true)]
    verbose: bool,

    #[arg(long, global = true)]
    yes: bool,

    #[arg(long, global = true)]
    force: bool,

    #[arg(long, global = true)]
    strict: bool,

    #[arg(long, global = true)]
    keep_unused_versions: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Version,
    Status,
    Repair {
        #[command(subcommand)]
        command: RepairCommand,
    },
    Packagepack {
        #[command(subcommand)]
        command: PackagepackCommand,
    },
    Enginepack {
        #[command(subcommand)]
        command: PackCommand,
    },
    Gamepack {
        #[command(subcommand)]
        command: PackCommand,
    },
    Modpack {
        #[command(subcommand)]
        command: PackCommand,
    },
    Engine {
        #[command(subcommand)]
        command: LeafCommand,
    },
    Game {
        #[command(subcommand)]
        command: LeafCommand,
    },
    #[command(name = "engine_mod")]
    EngineMod {
        #[command(subcommand)]
        command: LeafCommand,
    },
    #[command(name = "game_mod")]
    GameMod {
        #[command(subcommand)]
        command: LeafCommand,
    },
    #[command(name = "extension_mod")]
    ExtensionMod {
        #[command(subcommand)]
        command: LeafCommand,
    },
    Lock,
    Launch,
}

#[derive(Subcommand)]
enum RepairCommand {
    Status,
    Plan { target: RepairTarget },
    Apply { target: RepairTarget },
}

#[derive(Subcommand)]
enum PackagepackCommand {
    List { source: ContentSource },
    Status { packagepack_id: String },
    Fingerprint { packagepack_id: String },
    Inspect { packagepack_id: String },
    Validate { packagepack_id: String },
    Lock { packagepack_id: String },
    Install { packagepack_id: String },
    Uninstall { packagepack_id: String },
    Update { packagepack_id: String },
    New { packagepack_id: String },
    Fork {
        source_pack_id: String,
        new_pack_id: String,
    },
    Select {
        packagepack_id: String,
        child_content_type: Option<ContentType>,
        child_content_id: Option<String>,
    },
    Add {
        packagepack_id: String,
        child_content_type: ContentType,
        child_content_id: String,
    },
    Remove {
        packagepack_id: String,
        child_content_type: ContentType,
        child_content_id: String,
    },
    Unselect {
        packagepack_id: String,
        child_content_type: ContentType,
        child_content_id: String,
    },
}

#[derive(Subcommand)]
enum PackCommand {
    List { source: ContentSource },
    Status { pack_id: String },
    Fingerprint { pack_id: String },
    Inspect { pack_id: String },
    Validate { pack_id: String },
    Install { pack_id: String },
    Uninstall { pack_id: String },
    Update { pack_id: String },
    New { pack_id: String },
    Fork {
        source_pack_id: String,
        new_pack_id: String,
    },
    Add {
        pack_id: String,
        child_content_type: ContentType,
        child_content_id: String,
    },
    Remove {
        pack_id: String,
        child_content_type: ContentType,
        child_content_id: String,
    },
    Select {
        pack_id: String,
        child_content_type: ContentType,
        child_content_id: String,
    },
    Unselect {
        pack_id: String,
        child_content_type: ContentType,
        child_content_id: String,
    },
}

#[derive(Subcommand)]
enum LeafCommand {
    List { source: ContentSource },
    Status { content_id: String },
    Fingerprint { content_id: String },
    Inspect { content_id: String },
    Validate { content_id: String },
    Install { content_id: String },
    Uninstall { content_id: String },
    Update { content_id: String },
}

#[derive(Clone, Copy, ValueEnum)]
enum ContentSource {
    Discovered,
    Local,
    Git,
    Workshop,
    All,
}

#[derive(Clone, Copy, ValueEnum)]
enum ContentType {
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

#[derive(Clone, Copy, ValueEnum)]
enum RepairTarget {
    #[value(name = "core_state")]
    CoreState,
    Toolchain,
    Steam,
    #[value(name = "content_catalog")]
    ContentCatalog,
    #[value(name = "content_library")]
    ContentLibrary,
    #[value(name = "active_composition")]
    ActiveComposition,
    All,
}

fn main() {
    let cli = Cli::parse();
    dispatch(&cli);
}

fn dispatch(cli: &Cli) {
    match &cli.command {
        Command::Version => stub(cli, "launcher version"),
        Command::Status => stub(cli, "launcher status"),
        Command::Repair { command } => dispatch_repair(cli, command),
        Command::Packagepack { command } => dispatch_packagepack(cli, command),
        Command::Enginepack { command } => dispatch_pack(cli, "enginepack", command),
        Command::Gamepack { command } => dispatch_pack(cli, "gamepack", command),
        Command::Modpack { command } => dispatch_pack(cli, "modpack", command),
        Command::Engine { command } => dispatch_leaf(cli, "engine", command),
        Command::Game { command } => dispatch_leaf(cli, "game", command),
        Command::EngineMod { command } => dispatch_leaf(cli, "engine_mod", command),
        Command::GameMod { command } => dispatch_leaf(cli, "game_mod", command),
        Command::ExtensionMod { command } => dispatch_leaf(cli, "extension_mod", command),
        Command::Lock => stub(cli, "launcher lock selected packagepack"),
        Command::Launch => stub(cli, "launcher launch selected packagepack"),
    }
}

fn dispatch_repair(cli: &Cli, command: &RepairCommand) {
    match command {
        RepairCommand::Status => stub(cli, "launcher repair status"),
        RepairCommand::Plan { .. } => stub(cli, "launcher repair plan"),
        RepairCommand::Apply { .. } => stub(cli, "launcher repair apply"),
    }
}

fn dispatch_packagepack(cli: &Cli, command: &PackagepackCommand) {
    match command {
        PackagepackCommand::List { .. } => stub(cli, "launcher packagepack list"),
        PackagepackCommand::Status { .. } => stub(cli, "launcher packagepack status"),
        PackagepackCommand::Fingerprint { .. } => stub(cli, "launcher packagepack fingerprint"),
        PackagepackCommand::Inspect { .. } => stub(cli, "launcher packagepack inspect"),
        PackagepackCommand::Validate { .. } => stub(cli, "launcher packagepack validate"),
        PackagepackCommand::Lock { .. } => stub(cli, "launcher packagepack lock"),
        PackagepackCommand::Install { .. } => stub(cli, "launcher packagepack install"),
        PackagepackCommand::Uninstall { .. } => stub(cli, "launcher packagepack uninstall"),
        PackagepackCommand::Update { .. } => stub(cli, "launcher packagepack update"),
        PackagepackCommand::New { .. } => stub(cli, "launcher packagepack new"),
        PackagepackCommand::Fork { .. } => stub(cli, "launcher packagepack fork"),
        PackagepackCommand::Select {
            child_content_type,
            child_content_id,
            ..
        } => {
            if child_content_type.is_some() == child_content_id.is_some() {
                stub(cli, "launcher packagepack select");
            } else {
                eprintln!("packagepack select needs either only <PACKAGEPACK_ID> or both child arguments");
            }
        }
        PackagepackCommand::Add { .. } => stub(cli, "launcher packagepack add"),
        PackagepackCommand::Remove { .. } => stub(cli, "launcher packagepack remove"),
        PackagepackCommand::Unselect { .. } => stub(cli, "launcher packagepack unselect"),
    }
}

fn dispatch_pack(cli: &Cli, content_type: &str, command: &PackCommand) {
    let action = match command {
        PackCommand::List { .. } => "list",
        PackCommand::Status { .. } => "status",
        PackCommand::Fingerprint { .. } => "fingerprint",
        PackCommand::Inspect { .. } => "inspect",
        PackCommand::Validate { .. } => "validate",
        PackCommand::Install { .. } => "install",
        PackCommand::Uninstall { .. } => "uninstall",
        PackCommand::Update { .. } => "update",
        PackCommand::New { .. } => "new",
        PackCommand::Fork { .. } => "fork",
        PackCommand::Add { .. } => "add",
        PackCommand::Remove { .. } => "remove",
        PackCommand::Select { .. } => "select",
        PackCommand::Unselect { .. } => "unselect",
    };
    stub(cli, &format!("launcher {content_type} {action}"));
}

fn dispatch_leaf(cli: &Cli, content_type: &str, command: &LeafCommand) {
    let action = match command {
        LeafCommand::List { .. } => "list",
        LeafCommand::Status { .. } => "status",
        LeafCommand::Fingerprint { .. } => "fingerprint",
        LeafCommand::Inspect { .. } => "inspect",
        LeafCommand::Validate { .. } => "validate",
        LeafCommand::Install { .. } => "install",
        LeafCommand::Uninstall { .. } => "uninstall",
        LeafCommand::Update { .. } => "update",
    };
    stub(cli, &format!("launcher {content_type} {action}"));
}

fn stub(cli: &Cli, action: &str) {
    println!("Doing {action}! Trust me, I am definitely doing it and not just a placeholder message.");

    if cli.verbose {
        println!("verbose: true");
        println!("yes: {}", cli.yes);
        println!("force: {}", cli.force);
        println!("strict: {}", cli.strict);
        println!("keep_unused_versions: {}", cli.keep_unused_versions);
    }
}
