//! Island view descriptors, handles, and the feature trait.
//!
//! Two complementary ways of adding a view to the island:
//!
//! * **Descriptor + handle** — build an [`IslandView`] describing a widget,
//!   priority and size, register it and drive it through the returned
//!   [`IslandViewHandle`] (`show` / `hide` / `override_show_for` …).
//! * **Trait** — implement [`IslandFeature`] for stateful, self-driving
//!   features (e.g. the media player or notifications).

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use gtk4::prelude::*;

/// Monotonic counter used to break priority ties ("most recently requested wins").
static REQUEST_SEQ: AtomicU64 = AtomicU64::new(0);

/// Per-view state shared between the manager and the handle.
pub(crate) struct ViewState {
    /// Whether the view has a pending display request.
    pub requested: Cell<bool>,
    /// Whether the view is currently forcing itself to be displayed.
    pub override_active: Cell<bool>,
    /// Whether the view is currently the one displayed by the island.
    pub active: Cell<bool>,
    /// Deadline for auto-hiding a `show_for` request.
    pub auto_hide_at: RefCell<Option<Instant>>,
    /// Deadline for auto-releasing an `override_show_for`.
    pub release_at: RefCell<Option<Instant>>,
    /// Sequence stamp of the latest `show` call (tie-breaker).
    pub request_seq: Cell<u64>,
    /// Container widget holding the view content.
    pub container: gtk4::Box,
}

impl ViewState {
    pub(crate) fn new(container: gtk4::Box) -> Self {
        Self {
            requested: Cell::new(false),
            override_active: Cell::new(false),
            active: Cell::new(false),
            auto_hide_at: RefCell::new(None),
            release_at: RefCell::new(None),
            request_seq: Cell::new(0),
            container,
        }
    }
}

/// Programmatic handle to a registered island view.
///
/// Handles are cheap to clone and fully decoupled from the manager internals,
/// so they can be safely stored inside features and called from controller
/// callbacks without deadlocks.
#[derive(Clone)]
pub struct IslandViewHandle {
    pub(crate) id: String,
    pub(crate) state: Rc<ViewState>,
}

impl IslandViewHandle {
    /// Returns the view identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Requests the view to be displayed (subject to priority arbitration).
    ///
    /// Repeated calls while already requested are no-ops, so a pending
    /// `show_for` / `override_show_for` auto-hide deadline is preserved.
    pub fn show(&self) {
        if !self.state.requested.get() {
            self.state
                .request_seq
                .set(REQUEST_SEQ.fetch_add(1, Ordering::Relaxed));
            self.state.requested.set(true);
            self.state.auto_hide_at.borrow_mut().take();
        }
    }

    /// Requests the view to be displayed, then auto-hides it after `duration`.
    pub fn show_for(&self, duration: Duration) {
        self.show();
        self.state
            .auto_hide_at
            .borrow_mut()
            .replace(Instant::now() + duration);
    }

    /// Withdraws the display request.
    pub fn hide(&self) {
        self.state.requested.set(false);
        self.state.auto_hide_at.borrow_mut().take();
    }

    /// Forces the view to be displayed immediately, ignoring priority, until
    /// [`Self::release_override`] is called.
    ///
    /// This is the escape hatch for the "always visible" media player: a view
    /// can take over the capsule regardless of priority.
    pub fn override_show(&self) {
        self.show();
        self.state.override_active.set(true);
    }

    /// Forces the view to be displayed for `duration`, then automatically
    /// releases the override and withdraws the request. Control returns to
    /// whatever view was active before (typically the media player).
    pub fn override_show_for(&self, duration: Duration) {
        self.override_show();
        let deadline = Instant::now() + duration;
        self.state.release_at.borrow_mut().replace(deadline);
        self.state.auto_hide_at.borrow_mut().replace(deadline);
    }

    /// Ends an active override. The request flag is left untouched, so the
    /// view keeps participating in normal priority arbitration afterwards.
    pub fn release_override(&self) {
        self.state.override_active.set(false);
        self.state.release_at.borrow_mut().take();
    }

    /// Whether the view currently has a pending display request.
    pub fn is_requested(&self) -> bool {
        self.state.requested.get()
    }

    /// Whether the view is currently the one displayed by the island.
    pub fn is_active(&self) -> bool {
        self.state.active.get()
    }

    /// Swaps the content widget of the view at runtime.
    pub fn set_content(&self, widget: gtk4::Widget) {
        let container = self.state.container.clone();
        if let Some(child) = container.first_child() {
            container.remove(&child);
        }
        container.append(&widget);
    }
}

