//! Clap commands for packagepack launcher workflows.

use super::args::{ContentSource, ContentType};
use clap::Subcommand;
use vapor_launcher_core as core;

/// Packagepack commands include both root-level user selection and local mutable pack editing.
#[derive(Subcommand)]
pub(super) enum PackagepackCommand {
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

impl PackagepackCommand {
    pub(super) fn into_core(self) -> Result<core::PackagepackCommand, String> {
        Ok(match self {
            Self::List { source } => read(core::ContentReadCommand::List {
                source: source.into(),
            }),
            Self::Status { packagepack_id } => read(core::ContentReadCommand::Status {
                content_id: packagepack_id,
            }),
            Self::Fingerprint { packagepack_id } => read(core::ContentReadCommand::Fingerprint {
                content_id: packagepack_id,
            }),
            Self::Inspect { packagepack_id } => read(core::ContentReadCommand::Inspect {
                content_id: packagepack_id,
            }),
            Self::Validate { packagepack_id } => read(core::ContentReadCommand::Validate {
                content_id: packagepack_id,
            }),
            Self::Lock { packagepack_id } => core::PackagepackCommand::Lock { packagepack_id },
            Self::Install { packagepack_id } => installed(core::LauncherInstallCommand::Install {
                content_id: packagepack_id,
            }),
            Self::Uninstall { packagepack_id } => {
                installed(core::LauncherInstallCommand::Uninstall {
                    content_id: packagepack_id,
                })
            }
            Self::Update { packagepack_id } => installed(core::LauncherInstallCommand::Update {
                content_id: packagepack_id,
            }),
            Self::New { packagepack_id } => local(core::LauncherLocalPackCommand::New {
                pack_id: packagepack_id,
            }),
            Self::Fork {
                source_pack_id,
                new_pack_id,
            } => local(core::LauncherLocalPackCommand::Fork {
                source_pack_id,
                new_pack_id,
            }),
            Self::Select {
                packagepack_id,
                child_content_type,
                child_content_id,
            } => select(packagepack_id, child_content_type, child_content_id)?,
            Self::Add {
                packagepack_id,
                child_content_type,
                child_content_id,
            } => compose(core::PackCompositionCommand::Add {
                pack_id: packagepack_id,
                child: super::args::child(child_content_type, child_content_id),
            }),
            Self::Remove {
                packagepack_id,
                child_content_type,
                child_content_id,
            } => compose(core::PackCompositionCommand::Remove {
                pack_id: packagepack_id,
                child: super::args::child(child_content_type, child_content_id),
            }),
            Self::Unselect {
                packagepack_id,
                child_content_type,
                child_content_id,
            } => compose(core::PackCompositionCommand::Unselect {
                pack_id: packagepack_id,
                child: super::args::child(child_content_type, child_content_id),
            }),
        })
    }
}

fn read(command: core::ContentReadCommand) -> core::PackagepackCommand {
    core::PackagepackCommand::Read(command)
}

fn installed(command: core::LauncherInstallCommand) -> core::PackagepackCommand {
    core::PackagepackCommand::Installed(command)
}

fn local(command: core::LauncherLocalPackCommand) -> core::PackagepackCommand {
    core::PackagepackCommand::Local(command)
}

fn compose(command: core::PackCompositionCommand) -> core::PackagepackCommand {
    core::PackagepackCommand::Compose(command)
}

fn select(
    packagepack_id: String,
    child_content_type: Option<ContentType>,
    child_content_id: Option<String>,
) -> Result<core::PackagepackCommand, String> {
    match (child_content_type, child_content_id) {
        (None, None) => Ok(core::PackagepackCommand::SelectRoot { packagepack_id }),
        (Some(content_type), Some(content_id)) => {
            Ok(compose(core::PackCompositionCommand::Select {
                pack_id: packagepack_id,
                child: super::args::child(content_type, content_id),
            }))
        }
        _ => Err(
            "packagepack select needs either only <PACKAGEPACK_ID> or both child arguments"
                .to_owned(),
        ),
    }
}
