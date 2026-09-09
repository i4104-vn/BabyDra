use gtk4::prelude::*;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

/// Creates a standardized Popover widget.
pub fn create_popover(
    parent: &impl IsA<gtk4::Widget>,
    position: gtk4::PositionType,
    css_class: &str,
) -> gtk4::Popover {
    let popover = gtk4::Popover::new();
    popover.set_parent(parent);
    popover.set_position(position);
    if !css_class.is_empty() {
        popover.add_css_class(css_class);
    }
    popover
}

/// A row in a structured tooltip card (Key -> Value).
#[derive(Debug, Clone)]
pub struct TooltipRow {
    pub key: String,
    pub val: String,
    pub css_class: Option<String>,
}

impl TooltipRow {
    pub fn new(key: &str, val: &str, css_class: Option<&str>) -> Self {
        Self {
            key: key.to_string(),
            val: val.to_string(),
            css_class: css_class.map(|s| s.to_string()),
        }
    }
}

#[derive(Clone)]
pub struct TooltipPopover {
    pub popover: gtk4::Popover,
    suppress_fn: Rc<RefCell<Option<Rc<dyn Fn() -> bool>>>>,
}

impl Deref for TooltipPopover {
    type Target = gtk4::Popover;

    fn deref(&self) -> &Self::Target {
        &self.popover
    }
}

impl TooltipPopover {
    /// Creates a new `TooltipPopover` anchored to the specified parent widget.
    pub fn new(parent: &impl IsA<gtk4::Widget>, position: gtk4::PositionType) -> Self {
        let popover = gtk4::Popover::new();
        popover.set_parent(parent);
        popover.set_position(position);
        popover.add_css_class("status-popover");
        popover.add_css_class("tooltip-popover");
        popover.set_autohide(false);
        Self {
            popover,
            suppress_fn: Rc::new(RefCell::new(None)),
        }
    }

    pub fn set_suppress_fn(&self, f: impl Fn() -> bool + 'static) {
        *self.suppress_fn.borrow_mut() = Some(Rc::new(f));
    }

    pub fn is_suppressed(&self) -> bool {
        if let Some(ref f) = *self.suppress_fn.borrow() {
            f()
        } else {
            false
        }
    }

    /// Builds a structured card widget with an optional title, separator, and key-value rows.
    pub fn build_card(title: &str, rows: &[TooltipRow]) -> gtk4::Box {
        let card = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
        card.add_css_class("status-popover-card");
        card.add_css_class("tooltip-popover-card");
        card.set_margin_top(4);
        card.set_margin_bottom(4);
        card.set_margin_start(6);
        card.set_margin_end(6);

        if !title.is_empty() {
            let title_lbl = gtk4::Label::new(Some(title));
            title_lbl.add_css_class("status-popover-header");
            title_lbl.add_css_class("tooltip-popover-header");
            title_lbl.set_halign(gtk4::Align::Start);
            card.append(&title_lbl);

            if !rows.is_empty() {
                let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
                sep.add_css_class("status-popover-sep");
                sep.add_css_class("tooltip-popover-sep");
                card.append(&sep);
            }
        }

        for row in rows {
            let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
            row_box.add_css_class("status-popover-row");
            row_box.add_css_class("tooltip-popover-row");

            let key_lbl = gtk4::Label::new(Some(&row.key));
            key_lbl.add_css_class("status-popover-key");
            key_lbl.add_css_class("tooltip-popover-key");
            key_lbl.set_halign(gtk4::Align::Start);

            if row.val.is_empty() {
                key_lbl.set_hexpand(true);
                row_box.append(&key_lbl);
            } else {
                key_lbl.set_hexpand(true);

                let val_lbl = gtk4::Label::new(Some(&row.val));
                val_lbl.add_css_class("status-popover-val");
                val_lbl.add_css_class("tooltip-popover-val");
                if let Some(ref cls) = row.css_class {
                    val_lbl.add_css_class(cls);
                }
                val_lbl.set_halign(gtk4::Align::End);

                row_box.append(&key_lbl);
                row_box.append(&val_lbl);
            }

            card.append(&row_box);
        }

        card
    }

