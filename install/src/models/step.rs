#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardStep {
    Welcome,
    SourceBranch,
    Binaries,
    ExecuteInstall,
    Summary,
}

impl WizardStep {
    pub const ALL: [WizardStep; 5] = [
        WizardStep::Welcome,
        WizardStep::SourceBranch,
        WizardStep::Binaries,
        WizardStep::ExecuteInstall,
        WizardStep::Summary,
    ];

    pub fn title(&self) -> &'static str {
        match self {
            WizardStep::Welcome => "1. Welcome & Overview",
            WizardStep::SourceBranch => "2. Source Branch & Build",
            WizardStep::Binaries => "3. BabyDra Binaries",
            WizardStep::ExecuteInstall => "4. Execute Installation",
            WizardStep::Summary => "5. Summary & Launch",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            WizardStep::Welcome => "Welcome",
            WizardStep::SourceBranch => "Branch",
            WizardStep::Binaries => "Binaries",
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
