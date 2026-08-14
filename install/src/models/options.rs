#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallChannel {
    Release,
    Develop,
    LocalSource,
}

impl InstallChannel {
    pub fn name(&self) -> &'static str {
        match self {
            InstallChannel::Release => "Release Channel (Official Stable)",
            InstallChannel::Develop => "Develop Channel (Community & Feature Builds)",
            InstallChannel::LocalSource => "Local Directory (Pre-built Binaries)",
        }
    }

    pub fn git_branch(&self) -> &'static str {
        match self {
            InstallChannel::Release => "release",
            InstallChannel::Develop => "develop",
            InstallChannel::LocalSource => "local",
        }
    }

    #[allow(dead_code)]
    pub fn description(&self) -> &'static str {
        match self {
            InstallChannel::Release => {
                "Installs official tested stable binaries and configurations maintained by the author from 'release' branch."
            }
            InstallChannel::Develop => {
                "Installs community development builds containing latest experimental features from 'develop' branch."
            }
            InstallChannel::LocalSource => {
                "Installs pre-compiled binaries directly from the local target/release filesystem directory."
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BranchMetadata {
    pub channel: InstallChannel,
    pub branch_name: String,
    pub commit_hash: String,
    pub author_name: String,
    pub update_date: String,
    pub commit_msg: String,
}

impl Default for BranchMetadata {
    fn default() -> Self {
        Self {
            channel: InstallChannel::Release,
            branch_name: "release".into(),
            commit_hash: "N/A".into(),
            author_name: "N/A".into(),
            update_date: "N/A".into(),
            commit_msg: "No commit data available".into(),
        }
    }
}

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
            PresetProfile::BinariesAndBundle => "Binaries & /var/lib Staging Only",
            PresetProfile::Custom => "Custom Selection",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            PresetProfile::FullDesktop => {
                "Installs all binaries, /var/lib staging, labwc configs, themes, We10X icons, and greetd display manager."
            }
            PresetProfile::BinariesAndBundle => {
                "Copies all binaries to ~/.local/bin and /var/lib/babydra. Skips dotfiles and display manager."
            }
            PresetProfile::Custom => {
                "Manually configure individual packages, binary components, and system integrations."
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
