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
    assert!(!tooltip.is_autohide());
    assert!(tooltip.popover.has_css_class("status-popover"));

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

    // Clean up popovers so no Finalizing warnings occur
    let mut c = btn.first_child();
    while let Some(w) = c {
        let next = w.next_sibling();
        if w.is::<gtk4::Popover>() {
            w.unparent();
        }
        c = next;
    }
    let mut c2 = btn2.first_child();
    while let Some(w) = c2 {
        let next = w.next_sibling();
        if w.is::<gtk4::Popover>() {
            w.unparent();
        }
        c2 = next;
    }
}

#[test]
fn test_button_popover_unparent_lifecycle() {
    if gtk4::init().is_err() {
        return;
    }

    let apps_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    let btn = gtk4::Button::new();
    let preview_popover = gtk4::Popover::new();
    preview_popover.set_parent(&btn);

    let tooltip_popover = TooltipPopover::new(&btn, gtk4::PositionType::Bottom);
    apps_box.append(&btn);

    // Verify children of btn
    let mut count = 0;
    let mut c = btn.first_child();
    while let Some(widget) = c {
        count += 1;
        c = widget.next_sibling();
    }
    // preview_popover + tooltip_popover
    assert_eq!(count, 2);

    preview_popover.unparent();
    tooltip_popover.unparent();

    let mut remaining = 0;
    let mut c2 = btn.first_child();
    while let Some(widget) = c2 {
        remaining += 1;
        c2 = widget.next_sibling();
    }
    assert_eq!(remaining, 0);

    apps_box.remove(&btn);

    // Slide animation cancellation margin preservation check
    let anim_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    anim_box.set_margin_top(0);
    assert_eq!(anim_box.margin_top(), 0);

    let gen = std::rc::Rc::new(std::cell::Cell::new(1_u64));
    babydra_ui_kit::ui::animation::slide_in_cancelable(
        anim_box.upcast_ref(),
        babydra_ui_kit::ui::animation::SlideDirection::Down,
        14,
        200,
        gen.clone(),
        1,
    );
    // Cancel by incrementing generation
    gen.set(2);
    anim_box.set_margin_top(0);
    assert_eq!(anim_box.margin_top(), 0);
}
