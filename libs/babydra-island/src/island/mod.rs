//! Extensible Dynamic Island manager.
//!
//! The island is a notch capsule that displays exactly one *view* at a time.
//! Features register views either through the [`IslandFeature`] trait or
//! through the lightweight [`IslandView`] descriptor + [`IslandViewHandle`]
//! API. A single controller loop arbitrates between requested views by
//! priority (explicit overrides always win), animates transitions, and
//! dispatches hover / click events.
//!
//! Because the media player is effectively always visible while playing, the
//! override API ([`IslandViewHandle::override_show_for`]) lets temporary
//! overlays (volume, brightness, clipboard, timers, …) take over the capsule
//! for a fixed window and then hand control back to the player automatically.

pub mod view;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use gtk4::prelude::*;

pub use view::{IslandCtx, IslandFeature, IslandView, IslandViewHandle};

/// Size of the idle logo pill when `IslandConfig::idle_visible` is enabled.
const IDLE_SIZE: (i32, i32) = (28, 16);

thread_local! {
    static DEFAULT_ISLAND: RefCell<Option<Island>> = const { RefCell::new(None) };
}

/// Configuration for the island manager.
#[derive(Clone, Debug)]
pub struct IslandConfig {
    /// Show the idle logo pill when no view is active (default: hidden).
    pub idle_visible: bool,
    /// Controller loop interval in milliseconds.
    pub poll_interval_ms: u64,
    /// Expand animation duration in milliseconds.
    pub expand_ms: u64,
    /// Collapse animation duration in milliseconds.
    pub collapse_ms: u64,
}

impl Default for IslandConfig {
    fn default() -> Self {
        Self {
            idle_visible: false,
            poll_interval_ms: 150,
            expand_ms: 350,
            collapse_ms: 500,
        }
    }
}

/// What the island currently displays.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum IslandDisplay {
    Hidden,
    Idle,
    View(usize),
}

struct ViewRecord {
    id: String,
    priority: u8,
    size: Cell<(i32, i32)>,
    container: gtk4::Box,
    state: Rc<view::ViewState>,
    hover_keep: bool,
    capsule_class: Option<String>,
    feature: Option<Rc<RefCell<Box<dyn IslandFeature>>>>,
    on_show: Option<Rc<dyn Fn()>>,
    on_hide: Option<Rc<dyn Fn()>>,
    on_click: Option<Rc<dyn Fn()>>,
}

/// Everything needed to register a new view (built by the public APIs).
struct ViewSpec {
    id: String,
    priority: u8,
    size: (i32, i32),
    content: gtk4::Widget,
    hover_keep: bool,
    capsule_class: Option<String>,
    feature: Option<Rc<RefCell<Box<dyn IslandFeature>>>>,
    on_show: Option<Rc<dyn Fn()>>,
    on_hide: Option<Rc<dyn Fn()>>,
    on_click: Option<Rc<dyn Fn()>>,
}

struct IslandCore {
    cfg: IslandConfig,
    capsule: gtk4::Box,
    content_box: gtk4::Box,
    idle: Option<gtk4::Widget>,
    views: Vec<ViewRecord>,
    displayed: IslandDisplay,
    pending: Option<IslandDisplay>,
    animating: Cell<bool>,
    hovered: Cell<bool>,
}

/// The Dynamic Island manager.
///
/// Cheap to clone: every clone shares the same underlying state and controller
/// loop. The capsule widget is obtained via [`Island::capsule`] and appended
/// to the panel layout.
#[derive(Clone)]
pub struct Island {
    core: Rc<RefCell<IslandCore>>,
    source: Rc<RefCell<Option<glib::SourceId>>>,
}

impl Island {
    /// Returns a builder for a new island.
    pub fn builder() -> IslandBuilder {
        IslandBuilder::new()
    }

    /// Returns a clone of the notch capsule widget (append it to your layout).
    pub fn capsule(&self) -> gtk4::Box {
        self.core.borrow().capsule.clone()
    }

    /// Registers a descriptor-based view and returns its handle.
    pub fn register_view(&self, view: IslandView) -> IslandViewHandle {
        let IslandView {
            id,
            priority,
            size,
            content,
            hover_keep,
            capsule_class,
            on_show,
            on_hide,
            on_click,
        } = view;
        let widget = match content {
            view::IslandContent::Widget(w) => w,
            view::IslandContent::Builder(b) => b(),
        };
        self.register_view_inner(ViewSpec {
            id,
            priority,
            size,
            content: widget,
            hover_keep,
            capsule_class,
            feature: None,
            on_show: on_show.map(Rc::from),
            on_hide: on_hide.map(Rc::from),
            on_click: on_click.map(Rc::from),
        })
    }

