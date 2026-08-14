#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryLocation {
    UserLocalBin, // ~/.local/bin
    SystemBin,    // /usr/bin
}

#[derive(Debug, Clone)]
pub struct BinaryItem {
    pub name: String,
    pub description: String,
    pub crate_path: String,
    pub default_dest: BinaryLocation,
    pub selected: bool,
    pub exists_in_source: bool,
    pub source_size_bytes: Option<u64>,
    pub exists_in_target: bool,
    pub status_note: String,
}
