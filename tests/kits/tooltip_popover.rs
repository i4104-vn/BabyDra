use babydra_ui_kit::components::popovers::{TooltipPopover, TooltipRow};
use gtk4::prelude::*;

#[test]
fn test_tooltip_row_creation() {
    let row = TooltipRow::new("Status", "Connected", Some("text-success"));
    assert_eq!(row.key, "Status");
    assert_eq!(row.val, "Connected");
    assert_eq!(row.css_class.as_deref(), Some("text-success"));
}

#[test]
fn test_parse_tooltip_rows() {
    let raw = "Left-click: Open Launcher\nRight-click: Show Desktop\nSimple Note";
    let rows = TooltipPopover::parse_rows(raw);
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].key, "Left-click");
    assert_eq!(rows[0].val, "Open Launcher");
    assert_eq!(rows[1].key, "Right-click");
    assert_eq!(rows[1].val, "Show Desktop");
    assert_eq!(rows[2].key, "Simple Note");
    assert_eq!(rows[2].val, "");
}

#[test]
fn test_gtk_tooltip_popover_suite() {
    if gtk4::init().is_err() {
        return;
    }

    let rows = vec![
        TooltipRow::new("Download", "↓ 10 MB/s", None),
        TooltipRow::new("Upload", "↑ 2 MB/s", None),
    ];
    let card = TooltipPopover::build_card("Network Status", &rows);
    assert!(card.has_css_class("status-popover-card"));
    assert!(card.has_css_class("tooltip-popover-card"));

    let btn = gtk4::Button::new();
    let tooltip = TooltipPopover::attach_card_text(
        &btn,
        "BabyDra",
        "Left-click: Open Launcher\nRight-click: Show Desktop",
    );

    assert_eq!(tooltip.position(), gtk4::PositionType::Bottom);
    assert!(tooltip.has_css_class("status-popover"));
    assert!(tooltip.has_css_class("tooltip-popover"));
    assert_eq!(tooltip.is_autohide(), false);
    assert_eq!(tooltip.popover.has_css_class("status-popover"), true);

    let btn2 = gtk4::Button::new();
    let tooltip2 = TooltipPopover::attach_card_text(&btn2, "Title", "Text");
    assert!(!tooltip2.is_suppressed());

    let is_open = std::rc::Rc::new(std::cell::Cell::new(false));
    let is_open_c = is_open.clone();
    tooltip2.set_suppress_fn(move || is_open_c.get());

    assert!(!tooltip2.is_suppressed());
    is_open.set(true);
    assert!(tooltip2.is_suppressed());
    is_open.set(false);
    assert!(!tooltip2.is_suppressed());
}
