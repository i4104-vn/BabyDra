//! Manager topic module: Island handle, view registration, and process-wide singleton.

pub mod core;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use gtk4::prelude::*;

pub(crate) use self::core::IslandCore;

use crate::island::controller::island_tick;
use crate::island::models::{ViewRecord, ViewSpec};
use crate::island::ui::IslandBuilder;
use crate::island::view::{IslandCtx, IslandFeature, IslandView, IslandViewHandle, ViewState};

thread_local! {
    static DEFAULT_ISLAND: RefCell<Option<Island>> = const { RefCell::new(None) };
}

/// The Dynamic Island manager.
///
/// Cheap to clone: every clone shares the same underlying state and controller
/// loop. The capsule widget is obtained via [`Island::capsule`] (or the
/// outer wrapper with outside brackets via [`Island::widget`]).
#[derive(Clone)]
pub struct Island {
    pub(crate) root: gtk4::Box,
    pub(crate) capsule: gtk4::Box,
    pub(crate) core: Rc<RefCell<IslandCore>>,
    pub(crate) source: Rc<RefCell<Option<glib::SourceId>>>,
}

impl Island {
    /// Returns a builder for a new island.
    pub fn builder() -> IslandBuilder {
        IslandBuilder::new()
    }

    /// Returns the full island widget assembly (outer wrapper with outside brackets and capsule).
    pub fn widget(&self) -> gtk4::Box {
        self.root.clone()
    }

    /// Returns a clone of the notch capsule widget (anchoring popovers, size animations).
    ///
    /// Stored directly on `Island` to allow lock-free, panic-free widget access from any thread/context.
    pub fn capsule(&self) -> gtk4::Box {
        self.capsule.clone()
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
            focus,
            on_show,
            on_hide,
            on_click,
        } = view;
        let widget = match content {
            crate::island::view::IslandContent::Widget(w) => w,
            crate::island::view::IslandContent::Builder(b) => b(),
        };
        self.register_view_inner(ViewSpec {
            id,
            priority,
            size,
            content: widget,
            hover_keep,
            capsule_class,
            focus,
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
        let focus = feature.focus();
        let content = feature.build_view();
        let feature_rc = Rc::new(RefCell::new(feature));
        let handle = self.register_view_inner(ViewSpec {
            id,
            priority,
            size,
            content,
            hover_keep,
            capsule_class,
            focus,
            feature: Some(feature_rc.clone()),
            on_show: None,
            on_hide: None,
            on_click: None,
        });
        {
            let mut feature = feature_rc.borrow_mut();
            feature.init(&handle);
            let ctx = IslandCtx {
                capsule: self.capsule.clone(),
                current: false,
                hovered: false,
            };
            feature.attach(&ctx);
        }
        handle
    }

    pub(crate) fn register_view_inner(&self, spec: ViewSpec) -> IslandViewHandle {
        let ViewSpec {
            id,
            priority,
            size,
            content,
            hover_keep,
            capsule_class,
            focus,
            feature,
            on_show,
            on_hide,
            on_click,
        } = spec;
        let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        container.set_valign(gtk4::Align::Center);
        container.set_halign(gtk4::Align::Fill);
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_visible(false);
        container.append(&content);

        let state = Rc::new(ViewState::new(container.clone()));
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
            focus,
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
        let core = self.core.try_borrow().ok()?;
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
        let Ok(core) = self.core.try_borrow() else {
            return Vec::new();
        };
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

/// Returns the process-wide default island, if one has been built.
pub fn default_island() -> Option<Island> {
    DEFAULT_ISLAND.with(|d| d.try_borrow().ok().and_then(|opt| opt.clone()))
}

/// Triggers an immediate controller tick on the default island to process transitions without delay.
pub fn tick_default_island() {
    if let Some(island) = default_island() {
        island_tick(&island.core);
    }
}

/// Dismisses all popovers currently open on the default island capsule.
///
/// Safe: uses `island.capsule()` without borrowing `IslandCore`, completely preventing re-entrant panics.
pub fn dismiss_all_popovers() {
    if let Some(island) = default_island() {
        let capsule = island.capsule();
        let mut next = capsule.first_child();
        while let Some(child) = next {
            next = child.next_sibling();
            if let Some(popover) = child.downcast_ref::<gtk4::Popover>() {
                if popover.is_visible() {
                    popover.popdown();
                }
            }
        }
    }
}

pub(crate) fn set_default_island(island: &Island) {
    DEFAULT_ISLAND.with(|d| {
        if let Some(prev) = d.borrow_mut().take() {
            prev.dispose();
        }
        *d.borrow_mut() = Some(island.clone());
    });
}
