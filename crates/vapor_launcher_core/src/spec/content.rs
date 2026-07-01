//! Launcher content command specifications.

use super::{spec, CommandSpec, StateSurface};
use crate::commands::{
    ContentReadCommand, LauncherInstallCommand, LauncherLocalPackCommand, LeafCommand,
    PackagepackCommand, PackCommand, PackCompositionCommand,
};
use crate::content::ContentType;

pub(super) fn describe_packagepack(command: &PackagepackCommand) -> CommandSpec {
    match command {
        PackagepackCommand::Read(command) => describe_read(ContentType::Packagepack, command),
        PackagepackCommand::Installed(command) => {
            describe_installed(ContentType::Packagepack, command)
        }
        PackagepackCommand::Local(command) => describe_local_pack(ContentType::Packagepack, command),
        PackagepackCommand::Compose(command) => {
            describe_composition(ContentType::Packagepack, command)
        }
        PackagepackCommand::SelectRoot { .. } => spec(
            "launcher packagepack select",
            "Select the active root packagepack used by launcher lock and launch.",
            StateSurface::ActiveComposition,
            &["packagepack is installed or otherwise locally available"],
            &["record selected root packagepack"],
        ),
        PackagepackCommand::Lock { .. } => spec(
            "launcher packagepack lock",
            "Write a persistent lock artifact for an explicit packagepack.",
            StateSurface::ActiveComposition,
            &["packagepack is locally available", "packagepack can be resolved"],
            &["resolve explicit packagepack", "write or update its lock artifact"],
        ),
    }
}

pub(super) fn describe_pack(pack_type: ContentType, command: &PackCommand) -> CommandSpec {
    match command {
        PackCommand::Read(command) => describe_read(pack_type, command),
        PackCommand::Installed(command) => describe_installed(pack_type, command),
        PackCommand::Local(command) => describe_local_pack(pack_type, command),
        PackCommand::Compose(command) => describe_composition(pack_type, command),
    }
}

pub(super) fn describe_leaf(content_type: ContentType, command: &LeafCommand) -> CommandSpec {
    match command {
        LeafCommand::Read(command) => describe_read(content_type, command),
        LeafCommand::Installed(command) => describe_installed(content_type, command),
    }
}

fn describe_read(content_type: ContentType, command: &ContentReadCommand) -> CommandSpec {
    let action = match command {
        ContentReadCommand::List { .. } => "list",
        ContentReadCommand::Status { .. } => "status",
        ContentReadCommand::Fingerprint { .. } => "fingerprint",
        ContentReadCommand::Inspect { .. } => "inspect",
        ContentReadCommand::Validate { .. } => "validate",
    };
    read_spec(format!("launcher {} {action}", content_type.as_str()), "Read launcher-known content state.")
}

fn describe_installed(content_type: ContentType, command: &LauncherInstallCommand) -> CommandSpec {
    let (action, summary) = match command {
        LauncherInstallCommand::Install { .. } => ("install", "Install content and its dependency closure."),
        LauncherInstallCommand::Uninstall { .. } => ("uninstall", "Remove installed content when dependency safety permits it."),
        LauncherInstallCommand::Update { .. } => ("update", "Update installed content."),
    };
    installed_spec(format!("launcher {} {action}", content_type.as_str()), summary)
}

fn describe_local_pack(pack_type: ContentType, command: &LauncherLocalPackCommand) -> CommandSpec {
    let (action, summary) = match command {
        LauncherLocalPackCommand::New { .. } => ("new", "Create a blank local mutable pack."),
        LauncherLocalPackCommand::Fork { .. } => ("fork", "Create a local mutable pack from an existing source pack."),
    };
    local_pack_spec(format!("launcher {} {action}", pack_type.as_str()), summary)
}

fn describe_composition(pack_type: ContentType, command: &PackCompositionCommand) -> CommandSpec {
    let action = match command {
        PackCompositionCommand::Add { .. } => "add",
        PackCompositionCommand::Remove { .. } => "remove",
        PackCompositionCommand::Select { .. } => "select",
        PackCompositionCommand::Unselect { .. } => "unselect",
    };
    composition_spec(
        format!("launcher {} {action}", pack_type.as_str()),
        "Mutate child membership or active child selection inside a local mutable pack.",
    )
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
