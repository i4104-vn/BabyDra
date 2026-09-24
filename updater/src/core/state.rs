#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewState {
    Menu,
    Running,
    Finished,
    FactoryResetModal,
    ComponentModal,
    HelpModal,
    SudoModal,
}

#[derive(Debug, Clone)]
pub struct ActionItem {
    pub id: ActionId,
    pub title: &'static str,
    pub tag: &'static str,
    pub description: &'static str,
    pub requires_sudo: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionId {
    UpdateReload,
    SafetyCheck,
    StartDesktop,
    ComponentRestart,
    SyncConfigs,
    CleanWorkspace,
    FactoryReset,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResetMode {
    KeepPackages,
    RemoveShellPackages,
    DryRun,
    RemoveAllApps,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentTarget {
    Panel,
    Desktop,
    Switcher,
    Keymap,
    ReconfigureLabwc,
    RefreshGtkFonts,
}
