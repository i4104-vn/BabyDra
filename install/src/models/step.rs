#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardStep {
    Welcome = 0,
    SourceBranch = 1,
    SystemPackages = 2,
    Binaries = 3,
    VarLibBundle = 4,
    ConfigsThemes = 5,
    VariantSelection = 6,
    DisplayManager = 7,
    ExecuteInstall = 8,
    Summary = 9,
}

impl WizardStep {
    pub const ALL: [WizardStep; 10] = [
        WizardStep::Welcome,
        WizardStep::SourceBranch,
        WizardStep::SystemPackages,
        WizardStep::Binaries,
        WizardStep::VarLibBundle,
        WizardStep::ConfigsThemes,
        WizardStep::VariantSelection,
        WizardStep::DisplayManager,
        WizardStep::ExecuteInstall,
        WizardStep::Summary,
    ];

    pub fn title(&self) -> &'static str {
        match self {
            WizardStep::Welcome => "1. Welcome & Overview",
            WizardStep::SourceBranch => "2. Source Branch & Build",
            WizardStep::SystemPackages => "3. System Packages & Deps",
            WizardStep::Binaries => "4. BabyDra Binaries",
            WizardStep::VarLibBundle => "5. /var/lib Staging Bundle",
            WizardStep::ConfigsThemes => "6. Configs, Themes & Icons",
            WizardStep::VariantSelection => "7. Variant Selection",
            WizardStep::DisplayManager => "8. Greetd Display Manager",
            WizardStep::ExecuteInstall => "9. Execute Installation",
            WizardStep::Summary => "10. Summary & Launch",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            WizardStep::Welcome => "Welcome",
            WizardStep::SourceBranch => "Branch",
            WizardStep::SystemPackages => "Packages",
            WizardStep::Binaries => "Binaries",
            WizardStep::VarLibBundle => "VarLib Bundle",
            WizardStep::ConfigsThemes => "Configs & Themes",
            WizardStep::VariantSelection => "Variant",
            WizardStep::DisplayManager => "Display Manager",
            WizardStep::ExecuteInstall => "Install Progress",
            WizardStep::Summary => "Summary",
        }
    }

    pub fn next(&self) -> Option<WizardStep> {
        let idx = *self as usize;
        if idx + 1 < WizardStep::ALL.len() {
            Some(WizardStep::ALL[idx + 1])
        } else {
            None
        }
    }

    pub fn prev(&self) -> Option<WizardStep> {
        let idx = *self as usize;
        if idx > 0 {
            Some(WizardStep::ALL[idx - 1])
        } else {
            None
        }
    }
}
