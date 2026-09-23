//! Integration tests for Switcher Overlay layout and components.

use babydra_ui_kit::ui::overlay::create_overlay_window;
use gtk4::prelude::*;

#[test]
fn test_overlay_window_right_aligned_layout() {
    if gtk4::init().is_err() {
        return;
    }

    let app = gtk4::Application::new(Some("org.babydra.test.overlay"), Default::default());
    let comp = create_overlay_window(&app);

    // 1. Check window CSS
    assert!(comp.window.has_css_class("switcher-window"));

    // 2. Check overlay_box fills full screen
    assert_eq!(comp.overlay_box.halign(), gtk4::Align::Fill);
    assert_eq!(comp.overlay_box.valign(), gtk4::Align::Fill);
    assert!(comp.overlay_box.hexpands());
    assert!(comp.overlay_box.vexpands());

    // 3. Check switcher_wrapper is aligned to the right edge (Align::End)
    assert!(comp.switcher_wrapper.has_css_class("switcher-main-row"));
    assert_eq!(comp.switcher_wrapper.halign(), gtk4::Align::End);
    assert_eq!(comp.switcher_wrapper.valign(), gtk4::Align::Center);

    // 4. Check deck_container is aligned to the right edge
    assert!(comp.deck_container.has_css_class("switcher-deck-container"));
    assert_eq!(comp.deck_container.halign(), gtk4::Align::End);
    assert_eq!(comp.deck_container.valign(), gtk4::Align::Center);

    // 5. Check meta_bar is aligned to the end with left-aligned text inside
    assert!(comp.meta_bar.has_css_class("switcher-meta-bar"));
    assert_eq!(comp.meta_bar.halign(), gtk4::Align::End);
    assert_eq!(comp.meta_title.xalign(), 0.0);
    assert_eq!(comp.meta_subtitle.xalign(), 0.0);

    // 6. Check scrolled window has PolicyType::Never, PolicyType::External (hidden scrollbars)
    assert!(comp.scrolled.has_css_class("switcher-scrolled-window"));
    assert_eq!(
        comp.scrolled.policy(),
        (gtk4::PolicyType::Never, gtk4::PolicyType::External)
    );

    // 7. Check stationary focus_ring at the center
    assert!(comp.focus_ring.has_css_class("switcher-focus-ring"));
    assert!(!comp.focus_ring.can_target());
    assert_eq!(comp.focus_ring.halign(), gtk4::Align::Center);
    assert_eq!(comp.focus_ring.valign(), gtk4::Align::Center);

    // 8. Verify children hierarchy: switcher_wrapper contains meta_bar and deck_container
    let mut children = Vec::new();
    let mut curr = comp.switcher_wrapper.first_child();
    while let Some(c) = curr {
        curr = c.next_sibling();
        children.push(c);
    }
    assert_eq!(children.len(), 2);
    assert_eq!(children[0], comp.meta_bar.clone().upcast::<gtk4::Widget>());
    assert_eq!(children[1], comp.deck_container.clone().upcast::<gtk4::Widget>());

    assert_scrolled_window_adjustment();
}

fn assert_scrolled_window_adjustment() {
    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::External);
    scrolled.set_size_request(54, 302);

    let box_col = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    for _ in 0..10 {
        let btn = gtk4::Button::new();
        btn.set_size_request(54, 54);
        box_col.append(&btn);
    }
    scrolled.set_child(Some(&box_col));

    let win = gtk4::Window::new();
    win.set_child(Some(&scrolled));
    win.present();

    let ctx = gtk4::glib::MainContext::default();
    for _ in 0..10 {
        ctx.iteration(false);
    }

    let vadj = scrolled.vadjustment();
    println!(
        "ADJUSTMENT: value={}, lower={}, upper={}, page_size={}",
        vadj.value(),
        vadj.lower(),
        vadj.upper(),
        vadj.page_size()
    );

    vadj.set_value(62.0);
    println!("AFTER SET 62: value={}", vadj.value());
}
