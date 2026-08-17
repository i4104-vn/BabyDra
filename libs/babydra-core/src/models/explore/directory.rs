use crate::models::explore::file_entry::FileEntry;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortColumn {
    Name,
    Size,
    Modified,
    Type,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DirectoryModel {
    pub path: PathBuf,
    pub entries: Vec<FileEntry>,
    pub sort_column: SortColumn,
    pub sort_order: SortOrder,
    pub show_hidden: bool,
    pub filter: Option<String>,
}

impl DirectoryModel {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            entries: Vec::new(),
            sort_column: SortColumn::Name,
            sort_order: SortOrder::Ascending,
            show_hidden: false,
            filter: None,
        }
    }

    /// Sorts the internal entries vector based on sort_column, sort_order, and sorting folders first behavior.
    pub fn sort(&mut self, folders_first: bool) {
        self.entries.sort_by(|a, b| {
            if folders_first {
                let a_is_dir = matches!(
                    a.file_type,
                    crate::models::explore::file_entry::FileType::Directory
                );
                let b_is_dir = matches!(
                    b.file_type,
                    crate::models::explore::file_entry::FileType::Directory
                );
                if a_is_dir && !b_is_dir {
                    return std::cmp::Ordering::Less;
                }
                if !a_is_dir && b_is_dir {
                    return std::cmp::Ordering::Greater;
                }
            }

            let cmp = match self.sort_column {
                SortColumn::Name => a
                    .display_name
                    .to_lowercase()
                    .cmp(&b.display_name.to_lowercase()),
                SortColumn::Size => a.size.cmp(&b.size),
                SortColumn::Modified => a.modified.cmp(&b.modified),
                SortColumn::Type => a.mime_type.cmp(&b.mime_type),
            };

            if self.sort_order == SortOrder::Descending {
                cmp.reverse()
            } else {
                cmp
            }
        });
    }
}
