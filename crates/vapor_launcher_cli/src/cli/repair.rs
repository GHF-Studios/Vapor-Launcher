//! Clap commands for launcher repair workflows.

use clap::{Subcommand, ValueEnum};
use vapor_launcher_core as core;

/// Repair commands intentionally split planning from mutation.
#[derive(Subcommand)]
pub(super) enum RepairCommand {
    /// Inspect repairable areas without proposing mutations.
    Status,
    /// Produce a repair plan without applying it.
    Plan { target: RepairTarget },
    /// Apply repairs for a target after the plan is accepted.
    Apply { target: RepairTarget },
}

impl RepairCommand {
    pub(super) fn into_core(self) -> core::RepairCommand {
        match self {
            Self::Status => core::RepairCommand::Status,
            Self::Plan { target } => core::RepairCommand::Plan { target: target.into() },
            Self::Apply { target } => core::RepairCommand::Apply { target: target.into() },
        }
    }
}

/// Repair target used by `repair plan` and `repair apply`.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub(super) enum RepairTarget {
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

impl From<RepairTarget> for core::RepairTarget {
    fn from(value: RepairTarget) -> Self {
        match value {
            RepairTarget::CoreState => Self::CoreState,
            RepairTarget::Toolchain => Self::Toolchain,
            RepairTarget::Steam => Self::Steam,
            RepairTarget::ContentCatalog => Self::ContentCatalog,
            RepairTarget::ContentLibrary => Self::ContentLibrary,
            RepairTarget::ActiveComposition => Self::ActiveComposition,
            RepairTarget::All => Self::All,
        }
    }
}
