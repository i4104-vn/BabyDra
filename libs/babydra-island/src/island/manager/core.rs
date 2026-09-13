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
    pub user_selected: Cell<Option<(usize, u64)>>,
    pub last_scroll: Cell<Option<Instant>>,
}

impl IslandCore {
    /// Checks whether a registered view is currently alive and unexpired.
    /// Fully trait-driven and decoupled: no hardcoded feature names or if-else by ID!
    pub fn is_view_alive(&self, idx: usize, now: Instant) -> bool {
        if idx >= self.views.len() {
            return false;
        }
        let v = &self.views[idx];

        // 1. Purge expired deadlines first
        let has_active_flag = v.state.purge_if_expired(now);
        if !has_active_flag {
            return false;
        }

        // 2. If it has an active, unexpired timeout, it is alive
        if v.state.has_active_timeout(now) {
            return true;
        }

        // 3. If timeout has expired (or was never set), check if the feature is actively alive
        // (e.g. popover open, media playing, active notification).
        let feature_alive = v
            .feature
            .as_ref()
            .and_then(|f| f.try_borrow().ok().map(|feat| feat.is_alive()))
            .unwrap_or(false);

        if feature_alive {
            return true;
        }

        // Neither timeout nor persistent feature state is alive -> wipe from queue
        v.state.deactivate();
        false
    }

    /// Purges all expired timeouts across all registered views.
    pub fn purge_expired(&self, now: Instant) {
        for i in 0..self.views.len() {
            let _ = self.is_view_alive(i, now);
        }

        // Clean up user_selected if its view is no longer alive
        if let Some((sel_idx, _)) = self.user_selected.get() {
            if !self.is_view_alive(sel_idx, now) {
                self.user_selected.set(None);
            }
        }
    }

    /// Returns the indices of all currently active views whose timeout is valid or still alive.
    pub fn get_active_indices(&self) -> Vec<usize> {
        let now = Instant::now();
        self.purge_expired(now);

        let mut active = Vec::new();
        for i in 0..self.views.len() {
            if self.is_view_alive(i, now) {
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
