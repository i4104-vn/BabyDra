//! Builder that assembles the Dynamic Island widget tree and event controllers.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use gtk4::prelude::*;

use crate::island::controller::arbitration::island_tick;
use crate::island::controller::scroll::handle_island_scroll;
use crate::island::manager::core::IslandCore;
use crate::island::manager::{set_default_island, Island};
use crate::island::models::{IslandConfig, IslandDisplay};
use crate::island::view::{IslandFeature, IslandView};

/// Builder that assembles an [`Island`] with its initial views and features.
pub struct IslandBuilder {
    pub(crate) cfg: IslandConfig,
    pub(crate) idle: Option<gtk4::Widget>,
    pub(crate) views: Vec<IslandView>,
    pub(crate) features: Vec<Box<dyn IslandFeature>>,
}

impl IslandBuilder {
    pub fn new() -> Self {
        Self {
            cfg: IslandConfig::default(),
            idle: None,
            views: Vec::new(),
            features: Vec::new(),
        }
    }

    pub fn config(mut self, cfg: IslandConfig) -> Self {
        self.cfg = cfg;
        self
    }

    pub fn idle_visible(mut self, v: bool) -> Self {
        self.cfg.idle_visible = v;
        self
    }

    /// Sets the idle logo pill content shown when nothing else is active
    /// (only displayed when `idle_visible` is enabled).
    pub fn idle(mut self, widget: impl IsA<gtk4::Widget>) -> Self {
        self.idle = Some(widget.upcast());
        self
    }

    pub fn view(mut self, view: IslandView) -> Self {
        self.views.push(view);
        self
    }

    pub fn feature(mut self, f: Box<dyn IslandFeature>) -> Self {
        self.features.push(f);
        self
    }

    pub fn build(self) -> Island {
        build_island(self)
    }
}

impl Default for IslandBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn build_island(builder: IslandBuilder) -> Island {
    let cfg = builder.cfg;

    // Outer wrapper box containing the outside bracket indicators () and the notch capsule
    let root = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
    root.add_css_class("island-wrapper");
    root.set_valign(gtk4::Align::Start);
    root.set_halign(gtk4::Align::Center);

    // Left bracket indicator () sitting outside the capsule
    let left_bracket = gtk4::Label::new(Some("("));
    left_bracket.add_css_class("island-bracket");
    left_bracket.add_css_class("left");
    left_bracket.set_valign(gtk4::Align::Center);
    left_bracket.set_halign(gtk4::Align::Center);
    left_bracket.set_visible(false);

    // Dynamic Island notch capsule (the black pill)
    let capsule = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    capsule.add_css_class("panel-notch");
    capsule.set_valign(gtk4::Align::Start);
    capsule.set_halign(gtk4::Align::Center);
    capsule.set_visible(false);

    let content_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    content_box.add_css_class("notch-content");
    content_box.set_valign(gtk4::Align::Center);
    content_box.set_halign(gtk4::Align::Fill);
    content_box.set_hexpand(true);
    content_box.set_vexpand(true);
    capsule.append(&content_box);

    // Right bracket indicator () sitting outside the capsule
    let right_bracket = gtk4::Label::new(Some(")"));
    right_bracket.add_css_class("island-bracket");
    right_bracket.add_css_class("right");
    right_bracket.set_valign(gtk4::Align::Center);
    right_bracket.set_halign(gtk4::Align::Center);
    right_bracket.set_visible(false);

    root.append(&left_bracket);
    root.append(&capsule);
    root.append(&right_bracket);

    if let Some(idle) = &builder.idle {
        idle.set_visible(cfg.idle_visible);
        content_box.append(idle);
    }

    let core = Rc::new(RefCell::new(IslandCore {
        cfg,
        root: root.clone(),
        capsule: capsule.clone(),
        content_box,
        left_bracket,
        right_bracket,
        idle: builder.idle,
        views: Vec::new(),
        displayed: IslandDisplay::Hidden,
        pending: None,
        animating: Cell::new(false),
        hovered: Cell::new(false),
        user_selected: None,
        last_scroll: Cell::new(None),
    }));

    // Hover tracking on root and capsule
    let motion_enter = core.clone();
    let motion = gtk4::EventControllerMotion::new();
    motion.connect_enter(move |_, _, _| {
        motion_enter.borrow().hovered.set(true);
    });
    let motion_leave = core.clone();
    motion.connect_leave(move |_| {
        motion_leave.borrow().hovered.set(false);
    });
    root.add_controller(motion);

    // Scroll wheel cycle navigation among active island views.
    // Captured on root so it covers both outside brackets and inside capsule without duplication.
    let scroll_core = core.clone();
    let scroll = gtk4::EventControllerScroll::new(
        gtk4::EventControllerScrollFlags::VERTICAL
            | gtk4::EventControllerScrollFlags::HORIZONTAL
            | gtk4::EventControllerScrollFlags::DISCRETE,
    );
    scroll.set_propagation_phase(gtk4::PropagationPhase::Capture);
    scroll.connect_scroll(move |_, dx, dy| {
        handle_island_scroll(&scroll_core, dx, dy);
        gtk4::glib::Propagation::Stop
    });
    root.add_controller(scroll);

    // Click dispatch to the currently displayed view.
    let click_core = core.clone();
    let click = gtk4::GestureClick::new();
    click.set_button(gtk4::gdk::BUTTON_PRIMARY);
    click.connect_pressed(move |_, n_press, _, _| {
        if n_press > 1 {
            return;
        }
        let Ok(core) = click_core.try_borrow() else {
            return;
        };
        // Reset last_scroll to suppress any trailing scroll/kinetic events right after a click
        core.last_scroll.set(Some(std::time::Instant::now()));

        let idx = match core.displayed {
            IslandDisplay::View(i) => Some(i),
            _ => None,
        };
        let feature = idx.and_then(|i| core.views[i].feature.clone());
        let on_click = idx.and_then(|i| core.views[i].on_click.clone());
        drop(core);

        if let Some(i) = idx {
            if let Ok(mut c) = click_core.try_borrow_mut() {
                let next_seq = crate::island::view::next_request_seq();
                c.user_selected = Some((i, next_seq));
                c.views[i].state.request_seq.set(next_seq);
            }
        }

        if let Some(f) = feature {
            if let Ok(mut feat) = f.try_borrow_mut() {
                feat.on_click();
            }
        } else if let Some(cb) = on_click {
            cb();
        }
    });
    capsule.add_controller(click);

    let island = Island {
        root,
        capsule,
        core: core.clone(),
        source: Rc::new(RefCell::new(None)),
    };

    for view in builder.views {
        island.register_view(view);
    }
    for feature in builder.features {
        island.register_feature(feature);
    }

    // Controller loop.
    let loop_core = core.clone();
    let interval = Duration::from_millis(island.core.borrow().cfg.poll_interval_ms);
    let source = glib::timeout_add_local(interval, move || {
        island_tick(&loop_core);
        glib::ControlFlow::Continue
    });
    *island.source.borrow_mut() = Some(source);

    set_default_island(&island);
    island
}
