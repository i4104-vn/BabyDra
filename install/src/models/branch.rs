/// A git branch selectable as the installation source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchItem {
    /// Short branch name (e.g. `release`, `develop`).
    pub name: String,
    /// Whether this is the currently checked-out branch.
    pub is_current: bool,
    /// Whether the branch exists on the remote (`origin/<name>`).
    pub has_remote: bool,
    /// Whether this branch was selected by the user.
    pub selected: bool,
}
