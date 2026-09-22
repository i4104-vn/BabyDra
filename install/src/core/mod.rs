pub mod context;
pub mod event;
pub mod manifest;
pub mod pipeline;

pub use context::TaskContext;
pub use event::{InstallEvent, InstallPlan};
pub use manifest::{
    load_install_manifest, parse_manifest, BinaryManifestItem, BuildDepConfig, ConfigRule,
    DesktopConfig, GreetdConfig, InstallManifest, PermissionsConfig, StagingConfig, ThemeConfig,
};
pub use pipeline::{build_pipeline, TaskStep};
