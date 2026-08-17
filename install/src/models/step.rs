#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardStep {
    Welcome = 0,
    SystemPackages = 1,
    Binaries = 2,
    VarLibBundle = 3,
    ConfigsThemes = 4,
    VariantSelection = 5,
    DisplayManager = 6,
    ExecuteInstall = 7,
    Summary = 8,
}

impl WizardStep {
    pub const ALL: [WizardStep; 9] = [
        WizardStep::Welcome,
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
            WizardStep::SystemPackages => "2. System Packages & Deps",
            WizardStep::Binaries => "3. BabyDra Binaries",
            WizardStep::VarLibBundle => "4. /var/lib Staging Bundle",
            WizardStep::ConfigsThemes => "5. Configs, Themes & Icons",
            WizardStep::VariantSelection => "6. Variant Selection",
            WizardStep::DisplayManager => "7. Greetd Display Manager",
            WizardStep::ExecuteInstall => "8. Execute Installation",
            WizardStep::Summary => "9. Summary & Launch",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            WizardStep::Welcome => "Welcome",
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
