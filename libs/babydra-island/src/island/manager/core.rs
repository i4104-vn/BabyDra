//! Internal state structure and active view tracking for Dynamic Island.

use std::cell::Cell;
use std::time::Instant;

use gtk4::prelude::*;

use crate::island::models::{IslandConfig, IslandDisplay, ViewRecord};

pub(crate) struct IslandCore {
    pub cfg: IslandConfig,
    #[allow(dead_code)]
    pub root: gtk4::Box,
    pub capsule: gtk4::Box,
    pub content_box: gtk4::Box,
    pub left_bracket: gtk4::Label,
    pub right_bracket: gtk4::Label,
    pub idle: Option<gtk4::Widget>,
    pub views: Vec<ViewRecord>,
    pub displayed: IslandDisplay,
    pub pending: Option<IslandDisplay>,
    pub animating: Cell<bool>,
    pub hovered: Cell<bool>,
    pub user_selected: Option<(usize, u64)>,
    pub last_scroll: Cell<Option<Instant>>,
}

impl IslandCore {
    /// Returns the indices of all currently active views.
    pub fn get_active_indices(&self) -> Vec<usize> {
        let mut active = Vec::new();
        for (i, v) in self.views.iter().enumerate() {
            let is_active = v.state.override_active.get() || v.state.requested.get();
            if is_active {
                active.push(i);
            }
        }
        active
    }

    /// Synchronizes the visibility of the left and right bracket indicators ()
    /// so they only appear when multiple views are active at once.
    pub fn update_brackets(&self) {
        let show = self.get_active_indices().len() > 1;
        if self.left_bracket.is_visible() != show {
            self.left_bracket.set_visible(show);
        }
        if self.right_bracket.is_visible() != show {
            self.right_bracket.set_visible(show);
        }
    }
}
