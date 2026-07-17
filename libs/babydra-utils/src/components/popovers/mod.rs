use gtk4::prelude::*;

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

/// Creates a Popover widget and directly sets its child content.
pub fn create_popover_with_content(
    parent: &impl IsA<gtk4::Widget>,
    position: gtk4::PositionType,
    css_class: &str,
    content: &impl IsA<gtk4::Widget>,
) -> gtk4::Popover {
    let popover = create_popover(parent, position, css_class);
    popover.set_child(Some(content));
    popover
}
