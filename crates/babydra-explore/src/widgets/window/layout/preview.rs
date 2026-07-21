use std::rc::Rc;
use std::cell::{RefCell, Cell};
use gtk4::prelude::*;
use crate::widgets::status_bar::StatusBarWidgets;

/// Sets up the preview panel visibility toggling closure.
pub fn setup_preview_toggle(
    layout_paned: gtk4::Paned,
    revealer: gtk4::Revealer,
    preview_visible: Rc<Cell<bool>>,
    user_wants_preview: Rc<Cell<bool>>,
    status_bar_widgets_cell: Rc<RefCell<Option<StatusBarWidgets>>>,
) -> Rc<dyn Fn()> {
    let layout_paned = layout_paned.clone();
    let revealer_c = revealer.clone();
    let preview_visible = preview_visible;
    let user_wants_preview = user_wants_preview;
    let status_widgets_c = status_bar_widgets_cell;

    let toggle_preview = move || {
        let now_visible = !preview_visible.get();
        preview_visible.set(now_visible);
        user_wants_preview.set(now_visible);

        if now_visible {
            layout_paned.set_end_child(Some(&revealer_c));
            revealer_c.set_reveal_child(true);
        } else {
            revealer_c.set_reveal_child(false);
            let layout_paned_c = layout_paned.clone();
            let revealer_cc = revealer_c.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(250), move || {
                if !revealer_cc.reveals_child() {
                    layout_paned_c.set_end_child(None::<&gtk4::Widget>);
                }
            });
        }

        // Save updated settings
        {
            let mut current_settings = babydra_common::load_explore_settings();
            current_settings.preview_visible = now_visible;
            babydra_common::save_explore_settings(&current_settings);
        }

        if let Some(ref sw) = *status_widgets_c.borrow() {
            if now_visible {
                sw.btn_toggle_preview.add_css_class("status-bar-btn-active");
            } else {
                sw.btn_toggle_preview.remove_css_class("status-bar-btn-active");
            }
        }
    };
    Rc::new(toggle_preview)
}
