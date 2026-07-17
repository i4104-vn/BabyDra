use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::path::PathBuf;
use gtk4::prelude::*;
use babydra_common::{SessionState, ActivePane};

/// Configures split pane layout toggling, creating the right pane content view and sync callbacks dynamically.
pub fn setup_split_view(
    split_paned: gtk4::Paned,
    is_split: Rc<Cell<bool>>,
    right_scroll_cell: Rc<RefCell<Option<gtk4::Box>>>,
    right_content_handle: Rc<RefCell<Option<Rc<crate::widgets::content_view::ContentViewHandle>>>>,
    session: Rc<RefCell<SessionState>>,
    active_pane: Rc<Cell<ActivePane>>,
    navigate_pane_ref: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>,
    info_widgets: Rc<crate::widgets::info_panel::InfoPanelWidgets>,
    left_content_scroll: gtk4::Box,
) -> Rc<dyn Fn()> {
    let split_paned_c = split_paned.clone();
    let is_split_c = is_split.clone();
    let right_scroll_c = right_scroll_cell.clone();
    let right_handle_c = right_content_handle.clone();
    let session_c = session.clone();
    let active_pane_c = active_pane.clone();
    let nav_c = navigate_pane_ref.clone();
    let info_widgets_c = info_widgets.clone();
    let left_scroll_c = left_content_scroll.clone();

    let toggle_split_view = move || {
        if is_split_c.get() {
            split_paned_c.set_end_child(None::<&gtk4::Widget>);
            right_handle_c.replace(None);
            right_scroll_c.replace(None);
            is_split_c.set(false);
            active_pane_c.set(ActivePane::Left);
        } else {
            let current_p = session_c.borrow().active_tab().current_path.clone();
            let (tx_right, mut rx_right) = tokio::sync::mpsc::unbounded_channel::<PathBuf>();
            let right_nav_cb = move |path| {
                let _ = tx_right.send(path);
            };

            let nav_c_clone = nav_c.clone();
            let session_cc = session_c.clone();
            glib::MainContext::default().spawn_local(async move {
                while let Some(path) = rx_right.recv().await {
                    {
                        let mut s = session_cc.borrow_mut();
                        s.active_tab_mut().navigate_to(path.clone());
                    }
                    if let Some(ref f) = *nav_c_clone.borrow() {
                        f(ActivePane::Right, path);
                    }
                }
            });

            let info_panel_c = info_widgets_c.clone();
            let active_c = active_pane_c.clone();
            let left_scroll_cc = left_scroll_c.clone();
            let right_scroll_cc = right_scroll_c.clone();

            let (right_scroll, right_handle) = crate::widgets::content_view::create_content_view(
                right_nav_cb,
                move |sel| {
                    active_c.set(ActivePane::Right);
                    left_scroll_cc.remove_css_class("active-pane");
                    if let Some(ref rs) = *right_scroll_cc.borrow() {
                        rs.add_css_class("active-pane");
                    }
                    crate::widgets::info_panel::update_info_panel(&info_panel_c, &sel);
                }
            );

            split_paned_c.set_end_child(Some(&right_scroll));
            split_paned_c.set_position(390);

            right_scroll_c.borrow_mut().replace(right_scroll);
            right_handle_c.replace(Some(Rc::new(right_handle)));
            is_split_c.set(true);

            active_pane_c.set(ActivePane::Right);
            if let Some(ref f) = *nav_c.borrow() {
                f(ActivePane::Right, current_p);
            }
        }
    };

    Rc::new(toggle_split_view)
}
