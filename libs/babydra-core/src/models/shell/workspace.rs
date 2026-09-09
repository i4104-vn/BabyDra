use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workspace {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub is_active: bool,
}

impl Workspace {
    pub fn new(id: u32, name: &str, is_active: bool) -> Self {
        let icon = match id {
            1 => "workspace-1",
            2 => "workspace-2",
            3 => "workspace-3",
            4 => "workspace-4",
            _ => "window",
        };
        Self {
            id,
            name: name.to_string(),
            icon: icon.to_string(),
            is_active,
        }
    }
}
