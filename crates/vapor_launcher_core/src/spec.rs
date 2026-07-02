//! Human-readable launcher command specifications for current stub handlers.

mod content;
mod repair;

use crate::commands::LauncherCommand;

/// Broad state surface a future implementation may read or mutate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateSurface {
    ReadOnly,
    RepairPlan,
    RepairApply,
    InstalledLibrary,
    LocalMutablePack,
    ActiveComposition,
    RuntimeLaunch,
}

/// Command contract used by placeholder UIs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSpec {
    pub action: String,
    pub summary: &'static str,
    pub surface: StateSurface,
    pub preconditions: &'static [&'static str],
    pub future_effects: &'static [&'static str],
}

/// Describe a launcher command without executing it.
pub fn describe_command(command: &LauncherCommand) -> CommandSpec {
    match command {
        LauncherCommand::Version => spec(
            "launcher version",
            "Print Launcher version and build identity.",
            StateSurface::ReadOnly,
            &[],
            &["display version metadata"],
        ),
        LauncherCommand::Status => spec(
            "launcher status",
            "Summarize Launcher health, selected packagepack, and local state.",
            StateSurface::ReadOnly,
            &[],
            &["display local state and repair hints"],
        ),
        LauncherCommand::Repair(command) => repair::describe(command),
        LauncherCommand::Packagepack(command) => content::describe_packagepack(command),
        LauncherCommand::Pack { pack_type, command } => content::describe_pack(*pack_type, command),
        LauncherCommand::Leaf {
            content_type,
            command,
        } => content::describe_leaf(*content_type, command),
        LauncherCommand::LockSelectedPackagepack => spec(
            "launcher lock selected packagepack",
            "Write a persistent lock artifact for the selected packagepack.",
            StateSurface::ActiveComposition,
            &[
                "one packagepack is selected",
                "the selected packagepack can be resolved",
            ],
            &[
                "resolve the selected packagepack",
                "write or update its lock artifact",
            ],
        ),
        LauncherCommand::LaunchSelectedPackagepack => spec(
            "launcher launch selected packagepack",
            "Launch the selected packagepack.",
            StateSurface::RuntimeLaunch,
            &[
                "one packagepack is selected",
                "the selected packagepack has a usable locked graph",
            ],
            &[
                "prepare runtime inputs",
                "start the selected packagepack experience",
            ],
        ),
    }
}

pub(super) fn spec(
    action: impl Into<String>,
    summary: &'static str,
    surface: StateSurface,
    preconditions: &'static [&'static str],
    future_effects: &'static [&'static str],
) -> CommandSpec {
    CommandSpec {
        action: action.into(),
        summary,
        surface,
        preconditions,
        future_effects,
    }
}
