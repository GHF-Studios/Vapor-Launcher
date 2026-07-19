//! Clap parser and conversion into launcher-core command requests.

mod args;
mod leaf;
mod pack;
mod packagepack;
mod repair;

use clap::{Parser, Subcommand};
use leaf::LeafCommand;
use pack::PackCommand;
use packagepack::PackagepackCommand;
use repair::RepairCommand;
use vapor_launcher_core as core;

/// Parsed top-level Launcher invocation.
#[derive(Parser)]
#[command(name = "launcher_cli")]
#[command(version, about = "Install, compose, lock, and launch Vapor content.")]
#[command(
    long_about = "Launcher CLI for player and modpack-author workflows. Use it to inspect content, install packagepacks and dependencies, manage local mutable packs, lock the selected packagepack, and launch the selected experience."
)]
#[command(
    arg_required_else_help = true,
    subcommand_required = true,
    propagate_version = true
)]
pub(crate) struct Cli {
    /// Show operation planning, diagnostics, historical context, and live detail.
    #[arg(long, help_heading = "Output")]
    verbose: bool,
    /// Accept prompts automatically for commands that otherwise stop for confirmation.
    #[arg(long, help_heading = "Prompt Control")]
    yes: bool,
    /// Permit destructive or risk-bearing operations when that command supports it.
    #[arg(long, help_heading = "Prompt Control")]
    force: bool,
    /// Reject local pack mutations that would leave the pack invalid.
    #[arg(long, help_heading = "Prompt Control")]
    strict: bool,
    /// Keep old unused installed versions after update, lock, repair, or cleanup.
    #[arg(long, help_heading = "Prompt Control")]
    keep_unused_versions: bool,
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub(crate) fn into_parts(self) -> Result<(core::GlobalOptions, core::LauncherCommand), String> {
        let globals = core::GlobalOptions {
            verbose: self.verbose,
            yes: self.yes,
            force: self.force,
            strict: self.strict,
            keep_unused_versions: self.keep_unused_versions,
        };
        Ok((globals, self.command.into_core()?))
    }
}

/// Root Launcher workflows.
#[derive(Subcommand)]
enum Command {
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
    #[command(name = "engine-mod")]
    EngineMod {
        #[command(subcommand)]
        command: LeafCommand,
    },
    /// Work with installed game mod content.
    #[command(name = "game-mod")]
    GameMod {
        #[command(subcommand)]
        command: LeafCommand,
    },
    /// Work with installed extension mod content.
    #[command(name = "extension-mod")]
    ExtensionMod {
        #[command(subcommand)]
        command: LeafCommand,
    },
    /// Lock the currently selected packagepack.
    Lock,
    /// Launch the currently selected packagepack.
    Launch,
}

impl Command {
    fn into_core(self) -> Result<core::LauncherCommand, String> {
        Ok(match self {
            Self::Version => core::LauncherCommand::Version,
            Self::Status => core::LauncherCommand::Status,
            Self::Repair { command } => core::LauncherCommand::Repair(command.into_core()),
            Self::Packagepack { command } => {
                core::LauncherCommand::Packagepack(command.into_core()?)
            }
            Self::Enginepack { command } => pack_command(core::ContentType::Enginepack, command),
            Self::Gamepack { command } => pack_command(core::ContentType::Gamepack, command),
            Self::Modpack { command } => pack_command(core::ContentType::Modpack, command),
            Self::Engine { command } => leaf_command(core::ContentType::Engine, command),
            Self::Game { command } => leaf_command(core::ContentType::Game, command),
            Self::EngineMod { command } => leaf_command(core::ContentType::EngineMod, command),
            Self::GameMod { command } => leaf_command(core::ContentType::GameMod, command),
            Self::ExtensionMod { command } => {
                leaf_command(core::ContentType::ExtensionMod, command)
            }
            Self::Lock => core::LauncherCommand::LockSelectedPackagepack,
            Self::Launch => core::LauncherCommand::LaunchSelectedPackagepack,
        })
    }
}

fn pack_command(pack_type: core::ContentType, command: PackCommand) -> core::LauncherCommand {
    core::LauncherCommand::Pack {
        pack_type,
        command: command.into_core(),
    }
}

fn leaf_command(content_type: core::ContentType, command: LeafCommand) -> core::LauncherCommand {
    core::LauncherCommand::Leaf {
        content_type,
        command: command.into_core(),
    }
}
