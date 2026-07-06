//! Clap commands for mutable pack content managed by the launcher.

use super::args::{ContentSource, ContentType, child};
use clap::Subcommand;
use vapor_launcher_core as core;

/// Commands shared by non-root pack types.
#[derive(Subcommand)]
pub(super) enum PackCommand {
    List {
        source: ContentSource,
    },
    Status {
        pack_id: String,
    },
    Fingerprint {
        pack_id: String,
    },
    Inspect {
        pack_id: String,
    },
    Validate {
        pack_id: String,
    },
    Install {
        pack_id: String,
    },
    Uninstall {
        pack_id: String,
    },
    Update {
        pack_id: String,
    },
    New {
        pack_id: String,
    },
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

impl PackCommand {
    pub(super) fn into_core(self) -> core::PackCommand {
        match self {
            Self::List { source } => read(core::ContentReadCommand::List { source }),
            Self::Status { pack_id } => read(core::ContentReadCommand::Status {
                content_id: pack_id,
            }),
            Self::Fingerprint { pack_id } => read(core::ContentReadCommand::Fingerprint {
                content_id: pack_id,
            }),
            Self::Inspect { pack_id } => read(core::ContentReadCommand::Inspect {
                content_id: pack_id,
            }),
            Self::Validate { pack_id } => read(core::ContentReadCommand::Validate {
                content_id: pack_id,
            }),
            Self::Install { pack_id } => installed(core::LauncherInstallCommand::Install {
                content_id: pack_id,
            }),
            Self::Uninstall { pack_id } => installed(core::LauncherInstallCommand::Uninstall {
                content_id: pack_id,
            }),
            Self::Update { pack_id } => installed(core::LauncherInstallCommand::Update {
                content_id: pack_id,
            }),
            Self::New { pack_id } => local(core::LauncherLocalPackCommand::New { pack_id }),
            Self::Fork {
                source_pack_id,
                new_pack_id,
            } => local(core::LauncherLocalPackCommand::Fork {
                source_pack_id,
                new_pack_id,
            }),
            Self::Add {
                pack_id,
                child_content_type,
                child_content_id,
            } => compose(core::PackCompositionCommand::Add {
                pack_id,
                child: child(child_content_type, child_content_id),
            }),
            Self::Remove {
                pack_id,
                child_content_type,
                child_content_id,
            } => compose(core::PackCompositionCommand::Remove {
                pack_id,
                child: child(child_content_type, child_content_id),
            }),
            Self::Select {
                pack_id,
                child_content_type,
                child_content_id,
            } => compose(core::PackCompositionCommand::Select {
                pack_id,
                child: child(child_content_type, child_content_id),
            }),
            Self::Unselect {
                pack_id,
                child_content_type,
                child_content_id,
            } => compose(core::PackCompositionCommand::Unselect {
                pack_id,
                child: child(child_content_type, child_content_id),
            }),
        }
    }
}

fn read(command: core::ContentReadCommand) -> core::PackCommand {
    core::PackCommand::Read(command)
}

fn installed(command: core::LauncherInstallCommand) -> core::PackCommand {
    core::PackCommand::Installed(command)
}

fn local(command: core::LauncherLocalPackCommand) -> core::PackCommand {
    core::PackCommand::Local(command)
}

fn compose(command: core::PackCompositionCommand) -> core::PackCommand {
    core::PackCommand::Compose(command)
}
