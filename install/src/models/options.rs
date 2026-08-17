#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetProfile {
    FullDesktop,
    BinariesAndBundle,
    Custom,
}

impl PresetProfile {
    pub fn name(&self) -> &'static str {
        match self {
            PresetProfile::FullDesktop => "Full Desktop (Recommended)",
            PresetProfile::BinariesAndBundle => "Binaries & /var/lib Only",
            PresetProfile::Custom => "Custom Selection",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            PresetProfile::FullDesktop => {
                "Installs all binaries, /var/lib staging, labwc configs, themes, icons, and greetd display manager."
            }
            PresetProfile::BinariesAndBundle => {
                "Copies all compiled binaries to ~/.local/bin and stages /var/lib/babydra bundle. Skips dotfiles and greetd."
            }
            PresetProfile::Custom => {
                "Manually customize each step, package, binary, and system configuration."
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericOptionItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub detail: String,
    pub selected: bool,
    pub requires_root: bool,
}
