//! UI layout and drawing canvas handler for the screenshot crop/annotation editor window.

use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;

use babydra_common::models::{EditorState, Tool};
use babydra_common::services::screenshot::{trigger_save, trigger_copy};

use super::canvas::{draw_editor_canvas, setup_editor_gestures};
use super::color_popover::create_color_popover;

/// Sets up keyboard event controllers to handle global shortcuts like Escape (cancel),
/// Return (copy to clipboard), and Ctrl+S (save to file).
fn setup_editor_keys(
    window: &gtk4::ApplicationWindow,
    state: Rc<RefCell<EditorState>>,
) {
    let key_controller = gtk4::EventControllerKey::new();
    key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    
    let state_key = state.clone();
    let win_key = window.clone();
    key_controller.connect_key_pressed(move |_, key, _, state_flags| {
        match key {
            gtk4::gdk::Key::Escape => {
                win_key.close();
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Return => {
                if trigger_copy(&state_key.borrow(), &win_key) {
                    win_key.close();
                }
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::s | gtk4::gdk::Key::S => {
                if state_flags.contains(gtk4::gdk::ModifierType::CONTROL_MASK) {
                    if trigger_save(&state_key.borrow()) {
                        win_key.close();
                    }
                    gtk4::glib::Propagation::Stop
                } else {
                    gtk4::glib::Propagation::Proceed
                }
            }
            _ => gtk4::glib::Propagation::Proceed,
        }
    });

    window.add_controller(key_controller);
}

/// Constructs the screenshot editor window, maps its overlay design,
/// initializes the canvas, and builds the editing toolbars.
pub fn build_editor_ui(app: &gtk4::Application, temp_path: &str) -> gtk4::ApplicationWindow {
    let pixbuf = match gdk_pixbuf::Pixbuf::from_file(temp_path) {
        Ok(pb) => pb,
        Err(_) => return gtk4::ApplicationWindow::new(app),
    };

    let state = Rc::new(RefCell::new(EditorState::new(pixbuf)));

    let window = gtk4::ApplicationWindow::new(app);
    babydra_utils::ui::theme::apply_theme_class(&window);
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);

    // Stretch across the entire screen, ignoring panel exclusive zones
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_exclusive_zone(-1);
    window.add_css_class("screenshot-window");

    let overlay = gtk4::Overlay::new();
    window.set_child(Some(&overlay));

    // Drawing Canvas
    let drawing_area = gtk4::DrawingArea::new();
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);
    overlay.set_child(Some(&drawing_area));

    let state_draw = state.clone();
    drawing_area.set_draw_func(move |_, cr, width, height| {
        let s = state_draw.borrow();
        draw_editor_canvas(cr, &s, width as f64, height as f64);
    });

    // Floating macOS-style Glassmorphic Toolbar at the bottom-center
    let toolbar_wrapper = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    toolbar_wrapper.set_halign(gtk4::Align::Center);
    toolbar_wrapper.set_valign(gtk4::Align::End);
    toolbar_wrapper.set_margin_bottom(30);
    toolbar_wrapper.set_visible(false); // Hidden initially

    let toolbar = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    toolbar.add_css_class("screenshot-toolbar");
    toolbar.set_margin_start(16);
    toolbar.set_margin_end(16);
    toolbar.set_margin_top(8);
    toolbar.set_margin_bottom(8);

    // Tool buttons
    let btn_reset = gtk4::Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("refresh", 16))
        .build();
    btn_reset.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.reset_tooltip")));
    btn_reset.add_css_class("flat");
    btn_reset.add_css_class("screenshot-toolbar-btn");

    let btn_pen = gtk4::Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("edit", 16))
        .build();
    btn_pen.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.pen_tooltip")));
    btn_pen.add_css_class("flat");
    btn_pen.add_css_class("screenshot-toolbar-btn");

    let btn_rect = gtk4::Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("draw-rectangle-symbolic", 16))
        .build();
    btn_rect.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.rect_tooltip")));
    btn_rect.add_css_class("flat");
    btn_rect.add_css_class("screenshot-toolbar-btn");

    let btn_blur = gtk4::Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("view-conceal-symbolic", 16))
        .build();
    btn_blur.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.blur_tooltip")));
    btn_blur.add_css_class("flat");
    btn_blur.add_css_class("screenshot-toolbar-btn");

    let btn_eraser = gtk4::Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("broom", 16))
        .build();
    btn_eraser.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.eraser_tooltip")));
    btn_eraser.add_css_class("flat");
    btn_eraser.add_css_class("screenshot-toolbar-btn");

    let color_btn = gtk4::Button::new();
    color_btn.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.color_tooltip")));
    color_btn.add_css_class("flat");
    color_btn.add_css_class("screenshot-toolbar-btn");

    let color_dot = gtk4::DrawingArea::new();
    color_dot.set_size_request(16, 16);
    color_btn.set_child(Some(&color_dot));

    let state_indicator = state.clone();
    color_dot.set_draw_func(move |_, cr, w, h| {
        let (r, g, b) = state_indicator.borrow().current_color;
        let cx = w as f64 / 2.0;
        let cy = h as f64 / 2.0;
        let radius = (w.min(h) as f64 / 2.0) - 1.5;
        if radius <= 0.0 { return; }
        cr.arc(cx, cy, radius, 0.0, 2.0 * std::f64::consts::PI);
        cr.set_source_rgb(r, g, b);
        cr.fill_preserve().unwrap();
        cr.set_source_rgba(1.0, 1.0, 1.0, 0.6);
        cr.set_line_width(1.5);
        cr.stroke().unwrap();
    });

    let popover = create_color_popover(&color_btn, state.clone(), &color_dot);

    let popover_c = popover.clone();
    color_btn.connect_clicked(move |_| {
        popover_c.popup();
    });

    // Reset button click event
    let state_reset = state.clone();
    let toolbar_wrapper_reset = toolbar_wrapper.clone();
    let canvas_reset = drawing_area.clone();
    btn_reset.connect_clicked(move |_| {
        let mut s = state_reset.borrow_mut();
        s.has_selection = false;
        s.crop_x = 0.0;
        s.crop_y = 0.0;
        s.crop_w = 0.0;
        s.crop_h = 0.0;
        s.drawings.clear();
        s.active_stroke = None;
        s.active_rect = None;
        s.current_tool = Tool::Select;
        toolbar_wrapper_reset.set_visible(false);
        canvas_reset.queue_draw();
    });

    // Tool buttons click events
    let tools = vec![
        (btn_pen.clone(), Tool::Pen),
        (btn_rect.clone(), Tool::Rect),
        (btn_blur.clone(), Tool::Blur),
        (btn_eraser.clone(), Tool::Eraser),
    ];

    let tools_list = Rc::new(tools.clone());
    for (btn, tool) in tools {
        let state_tool = state.clone();
        let btn_clone = btn.clone();
        let tools_clone = tools_list.clone();
        btn.connect_clicked(move |_| {
            state_tool.borrow_mut().current_tool = tool;
            for (t_btn, _) in tools_clone.iter() {
                t_btn.remove_css_class("selected");
            }
            btn_clone.add_css_class("selected");
        });
    }

    // Action buttons
    let btn_copy = gtk4::Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("copy", 16))
        .build();
    btn_copy.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.copy_tooltip")));
    btn_copy.add_css_class("flat");
    btn_copy.add_css_class("screenshot-toolbar-btn");
    
    let state_copy = state.clone();
    let win_copy = window.clone();
    btn_copy.connect_clicked(move |_| {
        if trigger_copy(&state_copy.borrow(), &win_copy) {
            win_copy.close();
        }
    });

    let btn_save = gtk4::Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("download", 16))
        .build();
    btn_save.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.save_tooltip")));
    btn_save.add_css_class("flat");
    btn_save.add_css_class("screenshot-toolbar-btn");
    
    let state_save = state.clone();
    let win_save = window.clone();
    btn_save.connect_clicked(move |_| {
        if trigger_save(&state_save.borrow()) {
            win_save.close();
        }
    });

    let btn_cancel = gtk4::Button::builder()
        .child(&babydra_utils::ui::icon::get_icon("window-close-symbolic", 16))
        .build();
    btn_cancel.set_tooltip_text(Some(&babydra_common::i18n::t("screenshot.cancel_tooltip")));
    btn_cancel.add_css_class("flat");
    btn_cancel.add_css_class("screenshot-toolbar-btn");
    
    let win_cancel = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_cancel.close();
    });

    // Assemble toolbar
    toolbar.append(&btn_reset);
    
    let sep0 = gtk4::Label::new(Some("│"));
    sep0.add_css_class("capsule-separator");
    toolbar.append(&sep0);
    
    toolbar.append(&btn_pen);
    toolbar.append(&btn_rect);
    toolbar.append(&btn_blur);
    toolbar.append(&btn_eraser);
    
    let sep1 = gtk4::Label::new(Some("│"));
    sep1.add_css_class("capsule-separator");
    toolbar.append(&sep1);
    toolbar.append(&color_btn);

    let sep2 = gtk4::Label::new(Some("│"));
    sep2.add_css_class("capsule-separator");
    toolbar.append(&sep2);

    toolbar.append(&btn_copy);
    toolbar.append(&btn_save);
    toolbar.append(&btn_cancel);

    toolbar_wrapper.append(&toolbar);
    overlay.add_overlay(&toolbar_wrapper);

    // Mouse gestures setup
    setup_editor_gestures(&drawing_area, state.clone(), &toolbar_wrapper, &btn_pen);

    // Keyboard shortcuts setup
    setup_editor_keys(&window, state.clone());

    window
}