    /// Registers a trait-based feature and returns its handle.
    pub fn register_feature(&self, mut feature: Box<dyn IslandFeature>) -> IslandViewHandle {
        let id = feature.id().to_string();
        let priority = feature.priority();
        let size = feature.size();
        let hover_keep = feature.hover_keep();
        let capsule_class = feature.capsule_class();
        let content = feature.build_view();
        let feature_rc = Rc::new(RefCell::new(feature));
        let handle = self.register_view_inner(ViewSpec {
            id,
            priority,
            size,
            content,
            hover_keep,
            capsule_class,
            feature: Some(feature_rc.clone()),
            on_show: None,
            on_hide: None,
            on_click: None,
        });
        {
            let mut feature = feature_rc.borrow_mut();
            feature.init(&handle);
            let ctx = IslandCtx {
                capsule: self.core.borrow().capsule.clone(),
                current: false,
                hovered: false,
            };
            feature.attach(&ctx);
        }
        handle
    }

    fn register_view_inner(&self, spec: ViewSpec) -> IslandViewHandle {
        let ViewSpec {
            id,
            priority,
            size,
            content,
            hover_keep,
            capsule_class,
            feature,
            on_show,
            on_hide,
            on_click,
        } = spec;
        let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        container.set_valign(gtk4::Align::Center);
        container.set_halign(gtk4::Align::Fill);
        container.set_hexpand(true);
        container.set_visible(false);
        container.append(&content);

        let state = Rc::new(view::ViewState::new(container.clone()));
        let mut core = self.core.borrow_mut();
        core.content_box.append(&container);
        core.views.push(ViewRecord {
            id: id.clone(),
            priority,
            size: Cell::new(size),
            container,
            state: state.clone(),
            hover_keep,
            capsule_class,
            feature,
            on_show,
            on_hide,
            on_click,
        });
        drop(core);
        IslandViewHandle { id, state }
    }

    /// Returns the handle of a registered view by id.
    pub fn get_handle(&self, id: &str) -> Option<IslandViewHandle> {
        let core = self.core.borrow();
        core.views
            .iter()
            .find(|v| v.id == id)
            .map(|v| IslandViewHandle {
                id: v.id.clone(),
                state: v.state.clone(),
            })
    }

    /// Returns handles of all registered views.
    pub fn handles(&self) -> Vec<IslandViewHandle> {
        let core = self.core.borrow();
        core.views
            .iter()
            .map(|v| IslandViewHandle {
                id: v.id.clone(),
                state: v.state.clone(),
            })
            .collect()
    }

    /// Convenience: requests a view by id.
    pub fn show(&self, id: &str) {
        if let Some(h) = self.get_handle(id) {
            h.show();
        }
    }

    /// Convenience: withdraws a view request by id.
    pub fn hide(&self, id: &str) {
        if let Some(h) = self.get_handle(id) {
            h.hide();
        }
    }

    /// Convenience: forces a view to be displayed, optionally auto-releasing
    /// after `duration` (control then returns to the previous winner).
    pub fn override_view(&self, id: &str, duration: Option<Duration>) {
        if let Some(h) = self.get_handle(id) {
            match duration {
                Some(d) => h.override_show_for(d),
                None => h.override_show(),
            }
        }
    }

    /// Stops the controller loop (used when rebuilding the panel).
    pub fn dispose(&self) {
        if let Some(source) = self.source.borrow_mut().take() {
            source.remove();
        }
    }
}

