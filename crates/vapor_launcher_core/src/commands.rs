//! Typed launcher command requests.

use crate::content::{ContentSource, ContentType};
use crate::repair::RepairCommand;
use vapor_core::ChildContentRef;

/// Read-only commands shared by every launcher content kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentReadCommand {
    /// List content from one source.
    List { source: ContentSource },
    /// Show local status for one content identity.
    Status { content_id: String },
    /// Compute or display a deterministic content fingerprint.
    Fingerprint { content_id: String },
    /// Inspect metadata and local launcher state.
    Inspect { content_id: String },
    /// Validate metadata and compatibility requirements.
    Validate { content_id: String },
}

/// Installed-library commands shared by every launcher content kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LauncherInstallCommand {
    /// Install content and its required dependency closure.
    Install { content_id: String },
    /// Remove installed content when dependency safety permits it.
    Uninstall { content_id: String },
    /// Update installed content.
    Update { content_id: String },
}

/// Local mutable pack creation commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LauncherLocalPackCommand {
    /// Create a blank local mutable pack.
    New { pack_id: String },
    /// Create a local mutable pack from an existing source pack.
    Fork { source_pack_id: String, new_pack_id: String },
}

/// Composition mutations for local mutable packs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackCompositionCommand {
    /// Add child content to a local mutable pack.
    Add { pack_id: String, child: ChildContentRef },
    /// Remove child content from a local mutable pack.
    Remove { pack_id: String, child: ChildContentRef },
    /// Select active child content inside a local mutable pack.
    Select { pack_id: String, child: ChildContentRef },
    /// Keep child content present but inactive inside a local mutable pack.
    Unselect { pack_id: String, child: ChildContentRef },
}

/// Packagepack commands include both root-level user selection and local mutable pack editing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackagepackCommand {
    Read(ContentReadCommand),
    Installed(LauncherInstallCommand),
    Local(LauncherLocalPackCommand),
    Compose(PackCompositionCommand),
    /// Select the active root packagepack used by root `lock` and `launch`.
    SelectRoot { packagepack_id: String },
    /// Lock an explicit packagepack, selected or not.
    Lock { packagepack_id: String },
}

/// Commands shared by non-root pack types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackCommand {
    Read(ContentReadCommand),
    Installed(LauncherInstallCommand),
    Local(LauncherLocalPackCommand),
    Compose(PackCompositionCommand),
}

/// Commands shared by leaf content types that cannot contain child content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeafCommand {
    Read(ContentReadCommand),
    Installed(LauncherInstallCommand),
}

/// Root Launcher workflows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LauncherCommand {
    /// Print Launcher version/build identity.
    Version,
    /// Summarize Launcher health, selected packagepack, and local state.
    Status,
    /// Inspect, plan, or apply repairs to Launcher-managed state.
    Repair(RepairCommand),
    /// Work with packagepacks, including root selection and packagepack locking.
    Packagepack(PackagepackCommand),
    /// Work with pack content as installed content or local mutable packs.
    Pack { pack_type: ContentType, command: PackCommand },
    /// Work with installed leaf content.
    Leaf { content_type: ContentType, command: LeafCommand },
    /// Lock the currently selected packagepack.
    LockSelectedPackagepack,
    /// Launch the currently selected packagepack.
    LaunchSelectedPackagepack,
}
