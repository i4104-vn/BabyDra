//! Search filter logic for the Wi-Fi network list.

use gtk4::prelude::*;

fn widget_contains_text(w: &gtk4::Widget, query: &str) -> bool {
    if let Some(lbl) = w.downcast_ref::<gtk4::Label>() {
        if lbl.text().to_lowercase().contains(query) {
            return true;
        }
    }
    let mut child = w.first_child();
    while let Some(c) = child {
        if widget_contains_text(&c, query) {
            return true;
        }
        child = c.next_sibling();
    }
    false
}

/// Filters list box based on search query, keeping matching items and their section headers visible.
pub fn filter_wifi_list(list_box: &gtk4::ListBox, query: &str) {
    let query_lower = query.to_lowercase();
    let mut child = list_box.first_child();
    let mut current_header: Option<gtk4::Widget> = None;
    let mut current_header_has_match = false;

    while let Some(c) = child {
        if c.has_css_class("wifi-category-header-row") {
            if let Some(prev_h) = current_header.take() {
                prev_h.set_visible(query_lower.is_empty() || current_header_has_match);
            }
            current_header = Some(c.clone());
            current_header_has_match = false;
        } else if c.is::<gtk4::ListBoxRow>() {
            let visible = query_lower.is_empty() || widget_contains_text(&c, &query_lower);
            c.set_visible(visible);
            if visible {
                current_header_has_match = true;
            }
        }
        child = c.next_sibling();
    }

    if let Some(prev_h) = current_header {
        prev_h.set_visible(query_lower.is_empty() || current_header_has_match);
    }
}