/// Builder that assembles an [`Island`] with its initial views and features.
pub struct IslandBuilder {
    cfg: IslandConfig,
    idle: Option<gtk4::Widget>,
    views: Vec<IslandView>,
    features: Vec<Box<dyn IslandFeature>>,
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

fn build_island(builder: IslandBuilder) -> Island {
    let cfg = builder.cfg;

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
    capsule.append(&content_box);

    if let Some(idle) = &builder.idle {
        idle.set_visible(cfg.idle_visible);
        content_box.append(idle);
    }

    let core = Rc::new(RefCell::new(IslandCore {
        cfg,
        capsule: capsule.clone(),
        content_box,
        idle: builder.idle,
        views: Vec::new(),
        displayed: IslandDisplay::Hidden,
        pending: None,
        animating: Cell::new(false),
        hovered: Cell::new(false),
    }));

    // Hover tracking on the capsule.
    let motion_enter = core.clone();
    let motion = gtk4::EventControllerMotion::new();
    motion.connect_enter(move |_, _, _| {
        motion_enter.borrow().hovered.set(true);
    });
    let motion_leave = core.clone();
    motion.connect_leave(move |_| {
        motion_leave.borrow().hovered.set(false);
    });
    capsule.add_controller(motion);

    // Click dispatch to the currently displayed view.
    let click_core = core.clone();
    let click = gtk4::GestureClick::new();
    click.set_button(0);
    click.connect_pressed(move |_, _, _, _| {
        let core = click_core.borrow();
        let idx = match core.displayed {
            IslandDisplay::View(i) => Some(i),
            _ => None,
        };
        let feature = idx.and_then(|i| core.views[i].feature.clone());
        let on_click = idx.and_then(|i| core.views[i].on_click.clone());
        drop(core);
        if let Some(f) = feature {
            f.borrow_mut().on_click();
        } else if let Some(cb) = on_click {
            cb();
        }
    });
    capsule.add_controller(click);

    let island = Island {
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

/// Returns the process-wide default island, if one has been built by
/// `crate::render::create_system_island` / `build_default_island`.
///
/// Useful for registering extra features (volume/brightness overlays, battery,
/// clipboard, timers, …) from anywhere in the process. Re-resolve after the
/// panel is rebuilt, because the previous island is disposed.
pub fn default_island() -> Option<Island> {
    DEFAULT_ISLAND.with(|d| d.borrow().clone())
}

fn set_default_island(island: &Island) {
    DEFAULT_ISLAND.with(|d| {
        if let Some(prev) = d.borrow_mut().take() {
            prev.dispose();
        }
        *d.borrow_mut() = Some(island.clone());
    });
}

/// One controller tick: timers → feature ticks → arbitration → transitions.
fn island_tick(core_rc: &Rc<RefCell<IslandCore>>) {
    let mut core = core_rc.borrow_mut();
    let now = Instant::now();

    // 1. Auto-hide / auto-release timers.
    for v in core.views.iter() {
        if let Some(deadline) = v.state.auto_hide_at.borrow().as_ref() {
            if now >= *deadline {
                v.state.requested.set(false);
                v.state.auto_hide_at.borrow_mut().take();
            }
        }
        if let Some(deadline) = v.state.release_at.borrow().as_ref() {
            if now >= *deadline {
                v.state.override_active.set(false);
                v.state.release_at.borrow_mut().take();
            }
        }
    }

    // 2. Feature ticks (every registered feature, every tick).
    let current_idx = match core.displayed {
        IslandDisplay::View(i) => Some(i),
        _ => None,
    };
    let mut ctxs = Vec::new();
    for (i, v) in core.views.iter().enumerate() {
        if let Some(f) = &v.feature {
            ctxs.push((
                f.clone(),
                IslandCtx {
                    capsule: core.capsule.clone(),
                    current: current_idx == Some(i),
                    hovered: core.hovered.get(),
                },
            ));
        }
    }
    for (f, ctx) in ctxs {
        f.borrow_mut().tick(&ctx);
    }

    // 3. Arbitration.
    let winner = select_winner(&core);
    let desired = match winner {
        Some(w) => IslandDisplay::View(w),
        None if core.cfg.idle_visible && core.idle.is_some() => IslandDisplay::Idle,
        None => IslandDisplay::Hidden,
    };

    // 4. Transitions (deferred while an animation is running).
    if core.animating.get() {
        if desired != core.displayed {
            core.pending = Some(desired);
        }
        return;
    }
    if let Some(pending) = core.pending.take() {
        if pending != core.displayed {
            apply_transition(&mut core, pending, core_rc);
            return;
        }
    }
    if desired != core.displayed {
        apply_transition(&mut core, desired, core_rc);
    }
}

/// Picks the view to display: active overrides first (most recently requested
/// wins ties), then highest priority (ties broken by most recent request;
/// equal-priority ties also fall back to registration order).
fn select_winner(core: &IslandCore) -> Option<usize> {
    let mut override_best: Option<(u64, usize)> = None;
    for (i, v) in core.views.iter().enumerate() {
        if !v.state.override_active.get() {
            continue;
        }
        let seq = v.state.request_seq.get();
        if override_best.map(|(s, _)| seq > s).unwrap_or(true) {
            override_best = Some((seq, i));
        }
    }
    if let Some((_, i)) = override_best {
        return Some(i);
    }

    let mut best: Option<(u8, u64, usize)> = None;
    for (i, v) in core.views.iter().enumerate() {
        let wanted =
            v.state.requested.get() || (v.hover_keep && v.state.active.get() && core.hovered.get());
        if !wanted {
            continue;
        }
        let key = (v.priority, v.state.request_seq.get(), i);
        if best.map(|b| key > b).unwrap_or(true) {
            best = Some(key);
        }
    }
    best.map(|(_, _, i)| i)
}

/// Applies a transition to the new display state, animating the capsule.
fn apply_transition(
    core: &mut IslandCore,
    desired: IslandDisplay,
    core_rc: &Rc<RefCell<IslandCore>>,
) {
    // Hide the previously displayed view.
    if let IslandDisplay::View(prev) = core.displayed {
        let v = &core.views[prev];
        v.state.active.set(false);
        v.container.set_visible(false);
        if let Some(cls) = &v.capsule_class {
            core.capsule.remove_css_class(cls);
        }
        if let Some(f) = &v.feature {
            f.borrow_mut().on_hide();
        }
        if let Some(cb) = &v.on_hide {
            cb();
        }
    }
    core.displayed = desired;

    match desired {
        IslandDisplay::View(w) => {
            let v = &core.views[w];
            v.state.active.set(true);
            v.container.set_visible(true);
            if let Some(cls) = &v.capsule_class {
                core.capsule.add_css_class(cls);
            }
            if let Some(f) = &v.feature {
                f.borrow_mut().on_show();
            }
            if let Some(cb) = &v.on_show {
                cb();
            }
            if let Some(idle) = &core.idle {
                idle.set_visible(false);
            }
            // Features may report a live size (e.g. the notification view
            // re-measures its height on every render).
            let size = v
                .feature
                .as_ref()
                .map(|f| f.borrow().size())
                .unwrap_or_else(|| v.size.get());
            animate_expand(core, size, true, core_rc);
        }
        IslandDisplay::Idle => {
            if let Some(idle) = &core.idle {
                idle.set_visible(true);
            }
            animate_expand(core, IDLE_SIZE, false, core_rc);
        }
        IslandDisplay::Hidden => {
            animate_collapse(core, core_rc);
        }
    }
}

/// Expands the capsule to `target` with a zoom/size animation.
fn animate_expand(
    core: &mut IslandCore,
    target: (i32, i32),
    active_music: bool,
    core_rc: &Rc<RefCell<IslandCore>>,
) {
    core.animating.set(true);
    let capsule = core.capsule.clone();
    let (tw, th) = target;
    let cur_w = capsule.width().max(0);
    let cur_h = capsule.height().max(0);
    let ms = core.cfg.expand_ms;
    let rc2 = core_rc.clone();

    capsule.set_visible(true);
    if active_music {
        capsule.add_css_class("active-music");
    }
    if cur_w <= 0 || cur_h <= 0 {
        babydra_ui_kit::ui::animation::island_zoom_in(capsule.upcast_ref(), tw, th, ms);
        glib::timeout_add_local_once(Duration::from_millis(ms + 60), move || {
            rc2.borrow_mut().animating.set(false);
        });
    } else {
        babydra_ui_kit::ui::animation::island_animate_size(
            capsule.upcast_ref(),
            cur_w,
            tw,
            cur_h,
            th,
            ms,
            move || {
                rc2.borrow_mut().animating.set(false);
            },
        );
        // Belt-and-braces: guarantee `animating` clears even if the capsule's
        // frame clock stalls mid-animation (e.g. during a panel rebuild).
        let rc3 = core_rc.clone();
        glib::timeout_add_local_once(Duration::from_millis(ms + 120), move || {
            rc3.borrow_mut().animating.set(false);
        });
    }
}

/// Collapses the capsule back to hidden.
fn animate_collapse(core: &mut IslandCore, core_rc: &Rc<RefCell<IslandCore>>) {
    core.animating.set(true);
    let capsule = core.capsule.clone();
    let cur_w = capsule.width().max(1);
    let ms = core.cfg.collapse_ms;
    let rc2 = core_rc.clone();

    babydra_ui_kit::ui::animation::island_zoom_out(capsule.upcast_ref(), cur_w, ms, true);
    glib::timeout_add_local_once(Duration::from_millis(ms + 60), move || {
        let core = rc2.borrow_mut();
        core.animating.set(false);
        core.capsule.remove_css_class("active-music");
        core.capsule.remove_css_class("notification-mode");
        core.capsule.set_visible(false);
    });
}
