#[derive(Debug, Clone)]
pub struct InstalledApp {
    pub name: String,
    pub description: String,
    pub desktop_file: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
}
