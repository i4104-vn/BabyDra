//! Media player island feature.
//!
//! Polls playerctl in the background, keeps the compact notch player view
//! requested while a player is active, and renders the media control popover.
//!
//! ## Cấu trúc module (chuẩn feature)
//!
//! | Thư mục / File | Trách nhiệm |
//! | :--- | :--- |
//! | `mod.rs` | Struct + constructor + `IslandFeature` impl (vòng đời + tick) |
//! | `ui/view.rs` | Xây dựng cây widget (`PlayerWidgets::build`) |
//! | `ui/visualizer.rs` | Thanh hiệu ứng sóng nhạc chuyển động |
//! | `ui/popover.rs` | Popover điều khiển media player |
//! | `ui/render.rs` | Đẩy dữ liệu metadata vào widget (`update_player_view`) |
//! | `service/poll.rs` | Service nền: polling playerctl + cache |
//! | `service/art.rs` | Tải artwork, retry + fallback |
//! | `service/format.rs` | Helper định dạng thời gian và icon app |

pub mod service;
pub mod ui;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Instant;

use gtk4::prelude::*;

use crate::island::view::{CAPSULE_HEIGHT, PLAYER_CAPSULE_WIDTH};
use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use service::{art, get_player_icon_name, poll};
use ui::{render, MediaPopover, PlayerWidgets};

pub const PRIORITY: u8 = 50;

/// Media player island feature: requests the compact player view while an
/// MPRIS player is active and keeps the popover in sync.
pub struct MediaPlayerFeature {
    handle: Option<IslandViewHandle>,
    widgets: PlayerWidgets,
    popover: RefCell<Option<MediaPopover>>,
    latest_metadata: Rc<RefCell<Option<String>>>,
    last_metadata: Option<String>,
    cached_meta: render::PlayerMeta,
    cached_player_active: bool,
    last_song_key: String,
    player_icon_name: String,
    view_ready: Cell<bool>,
    art_loaded_for_current_song: Rc<Cell<bool>>,
    last_attempted_url: Rc<RefCell<String>>,
    fail_count: Rc<Cell<u32>>,
    art_request_pending: Rc<Cell<bool>>,
    next_art_retry_at: Rc<Cell<Option<Instant>>>,
    play_icon_state: Cell<Option<bool>>,
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
            last_metadata: None,
            cached_meta: render::PlayerMeta::default(),
            cached_player_active: false,
            last_song_key: String::new(),
            player_icon_name: "music".to_string(),
            view_ready: Cell::new(false),
            art_loaded_for_current_song: Rc::new(Cell::new(false)),
            last_attempted_url: Rc::new(RefCell::new(String::new())),
            fail_count: Rc::new(Cell::new(0)),
            art_request_pending: Rc::new(Cell::new(false)),
            next_art_retry_at: Rc::new(Cell::new(None)),
            play_icon_state: Cell::new(None),
            art_sender,
            art_receiver: Some(art_receiver),
            is_playing,
        }
    }

    /// One tick: parse the cached metadata, request show/hide and refresh UI.
    fn refresh(&mut self, ctx: &IslandCtx) {
        let metadata_changed = {
            let latest = self.latest_metadata.borrow();
            latest.as_deref() != self.last_metadata.as_deref()
        };
        let mut song_changed = false;

        if metadata_changed {
            self.last_metadata = self.latest_metadata.borrow().clone();
            let (meta, player_active) = self
                .last_metadata
                .as_deref()
                .map(render::parse_metadata)
                .unwrap_or_default();
            self.cached_meta = meta;
            self.cached_player_active = player_active;

            let song_key = format!(
                "{}|{}|{}|{}",
                self.cached_meta.title,
                self.cached_meta.artist,
                self.cached_meta.player_name_raw,
                self.cached_meta.art_url
            );
            song_changed = song_key != self.last_song_key;
            if song_changed {
                self.last_song_key = song_key;
                self.player_icon_name = get_player_icon_name(&self.cached_meta.player_name_raw);
                self.art_loaded_for_current_song.set(false);
                self.art_request_pending.set(false);
                self.next_art_retry_at.set(None);
                self.fail_count.set(0);
                self.last_attempted_url.borrow_mut().clear();
            }
        }

        let meta = &self.cached_meta;
        let player_active = self.cached_player_active;

        let is_playing = player_active && meta.playing;
        let popover_open = self
            .popover
            .borrow()
            .as_ref()
            .map(|p| p.is_visible())
            .unwrap_or(false);

        let should_show = is_playing || popover_open || (player_active && ctx.is_current());
        if let Some(h) = &self.handle {
            if should_show {
                h.show();
            } else {
                h.hide();
            }
        }

        self.is_playing.set(is_playing);

        let art_retry_due = !self.art_loaded_for_current_song.get()
            && !self.art_request_pending.get()
            && self
                .next_art_retry_at
                .get()
                .map(|deadline| Instant::now() >= deadline)
                .unwrap_or(false);
        if should_show
            && ctx.is_current()
            && (metadata_changed || !self.view_ready.get() || art_retry_due)
        {
            self.update_player_view(meta, song_changed);
            self.view_ready.set(true);
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
        (PLAYER_CAPSULE_WIDTH, CAPSULE_HEIGHT)
    }

    fn is_alive(&self) -> bool {
        self.is_playing.get()
            || self
                .popover
                .borrow()
                .as_ref()
                .map(|p| p.is_visible())
                .unwrap_or(false)
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
        let popover = MediaPopover::new(&ctx.capsule());
        let art_container = self.widgets.art_container.clone();
        let popover_art = popover.art_container.clone();
        let last_attempted_url = self.last_attempted_url.clone();
        let art_loaded = self.art_loaded_for_current_song.clone();
        let fail_count = self.fail_count.clone();
        let request_pending = self.art_request_pending.clone();
        let next_retry_at = self.next_art_retry_at.clone();
        if let Some(rx) = self.art_receiver.take() {
            art::spawn_art_receiver(
                rx,
                art_container,
                popover_art,
                last_attempted_url,
                art_loaded,
                fail_count,
                request_pending,
                next_retry_at,
            );
        }

        self.popover.replace(Some(popover));
    }

    fn on_hide(&mut self) {
        self.view_ready.set(false);
        if let Some(popover) = self.popover.borrow().as_ref() {
            popover.popdown();
        }
    }

    fn on_click(&mut self) {
        if let Some(popover) = self.popover.borrow().as_ref() {
            popover.toggle();
        }
    }

    fn open_badge(&mut self) {
        if let Some(popover) = self.popover.borrow().as_ref() {
            if !popover.is_visible() {
                popover.popup();
            }
        }
    }

    fn tick(&mut self, ctx: &IslandCtx) {
        self.refresh(ctx);
    }
}
