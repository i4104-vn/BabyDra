use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct HoverPopoverRow {
    pub key: String,
    pub val: String,
    pub css_class: Option<String>,
}

impl HoverPopoverRow {
    pub fn new(key: &str, val: &str, css_class: Option<&str>) -> Self {
        Self {
            key: key.to_string(),
            val: val.to_string(),
            css_class: css_class.map(|s| s.to_string()),
        }
    }
}

/// Build hover popover card.
pub fn build_hover_card(title: &str, rows: Vec<HoverPopoverRow>) -> gtk4::Box {
    let card = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    card.add_css_class("status-popover-card");
    card.set_margin_top(4);
    card.set_margin_bottom(4);
    card.set_margin_start(6);
    card.set_margin_end(6);

    let title_lbl = gtk4::Label::new(Some(title));
    title_lbl.add_css_class("status-popover-header");
    title_lbl.set_halign(gtk4::Align::Start);
    card.append(&title_lbl);

    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.add_css_class("status-popover-sep");
    card.append(&sep);

    for row in rows {
        let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        row_box.add_css_class("status-popover-row");

        let key_lbl = gtk4::Label::new(Some(&row.key));
        key_lbl.add_css_class("status-popover-key");
        key_lbl.set_halign(gtk4::Align::Start);
        key_lbl.set_hexpand(true);

        let val_lbl = gtk4::Label::new(Some(&row.val));
        val_lbl.add_css_class("status-popover-val");
        if let Some(ref cls) = row.css_class {
            val_lbl.add_css_class(cls);
        }
        val_lbl.set_halign(gtk4::Align::End);

        row_box.append(&key_lbl);
        row_box.append(&val_lbl);
        card.append(&row_box);
    }

    card
}

/// Attach hover popover.
pub fn attach_hover_popover(
    anchor_widget: &impl IsA<gtk4::Widget>,
    popover: &gtk4::Popover,
    update_fn: Rc<dyn Fn()>,
) {
    popover.set_autohide(false);

    let is_hovered = Rc::new(RefCell::new(false));

    // Motion controller on the anchor icon
    let motion_icon = gtk4::EventControllerMotion::new();

    let is_hovered_icon_enter = is_hovered.clone();
    let popover_enter = popover.clone();
    let update_fn_enter = update_fn.clone();
    motion_icon.connect_enter(move |_, _, _| {
        *is_hovered_icon_enter.borrow_mut() = true;
        update_fn_enter();
        popover_enter.popup();
    });

    let is_hovered_icon_leave = is_hovered.clone();
    let popover_leave = popover.clone();
    motion_icon.connect_leave(move |_| {
        *is_hovered_icon_leave.borrow_mut() = false;
        let is_h = is_hovered_icon_leave.clone();
        let pop = popover_leave.clone();
        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
            if !*is_h.borrow() {
                pop.popdown();
            }
            gtk4::glib::ControlFlow::Break
        });
    });
    anchor_widget.add_controller(motion_icon);

    // Motion controller on Popover content to keep open while mouse is inside popover card
    let motion_popover = gtk4::EventControllerMotion::new();
    let is_hovered_pop_enter = is_hovered.clone();
    motion_popover.connect_enter(move |_, _, _| {
        *is_hovered_pop_enter.borrow_mut() = true;
    });

    let is_hovered_pop_leave = is_hovered.clone();
    let popover_pop_leave = popover.clone();
    motion_popover.connect_leave(move |_| {
        *is_hovered_pop_leave.borrow_mut() = false;
        let is_h = is_hovered_pop_leave.clone();
        let pop = popover_pop_leave.clone();
        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
            if !*is_h.borrow() {
                pop.popdown();
            }
            gtk4::glib::ControlFlow::Break
        });
    });
    popover.add_controller(motion_popover);
}