    /// Parses a multi-line string with "Key: Value" lines into `TooltipRow` items.
    pub fn parse_rows(text: &str) -> Vec<TooltipRow> {
        text.lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    return None;
                }
                if let Some((k, v)) = trimmed.split_once(':') {
                    Some(TooltipRow::new(k.trim(), v.trim(), None))
                } else {
                    Some(TooltipRow::new(trimmed, "", None))
                }
            })
            .collect()
    }

    /// Attaches hover motion controllers to the anchor widget and popover content.
    pub fn attach_hover(
        &self,
        anchor: &impl IsA<gtk4::Widget>,
        update_fn: Option<Rc<dyn Fn()>>,
    ) {
        let is_hovered = Rc::new(RefCell::new(false));
        let suppress_fn = self.suppress_fn.clone();

        // 1. Motion controller on anchor widget
        let motion_anchor = gtk4::EventControllerMotion::new();
        let is_h_enter = is_hovered.clone();
        let pop_enter = self.popover.clone();
        let update_c = update_fn.clone();
        let sup_enter = suppress_fn.clone();
        motion_anchor.connect_enter(move |_, _, _| {
            *is_h_enter.borrow_mut() = true;
            if let Some(ref f) = *sup_enter.borrow() {
                if f() {
                    *is_h_enter.borrow_mut() = false;
                    if pop_enter.is_visible() {
                        pop_enter.popdown();
                    }
                    return;
                }
            }
            if let Some(ref update) = update_c {
                update();
            }
            pop_enter.popup();
        });

        let is_h_motion = is_hovered.clone();
        let pop_motion = self.popover.clone();
        let sup_motion = suppress_fn.clone();
        motion_anchor.connect_motion(move |_, _, _| {
            if let Some(ref f) = *sup_motion.borrow() {
                if f() {
                    *is_h_motion.borrow_mut() = false;
                    if pop_motion.is_visible() {
                        pop_motion.popdown();
                    }
                }
            }
        });

        let is_h_leave = is_hovered.clone();
        let pop_leave = self.popover.clone();
        motion_anchor.connect_leave(move |_| {
            *is_h_leave.borrow_mut() = false;
            let is_h = is_h_leave.clone();
            let pop = pop_leave.clone();
            gtk4::glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
                if !*is_h.borrow() {
                    pop.popdown();
                }
                gtk4::glib::ControlFlow::Break
            });
        });
        anchor.add_controller(motion_anchor);

        // 2. Motion controller on popover card
        let motion_pop = gtk4::EventControllerMotion::new();
        let is_h_pop_enter = is_hovered.clone();
        let pop_pop = self.popover.clone();
        let sup_pop = suppress_fn.clone();
        motion_pop.connect_enter(move |_, _, _| {
            if let Some(ref f) = *sup_pop.borrow() {
                if f() {
                    *is_h_pop_enter.borrow_mut() = false;
                    if pop_pop.is_visible() {
                        pop_pop.popdown();
                    }
                    return;
                }
            }
            *is_h_pop_enter.borrow_mut() = true;
        });

        let pop_pop_motion = self.popover.clone();
        let sup_pop_motion = suppress_fn.clone();
        motion_pop.connect_motion(move |_, _, _| {
            if let Some(ref f) = *sup_pop_motion.borrow() {
                if f() && pop_pop_motion.is_visible() {
                    pop_pop_motion.popdown();
                }
            }
        });

        let is_h_pop_leave = is_hovered.clone();
        let pop_pop_leave = self.popover.clone();
        motion_pop.connect_leave(move |_| {
            *is_h_pop_leave.borrow_mut() = false;
            let is_h = is_h_pop_leave.clone();
            let pop = pop_pop_leave.clone();
            gtk4::glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
                if !*is_h.borrow() {
                    pop.popdown();
                }
                gtk4::glib::ControlFlow::Break
            });
        });
        self.popover.add_controller(motion_pop);
    }

    /// Attaches a card tooltip where rows are automatically parsed from multi-line text.
    pub fn attach_card_text(
        anchor: &impl IsA<gtk4::Widget>,
        title: &str,
        text: &str,
    ) -> Self {
        let rows = Self::parse_rows(text);
        let card = Self::build_card(title, &rows);
        let tooltip = Self::new(anchor, gtk4::PositionType::Bottom);
        tooltip.popover.set_child(Some(&card));
        tooltip.attach_hover(anchor, None);
        tooltip
    }
}
