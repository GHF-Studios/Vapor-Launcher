//! Global command options shared by every launcher surface.

/// Global execution knobs accepted by every launcher command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GlobalOptions {
    /// Print operation planning, diagnostics, historical context, and live detail.
    pub verbose: bool,
    /// Accept non-destructive interactive prompts.
    pub yes: bool,
    /// Accept destructive or risk-bearing operations when the command supports it.
    pub force: bool,
    /// Reject local pack mutations that would leave the pack invalid.
    pub strict: bool,
    /// Keep old unused installed versions after update, lock, repair, or cleanup.
    pub keep_unused_versions: bool,
}
