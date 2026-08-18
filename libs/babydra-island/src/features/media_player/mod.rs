//! Media player island feature.
//!
//! Polls playerctl in the background, keeps the compact notch player view
//! requested while a player is active, and renders the media control popover.
//!
//! ## Cấu trúc module (chuẩn feature)
//!
//! | File | Trách nhiệm |
//! | :--- | :--- |
//! | `mod.rs` | Struct + constructor + `IslandFeature` impl (vòng đời + tick) |
//! | `view.rs` | Xây dựng cây widget (`PlayerWidgets::build`) |
//! | `render.rs` | Đẩy dữ liệu metadata vào widget (`update_player_view`) |
//! | `poll.rs` | Service nền: polling playerctl + cache |
//! | `art.rs` | Tải artwork, retry + fallback |
//! | `popover.rs`, `visualizer.rs`, `format.rs` | Helper riêng của feature |

mod art;
mod format;
mod poll;
mod popover;
mod render;
mod view;
mod visualizer;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk4::prelude::*;

use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use view::PlayerWidgets;

pub const PRIORITY: u8 = 50;
const TARGET_WIDTH: i32 = 200;
const TARGET_HEIGHT: i32 = 28;

/// Media player island feature: requests the compact player view while an
/// MPRIS player is active and keeps the popover in sync.
pub struct MediaPlayerFeature {
    handle: Option<IslandViewHandle>,
    widgets: PlayerWidgets,
    popover: RefCell<Option<popover::MediaPopover>>,
    latest_metadata: Rc<RefCell<Option<String>>>,
    poll_counter: Cell<u32>,
    last_meta_key: RefCell<String>,
    art_loaded_for_current_song: Rc<Cell<bool>>,
    last_attempted_url: Rc<RefCell<String>>,
    fail_count: Rc<Cell<u32>>,
    art_sender: tokio::sync::mpsc::UnboundedSender<art::ArtPayload>,
    art_receiver: Option<tokio::sync::mpsc::UnboundedReceiver<art::ArtPayload>>,
    is_playing: Rc<Cell<bool>>,
}

impl MediaPlayerFeature {
    pub fn new() -> Self {
        let (widgets, is_playing) = PlayerWidgets::build();
        let latest_metadata = poll::spawn_playerctl_polling();
        let (art_sender, art_receiver) = tokio::sync::mpsc::unbounded_channel::<art::ArtPayload>();

        Self {
            handle: None,
            widgets,
            popover: RefCell::new(None),
            latest_metadata,
            poll_counter: Cell::new(0),
            last_meta_key: RefCell::new(String::new()),
            art_loaded_for_current_song: Rc::new(Cell::new(false)),
            last_attempted_url: Rc::new(RefCell::new(String::new())),
            fail_count: Rc::new(Cell::new(0)),
            art_sender,
            art_receiver: Some(art_receiver),
            is_playing,
        }
    }

    /// One tick: parse the cached metadata, request show/hide and refresh UI.
    fn refresh(&mut self, ctx: &IslandCtx) {
        let metadata = self.latest_metadata.borrow().clone();

        let (meta, player_active) = metadata
            .as_deref()
            .map(render::parse_metadata)
            .unwrap_or_default();

        if let Some(h) = &self.handle {
            if player_active {
                h.show();
            } else {
                h.hide();
            }
        }

        self.is_playing.set(player_active && meta.playing);

        if player_active && ctx.is_current() {
            self.update_player_view(&meta);
        }
    }
}

impl Default for MediaPlayerFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl IslandFeature for MediaPlayerFeature {
    fn id(&self) -> &str {
        "media_player"
    }

    fn priority(&self) -> u8 {
        PRIORITY
    }

    fn size(&self) -> (i32, i32) {
        (TARGET_WIDTH, TARGET_HEIGHT)
    }

    fn build_view(&mut self) -> gtk4::Widget {
        self.widgets.music_view.clone().upcast()
    }

    fn init(&mut self, handle: &IslandViewHandle) {
        self.handle = Some(handle.clone());
    }

    fn attach(&mut self, ctx: &IslandCtx) {
        // Build the media control popover (needs the capsule) and start the
        // artwork receiver now that the art containers exist.
        let popover = popover::MediaPopover::new(&ctx.capsule());
        let art_container = self.widgets.art_container.clone();
        let popover_art = popover.art_container.clone();
        let last_attempted_url = self.last_attempted_url.clone();
        let art_loaded = self.art_loaded_for_current_song.clone();
        let fail_count = self.fail_count.clone();
        if let Some(rx) = self.art_receiver.take() {
            art::spawn_art_receiver(
                rx,
                art_container,
                popover_art,
                last_attempted_url,
                art_loaded,
                fail_count,
            );
        }
        self.popover.replace(Some(popover));
    }

    fn on_hide(&mut self) {
        if let Some(popover) = self.popover.borrow().as_ref() {
            popover.popdown();
        }
    }

    fn on_click(&mut self) {
        if let Some(popover) = self.popover.borrow().as_ref() {
            popover.toggle();
        }
    }

    fn tick(&mut self, ctx: &IslandCtx) {
        self.refresh(ctx);
    }
}
