//! Reusable launcher command vocabulary and command specifications.
//!
//! This crate still does not implement installation, locking, Steam, repair, or
//! launch behavior. It defines the typed request model and documented command
//! contracts that the CLI, and later a GUI, can share.

#![forbid(unsafe_code)]

pub mod commands;
pub mod content;
pub mod options;
pub mod repair;
pub mod spec;

pub use commands::{
    ContentReadCommand, LauncherCommand, LauncherInstallCommand, LauncherLocalPackCommand,
    LeafCommand, PackCommand, PackCompositionCommand, PackagepackCommand,
};
pub use content::{ContentSource, ContentType, allowed_pack_children};
pub use options::GlobalOptions;
pub use repair::{RepairCommand, RepairTarget};
pub use spec::{CommandSpec, StateSurface, describe_command};
pub use vapor_core::ChildContentRef;
