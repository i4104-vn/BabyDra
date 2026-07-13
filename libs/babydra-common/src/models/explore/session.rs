use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use crate::models::explore::tab::TabState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivePane {
    Left,
    Right,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionState {
    pub tabs: Vec<TabState>,
    pub active_tab_index: usize,
    pub split_mode: bool,
    pub right_active: bool,
}

impl SessionState {
    pub fn new(default_path: PathBuf) -> Self {
        Self {
            tabs: vec![TabState::new(default_path)],
            active_tab_index: 0,
            split_mode: false,
            right_active: false,
        }
    }

    pub fn active_tab(&self) -> &TabState {
        &self.tabs[self.active_tab_index]
    }

    pub fn active_tab_mut(&mut self) -> &mut TabState {
        &mut self.tabs[self.active_tab_index]
    }

    pub fn add_tab(&mut self, path: PathBuf) -> usize {
        let new_tab = TabState::new(path);
        self.tabs.push(new_tab);
        self.active_tab_index = self.tabs.len() - 1;
        self.active_tab_index
    }

    pub fn close_tab(&mut self, index: usize) -> bool {
        if self.tabs.len() <= 1 {
            return false; // Cannot close the last remaining tab
        }

        self.tabs.remove(index);
        if self.active_tab_index >= self.tabs.len() {
            self.active_tab_index = self.tabs.len() - 1;
        }
        true
    }
}
