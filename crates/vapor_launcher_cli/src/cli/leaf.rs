//! Clap commands for leaf content managed by the launcher.

use super::args::ContentSource;
use clap::Subcommand;
use vapor_launcher_core as core;

/// Commands shared by leaf content types that cannot contain child content.
#[derive(Subcommand)]
pub(super) enum LeafCommand {
    List { source: ContentSource },
    Status { content_id: String },
    Fingerprint { content_id: String },
    Inspect { content_id: String },
    Validate { content_id: String },
    Install { content_id: String },
    Uninstall { content_id: String },
    Update { content_id: String },
}

impl LeafCommand {
    pub(super) fn into_core(self) -> core::LeafCommand {
        match self {
            Self::List { source } => {
                core::LeafCommand::Read(core::ContentReadCommand::List { source: source.into() })
            }
            Self::Status { content_id } => {
                core::LeafCommand::Read(core::ContentReadCommand::Status { content_id })
            }
            Self::Fingerprint { content_id } => {
                core::LeafCommand::Read(core::ContentReadCommand::Fingerprint { content_id })
            }
            Self::Inspect { content_id } => {
                core::LeafCommand::Read(core::ContentReadCommand::Inspect { content_id })
            }
            Self::Validate { content_id } => {
                core::LeafCommand::Read(core::ContentReadCommand::Validate { content_id })
            }
            Self::Install { content_id } => {
                core::LeafCommand::Installed(core::LauncherInstallCommand::Install { content_id })
            }
            Self::Uninstall { content_id } => {
                core::LeafCommand::Installed(core::LauncherInstallCommand::Uninstall { content_id })
            }
            Self::Update { content_id } => {
                core::LeafCommand::Installed(core::LauncherInstallCommand::Update { content_id })
            }
        }
    }
}
