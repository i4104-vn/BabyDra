pub mod desktop;
pub mod loader;
pub mod model;
pub mod workspace;

pub use loader::load_config;
pub use model::UpdaterConfig;
pub use workspace::WorkspaceBinary;
