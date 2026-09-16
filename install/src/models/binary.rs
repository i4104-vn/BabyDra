#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryLocation {
    UserLocalBin, // ~/.local/bin
    SystemBin,    // /usr/bin
}

#[derive(Debug, Clone)]
pub struct BinaryItem {
    pub name: String,
    /// File name emitted in the selected release directory. Usually equal to
    /// `name`, but kept separate so the manifest can rename a command without
    /// changing the build layout.
    pub source_name: String,
    pub description: String,
    pub crate_path: String,
    pub default_dest: BinaryLocation,
    /// Whether the installer should generate a desktop entry when the source
    /// branch does not provide one with the same binary name.
    pub export_desktop: bool,
    pub selected: bool,
    pub exists_in_source: bool,
    pub source_size_bytes: Option<u64>,
    pub exists_in_target: bool,
}