/// Content of a view: either an already-built widget or a lazy builder.
pub(crate) enum IslandContent {
    /// A pre-built content widget.
    Widget(gtk4::Widget),
    /// A closure that builds the content widget at registration time.
    Builder(Box<dyn FnOnce() -> gtk4::Widget>),
}

/// Static description of a view that can occupy the island capsule.
pub struct IslandView {
    pub(crate) id: String,
    pub(crate) priority: u8,
    pub(crate) size: (i32, i32),
    pub(crate) content: IslandContent,
    pub(crate) hover_keep: bool,
    pub(crate) capsule_class: Option<String>,
    pub(crate) on_show: Option<Box<dyn Fn()>>,
    pub(crate) on_hide: Option<Box<dyn Fn()>>,
    pub(crate) on_click: Option<Box<dyn Fn()>>,
}

impl IslandView {
    /// Creates a view with the given id and content widget.
    pub fn new(id: impl Into<String>, content: gtk4::Widget) -> Self {
        Self {
            id: id.into(),
            priority: 50,
            size: (200, 30),
            content: IslandContent::Widget(content),
            hover_keep: false,
            capsule_class: None,
            on_show: None,
            on_hide: None,
            on_click: None,
        }
    }

    /// Creates a view whose content widget is built lazily at registration.
    pub fn with_builder(
        id: impl Into<String>,
        build: impl FnOnce() -> gtk4::Widget + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            priority: 50,
            size: (200, 30),
            content: IslandContent::Builder(Box::new(build)),
            hover_keep: false,
            capsule_class: None,
            on_show: None,
            on_hide: None,
            on_click: None,
        }
    }

    /// Sets the arbitration priority (higher wins).
    pub fn priority(mut self, p: u8) -> Self {
        self.priority = p;
        self
    }

    /// Sets the target capsule size while this view is displayed.
    pub fn size(mut self, w: i32, h: i32) -> Self {
        self.size = (w, h);
        self
    }

    /// Keeps the view displayed while the pointer hovers the capsule.
    pub fn hover_keep(mut self, b: bool) -> Self {
        self.hover_keep = b;
        self
    }

    /// Extra CSS class applied to the capsule while this view is displayed.
    pub fn capsule_class(mut self, c: impl Into<String>) -> Self {
        self.capsule_class = Some(c.into());
        self
    }

    /// Callback invoked when the view becomes the displayed one.
    pub fn on_show(mut self, f: impl Fn() + 'static) -> Self {
        self.on_show = Some(Box::new(f));
        self
    }

    /// Callback invoked when the view stops being displayed.
    pub fn on_hide(mut self, f: impl Fn() + 'static) -> Self {
        self.on_hide = Some(Box::new(f));
        self
    }

    /// Callback invoked when the capsule is clicked while this view is displayed.
    pub fn on_click(mut self, f: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

/// Context handed to feature callbacks.
#[derive(Clone)]
pub struct IslandCtx {
    pub(crate) capsule: gtk4::Box,
    pub(crate) current: bool,
    pub(crate) hovered: bool,
}

impl IslandCtx {
    /// Returns a clone of the notch capsule widget.
    pub fn capsule(&self) -> gtk4::Box {
        self.capsule.clone()
    }

    /// Whether the calling feature's view is currently displayed.
    pub fn is_current(&self) -> bool {
        self.current
    }

    /// Whether the pointer is currently hovering the capsule.
    pub fn is_hovered(&self) -> bool {
        self.hovered
    }
}

/// Trait for complex, stateful island features (e.g. the media player).
pub trait IslandFeature {
    /// Unique view identifier.
    fn id(&self) -> &str;

    /// Arbitration priority (higher wins).
    fn priority(&self) -> u8 {
        50
    }

    /// Target capsule size while this feature's view is displayed.
    fn size(&self) -> (i32, i32) {
        (200, 30)
    }

    /// Keep the view displayed while the pointer hovers the capsule.
    fn hover_keep(&self) -> bool {
        false
    }

    /// Extra CSS class applied to the capsule while this view is displayed.
    fn capsule_class(&self) -> Option<String> {
        None
    }

    /// Builds the content widget for this feature's view.
    fn build_view(&mut self) -> gtk4::Widget;

    /// Called once at registration, giving the feature its handle.
    fn init(&mut self, _handle: &IslandViewHandle) {}

    /// Called once at registration, exposing the capsule (e.g. for popovers).
    fn attach(&mut self, _ctx: &IslandCtx) {}

    /// Called when the view becomes the displayed one.
    fn on_show(&mut self) {}

    /// Called when the view stops being displayed.
    fn on_hide(&mut self) {}

    /// Called when the capsule is clicked while this view is displayed.
    fn on_click(&mut self) {}

    /// Periodic callback invoked for every registered feature at each poll tick.
    fn tick(&mut self, _ctx: &IslandCtx) {}
}
