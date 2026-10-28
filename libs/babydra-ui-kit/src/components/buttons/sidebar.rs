//! Reusable sidebar dock helpers (dock icon button, hover popover tooltips).

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Attaches a smooth hover popover tooltip to a sidebar dock button.
pub fn attach_sidebar_popover(
    btn: &gtk4::Button,
    title: &str,
    active_popover: &Rc<RefCell<Option<gtk4::Popover>>>,
) -> (gtk4::Popover, gtk4::Label) {
    let popover = gtk4::Popover::new();
    popover.set_parent(btn);
    popover.set_position(gtk4::PositionType::Right);
    popover.set_has_arrow(true);
    popover.set_autohide(false);
    popover.set_can_target(false);
    popover.add_css_class("sidebar-item-popover");

    let lbl = gtk4::Label::new(Some(title));
    lbl.add_css_class("sidebar-item-popover-label");
    lbl.set_halign(gtk4::Align::Center);
    lbl.set_valign(gtk4::Align::Center);
    popover.set_child(Some(&lbl));

    let motion = gtk4::EventControllerMotion::new();
    {
        let p_enter = popover.clone();
        let active_pop_c = active_popover.clone();
        motion.connect_enter(move |_, _, _| {
            if let Some(prev) = active_pop_c.borrow_mut().take() {
                prev.popdown();
            }
            *active_pop_c.borrow_mut() = Some(p_enter.clone());
            p_enter.popup();
        });
    }
    {
        let p_leave = popover.clone();
        let active_pop_c = active_popover.clone();
        motion.connect_leave(move |_| {
            p_leave.popdown();
            let mut cur = active_pop_c.borrow_mut();
            if let Some(ref c) = *cur {
                if c == &p_leave {
                    *cur = None;
                }
            }
        });
    }
    btn.add_controller(motion);

    (popover, lbl)
}

/// Creates a standard 44x44 icon-only dock button matching Settings and Explore sidebar style.
pub fn create_sidebar_dock_btn(
    icon_widget: &impl IsA<gtk4::Widget>,
    title: &str,
    is_active: bool,
    active_popover: &Rc<RefCell<Option<gtk4::Popover>>>,
) -> (gtk4::Button, gtk4::Label) {
    let btn = gtk4::Button::new();
    btn.add_css_class("sidebar-item");
    btn.add_css_class("sidebar-icon-btn");
    btn.add_css_class("flat");
    btn.set_cursor_from_name(Some("pointer"));
    btn.set_halign(gtk4::Align::Center);
    btn.set_valign(gtk4::Align::Center);
    btn.set_size_request(44, 44);

    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    hbox.set_halign(gtk4::Align::Center);
    hbox.set_valign(gtk4::Align::Center);
    icon_widget.set_valign(gtk4::Align::Center);
    icon_widget.set_halign(gtk4::Align::Center);
    hbox.append(icon_widget);

    let hidden_lbl = gtk4::Label::new(Some(title));
    hidden_lbl.set_visible(false);
    hbox.append(&hidden_lbl);

    btn.set_child(Some(&hbox));

    if is_active {
        btn.add_css_class("active-nav");
    }

    let (_popover, lbl) = attach_sidebar_popover(&btn, title, active_popover);

    (btn, lbl)
}
