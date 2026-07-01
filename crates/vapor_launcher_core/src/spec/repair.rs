//! Launcher repair command specifications.

use super::{spec, CommandSpec, StateSurface};
use crate::repair::RepairCommand;

pub(super) fn describe(command: &RepairCommand) -> CommandSpec {
    match command {
        RepairCommand::Status => spec(
            "launcher repair status",
            "Inspect repairable Launcher targets without proposing mutation.",
            StateSurface::ReadOnly,
            &[],
            &["display repair target health"],
        ),
        RepairCommand::Plan { .. } => spec(
            "launcher repair plan",
            "Prepare a repair plan without applying it.",
            StateSurface::RepairPlan,
            &["repair target is known"],
            &["compute proposed repair operations"],
        ),
        RepairCommand::Apply { .. } => spec(
            "launcher repair apply",
            "Apply repairs for a Launcher target.",
            StateSurface::RepairApply,
            &["repair target is known", "planned mutations are acceptable"],
            &["repair selected Launcher-managed state"],
        ),
    }
}
