use std::path::PathBuf;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TabState {
    pub id: Uuid,
    pub current_path: PathBuf,
    pub history: Vec<PathBuf>,
    pub history_index: usize,
    pub selection: Vec<PathBuf>,
    pub view_mode: String, // "icons" | "list" | "detail"
    pub show_hidden: bool,
}

impl TabState {
    pub fn new(path: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4(),
            current_path: path.clone(),
            history: vec![path],
            history_index: 0,
            selection: Vec::new(),
            view_mode: "icons".to_string(),
            show_hidden: false,
        }
    }

    pub fn navigate_to(&mut self, path: PathBuf) {
        // Truncate history forward if we were navigated back
        if self.history_index + 1 < self.history.len() {
            self.history.truncate(self.history_index + 1);
        }
        self.history.push(path.clone());
        self.history_index = self.history.len() - 1;
        self.current_path = path;
        self.selection.clear();
    }

    pub fn go_back(&mut self) -> bool {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.current_path = self.history[self.history_index].clone();
            self.selection.clear();
            true
        } else {
            false
        }
    }

    pub fn go_forward(&mut self) -> bool {
        if self.history_index + 1 < self.history.len() {
            self.history_index += 1;
            self.current_path = self.history[self.history_index].clone();
            self.selection.clear();
            true
        } else {
            false
        }
    }

    pub fn go_up(&mut self) -> bool {
        if let Some(parent) = self.current_path.parent() {
            self.navigate_to(parent.to_path_buf());
            true
        } else {
            false
        }
    }
}
