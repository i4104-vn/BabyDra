//! Fluent builder for declaratively constructing and presenting context menus.

use super::items::*;
use gtk4::prelude::*;
use gtk4::{Box, Button, Orientation, Popover, PositionType};
use std::rc::Rc;

/// Fluent builder for constructing and presenting context menus declaratively.
pub struct ContextMenuBuilder {
    vbox: Box,
    popover: Popover,
    footer_box: Option<Box>,
}

impl ContextMenuBuilder {
    /// Starts building a context menu attached to `parent`.
    pub fn new(parent: &impl IsA<gtk4::Widget>) -> Self {
        let popover = Popover::builder()
            .has_arrow(false)
            .autohide(true)
            .build();
        popover.set_parent(parent.as_ref());
        popover.add_css_class("context-menu-popover");
        popover.add_css_class("explore-popover");
        popover.add_css_class("desktop-context-menu");

        let vbox = Box::new(Orientation::Vertical, 2);
        vbox.set_css_classes(&["context-menu-box"]);
        vbox.set_width_request(200);
        popover.set_child(Some(&vbox));

        Self {
            vbox,
            popover,
            footer_box: None,
        }
    }

    /// Positions the popover at exact pointer `(x, y)` coordinates.
    pub fn at_coords(self, x: f64, y: f64) -> Self {
        self.popover.set_has_arrow(false);
        let rect = gtk4::gdk::Rectangle::new(x as i32, y as i32, 1, 1);
        self.popover.set_pointing_to(Some(&rect));
        self
    }

    /// Anchors the popover relative to the parent widget with an arrow.
    pub fn relative_to(self, pos: PositionType) -> Self {
        self.popover.set_has_arrow(true);
        self.popover.set_position(pos);
        self
    }

    /// Sets a custom minimum width for the context menu.
    pub fn with_width(self, width: i32) -> Self {
        self.vbox.set_width_request(width);
        self
    }

    /// Adds a CSS class to the popover.
    pub fn with_css_class(self, class: &str) -> Self {
        self.popover.add_css_class(class);
        self
    }

    /// Appends a standard clickable item.
    pub fn item(self, label: &str, icon: &str, on_click: impl Fn() + 'static) -> Self {
        let btn = create_menu_item(label, icon);
        let pop = self.popover.clone();
        let cb = Rc::new(on_click);
        btn.connect_clicked(move |_| {
            pop.popdown();
            cb();
        });
        self.vbox.append(&btn);
        self
    }

    /// Appends a standard clickable item with sensitivity control.
    pub fn item_sensitive(
        self,
        label: &str,
        icon: &str,
        sensitive: bool,
        on_click: impl Fn() + 'static,
    ) -> Self {
        let btn = create_menu_item_sensitive(label, icon, sensitive);
        let pop = self.popover.clone();
        let cb = Rc::new(on_click);
        btn.connect_clicked(move |_| {
            pop.popdown();
            cb();
        });
        self.vbox.append(&btn);
        self
    }

    /// Appends an item with a keyboard shortcut hint.
    pub fn item_with_shortcut(
        self,
        label: &str,
        icon: &str,
        shortcut: &str,
        on_click: impl Fn() + 'static,
    ) -> Self {
        let btn = create_menu_item_with_shortcut(label, icon, shortcut);
        let pop = self.popover.clone();
        let cb = Rc::new(on_click);
        btn.connect_clicked(move |_| {
            pop.popdown();
            cb();
        });
        self.vbox.append(&btn);
        self
    }

    /// Appends a destructive/danger item (e.g. Delete).
    pub fn destructive_item(self, label: &str, icon: &str, on_click: impl Fn() + 'static) -> Self {
        let btn = create_menu_item_destructive(label, icon);
        let pop = self.popover.clone();
        let cb = Rc::new(on_click);
        btn.connect_clicked(move |_| {
            pop.popdown();
            cb();
        });
        self.vbox.append(&btn);
        self
    }

    /// Appends a destructive/danger item with sensitivity control.
    pub fn destructive_item_sensitive(
        self,
        label: &str,
        icon: &str,
        sensitive: bool,
        on_click: impl Fn() + 'static,
    ) -> Self {
        let btn = create_menu_item_destructive_sensitive(label, icon, sensitive);
        let pop = self.popover.clone();
        let cb = Rc::new(on_click);
        btn.connect_clicked(move |_| {
            pop.popdown();
            cb();
        });
        self.vbox.append(&btn);
        self
    }

    /// Appends a horizontal separator.
    pub fn separator(self) -> Self {
        self.vbox.append(&create_menu_separator());
        self
    }

    /// Appends a section group header.
    pub fn group_header(self, label: &str) -> Self {
        self.vbox.append(&create_menu_group_header(label));
        self
    }

    /// Appends an arbitrary custom widget inside the context menu.
    pub fn custom_widget(self, widget: &impl IsA<gtk4::Widget>) -> Self {
        self.vbox.append(widget);
        self
    }

    /// Appends a raw button widget directly into the context menu.
    pub fn raw_item(self, button: &Button) -> Self {
        self.vbox.append(button);
        self
    }

    /// Appends a quick action icon button to the footer row.
    pub fn footer_button(
        mut self,
        icon: &str,
        tooltip: &str,
        on_click: impl Fn() + 'static,
    ) -> Self {
        let footer_box = match &self.footer_box {
            Some(fb) => fb.clone(),
            None => {
                let (footer_container, fb) = create_footer_container();
                self.vbox.append(&create_menu_separator());
                self.vbox.append(&footer_container);
                self.footer_box = Some(fb.clone());
                fb
            }
        };

        let btn = create_footer_icon_button(icon, tooltip);
        let pop = self.popover.clone();
        let cb = Rc::new(on_click);
        btn.connect_clicked(move |_| {
            pop.popdown();
            cb();
        });
        footer_box.append(&btn);
        self
    }

    /// Appends a quick action icon button to the footer row with sensitivity control.
    pub fn footer_button_sensitive(
        mut self,
        icon: &str,
        tooltip: &str,
        sensitive: bool,
        on_click: impl Fn() + 'static,
    ) -> Self {
        let footer_box = match &self.footer_box {
            Some(fb) => fb.clone(),
            None => {
                let (footer_container, fb) = create_footer_container();
                self.vbox.append(&create_menu_separator());
                self.vbox.append(&footer_container);
                self.footer_box = Some(fb.clone());
                fb
            }
        };

        let btn = create_footer_icon_button(icon, tooltip);
        btn.set_sensitive(sensitive);
        let pop = self.popover.clone();
        let cb = Rc::new(on_click);
        btn.connect_clicked(move |_| {
            pop.popdown();
            cb();
        });
        footer_box.append(&btn);
        self
    }

    /// Returns a reference to the underlying Popover.
    pub fn popover(&self) -> &Popover {
        &self.popover
    }

    /// Returns a reference to the internal vertical Box container.
    pub fn container(&self) -> &Box {
        &self.vbox
    }

    /// Finishes building and displays the popover immediately.
    pub fn popup(self) -> Popover {
        self.popover.popup();
        self.popover
    }

    /// Builds and returns `(Popover, Box)` container without opening.
    pub fn build(self) -> (Popover, Box) {
        (self.popover, self.vbox)
    }
}
