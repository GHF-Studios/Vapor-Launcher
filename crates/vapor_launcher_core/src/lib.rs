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
    LeafCommand, PackagepackCommand, PackCommand, PackCompositionCommand,
};
pub use content::{allowed_pack_children, ContentSource, ContentType};
pub use options::GlobalOptions;
pub use repair::{RepairCommand, RepairTarget};
pub use spec::{describe_command, CommandSpec, StateSurface};
pub use vapor_core::ChildContentRef;
