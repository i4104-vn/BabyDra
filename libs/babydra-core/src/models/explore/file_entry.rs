use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    Directory,
    RegularFile,
    Symlink,
    Socket,
    BlockDevice,
    CharacterDevice,
    Fifo,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: OsString,
    pub display_name: String,
    pub file_type: FileType,
    pub mime_type: String,
    pub size: u64,
    pub modified: Option<SystemTime>,
    pub created: Option<SystemTime>,
    pub permissions: u32,
    pub owner: String,
    pub group: String,
    pub is_hidden: bool,
    pub icon_name: String,
    pub thumbnail_path: Option<PathBuf>,
}
