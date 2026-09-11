# Hướng dẫn sử dụng Dynamic Island (babydra-island)

## Tổng quan

Dynamic Island là một **notch capsule** hiển thị **một view tại một thời điểm**. Hệ thống quản lý arbitration (quyết định hiển thị view nào) dựa trên:

1. **Override** (ưu tiên cao nhất) - `override_show_for()`: volume, brightness, clipboard popup
2. **Priority** - Feature có priority cao hơn thắng
3. **Request sequence** - Yêu cầu mới hơn thắng (tie-breaker)
4. **Hover keep** - Giữ view khi hover capsule

## Kiến trúc Island

```
Island (manager)
├── IslandBuilder          # Xây dựng island với config, idle, views, features
├── IslandConfig           # Cấu hình: idle_visible, poll_interval, expand/collapse ms
├── IslandView             # Descriptor-based view (widget + metadata)
├── IslandViewHandle       # Handle để show/hide/override view
├── IslandFeature (trait)  # Stateful feature (media player, notifications)
├── IslandCtx              # Context passed to feature callbacks
└── IslandDisplay (enum)   # Hidden | Idle | View(index)
```

## Cách 1: Descriptor + Handle (Lightweight Views)

Dùng cho views đơn giản, stateless: volume overlay, brightness, timer, clipboard popup.

### Tạo IslandView
```rust
use babydra_island::{IslandView, IslandViewHandle};

let view = IslandView::new("volume_overlay", build_volume_widget())
    .priority(80)                    // Cao hơn media player (50)
    .size(180, 32)                   // Target capsule size
    .hover_keep(true)                // Giữ khi hover
    .capsule_class("volume-mode")    // CSS class cho capsule
    .focus(false)                    // Không chiếm keyboard focus (mặc định: false)
    .on_show(|| log::info!("Volume shown"))
    .on_hide(|| log::info!("Volume hidden"))
    .on_click(|| toggle_mute());

// Hoặc lazy builder
let view = IslandView::with_builder("clipboard", || build_clipboard_popup())
    .priority(90)
    .size(300, 200);
```

### Đăng ký và sử dụng Handle
```rust
// Lấy island instance (từ panel/main)
let island = babydra_island::default_island().expect("Island not initialized");

// Đăng ký view, nhận handle
let handle: IslandViewHandle = island.register_view(view);

// Sử dụng handle
handle.show();                           // Request hiển thị
handle.show_for(Duration::from_secs(3)); // Auto-hide sau 3s
handle.override_show_for(Duration::from_millis(1500)); // Override tạm thời
handle.hide();                           // Withdraw request
handle.release_override();               // Kết thúc override

// Kiểm tra state
if handle.is_active() { ... }
if handle.is_requested() { ... }

// Thay đổi content runtime
handle.set_content(new_widget);
```

### Priority Guidelines

| View Type | Priority | Reason |
|-----------|----------|--------|
| Volume/Brightness overlay | 90-100 | System critical, short-lived |
| Clipboard popup | 85 | User-initiated, temporary |
| Timer/Stopwatch | 80 | User-initiated, may persist |
| Notifications | 70 | Important but not critical |
| Media Player | 50 | Default "always on" khi playing |
| Idle logo | 0 (special) | Chỉ khi idle_visible=true |

---

## Cách 2: IslandFeature Trait (Stateful Features)

Dùng cho features phức tạp, stateful, tự quản lý lifecycle: Media Player, Notifications, Clipboard history.

### Implement IslandFeature

```rust
use babydra_island::{IslandFeature, IslandViewHandle, IslandCtx};
use gtk4::prelude::*;

struct MyFeature {
    handle: Option<IslandViewHandle>,
    widgets: MyWidgets,
    state: Rc<RefCell<FeatureState>>,
    // Service receivers, timers, etc.
}

impl MyFeature {
    pub fn new() -> Self {
        let widgets = MyWidgets::build();
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(background_service(rx));
        
        Self {
            handle: None,
            widgets,
            state: Rc::new(RefCell::new(FeatureState::default())),
            // ...
        }
    }
}

impl IslandFeature for MyFeature {
    fn id(&self) -> &str { "my_feature" }
    
    fn priority(&self) -> u8 { 60 }
    
    fn size(&self) -> (i32, i32) { (220, 36) }
    
    fn hover_keep(&self) -> bool { true }
    
    fn capsule_class(&self) -> Option<String> { 
        Some("my-feature-mode".into()) 
    }
    
    fn focus(&self) -> bool { false } // true nếu cần chiếm keyboard (Exclusive), false để không ảnh hưởng app khác
    
    fn build_view(&mut self) -> gtk4::Widget {
        self.widgets.main_view.clone().upcast()
    }
    
    fn init(&mut self, handle: &IslandViewHandle) {
        self.handle = Some(handle.clone());
    }
    
    fn attach(&mut self, ctx: &IslandCtx) {
        // Setup popovers, start receivers, cần capsule widget
        let popover = MyPopover::new(&ctx.capsule());
        self.popover = Some(popover);
        self.start_receiver();
    }
    
    fn on_show(&mut self) {
        // View vừa trở thành active
        self.refresh_ui();
    }
    
    fn on_hide(&mut self) {
        // View không còn active
        self.popover.as_ref().map(|p| p.popdown());
    }
    
    fn on_click(&mut self) {
        // Click vào capsule khi feature này active
        self.popover.as_ref().map(|p| p.toggle());
    }
    
    fn tick(&mut self, ctx: &IslandCtx) {
        // Được gọi mỗi poll_interval (mặc định 150ms)
        // - Poll service data
        // - Update handle.show()/hide() dựa trên state
        // - Refresh UI nếu is_current()
        self.refresh(ctx);
    }
}
```

### Feature Lifecycle

```
register_feature()
    │
    ├─► build_view()          # Tạo widget content
    │
    ├─► init(handle)          # Nhận IslandViewHandle
    │
    ├─► attach(ctx)           # Capsule ready, setup popovers, spawn receivers
    │
    ├─► [Controller Loop - mỗi 150ms]
    │       │
    │       ├─► tick(ctx)     # Poll data, update handle.show/hide, refresh UI
    │       │
    │       ├─► Arbitration   # Priority + override + hover → winner
    │       │
    │       └─► Transition    # Animate capsule size, swap views
    │
    ├─► on_show()             # Khi trở thành winner
    │
    ├─► on_hide()             # Khi không còn winner
    │
    └─► on_click()            # Click capsule khi active
```

### Ví dụ thực tế: Media Player Feature

Xem `libs/babydra-island/src/features/media_player/mod.rs`

Key points:
- `service/poll.rs`: Background thread polling `playerctl` metadata
- `service/art.rs`: Async artwork loading với retry/fallback
- `ui/view.rs`: `PlayerWidgets::build()` tạo compact view + popover
- `ui/render.rs`: `update_player_view()` binding metadata → widgets
- `tick()`: Parse metadata, `handle.show()/hide()`, update UI nếu current

---

## Cách 3: Mở rộng Island (Extending)

### Thêm Feature mới vào System Island

Chỉnh sửa `libs/babydra-island/src/render.rs`:

```rust
use crate::features::my_feature::MyFeature;

pub fn create_system_island() -> Island {
    Island::builder()
        .config(IslandConfig {
            idle_visible: true,
            poll_interval_ms: 150,
            expand_ms: 350,
            collapse_ms: 500,
        })
        .idle(build_idle_logo())  // Optional idle widget
        .feature(Box::new(MediaPlayerFeature::new()))
        .feature(Box::new(NotificationFeature::new()))
        .feature(Box::new(ClipboardFeature::new()))
        .feature(Box::new(MyFeature::new()))  // ← Thêm feature mới
        .build()
}
```

### Override tạm thời từ bất kỳ đâu

```rust
// Trong volume/brightness controller
fn on_volume_change(&self, level: u8) {
    if let Some(island) = babydra_island::default_island() {
        let handle = island.get_handle("volume_overlay")
            .or_else(|| {
                // Lazy register nếu chưa có
                let view = IslandView::new("volume_overlay", build_volume_widget(level))
                    .priority(95)
                    .size(180, 32);
                Some(island.register_view(view))
            });
        
        if let Some(h) = handle {
            h.override_show_for(Duration::from_millis(1500));
            // Update widget content nếu cần
            h.set_content(build_volume_widget(level));
        }
    }
}
```

### Tự build Island tùy chỉnh (cho testing/app riêng)

```rust
use babydra_island::{Island, IslandBuilder, IslandConfig, IslandView};

let island = Island::builder()
    .config(IslandConfig {
        idle_visible: false,
        poll_interval_ms: 100,
        expand_ms: 200,
        collapse_ms: 300,
    })
    .view(IslandView::new("custom", build_custom_widget())
        .priority(60)
        .size(250, 40))
    .feature(Box::new(MyFeature::new()))
    .build();

// Lấy capsule widget để add vào layout
let capsule = island.capsule();
panel_layout.append(&capsule);

// Cleanup khi rebuild panel
island.dispose();
```

---

## Animation & Styling

### CSS Classes

Capsule nhận các class động:
- `.panel-notch` - Base class
- `.active-music` - Khi media player active
- `.notification-mode` - Khi notification active
- `.volume-mode`, `.my-feature-mode` - Tuỳ `capsule_class()`

### Animation Helpers (từ `babydra_ui_kit`)

```rust
use babydra_ui_kit::ui::animation::island::*;

// Tự động dùng bởi island internal, nhưng có thể dùng cho custom:
island_zoom_in(capsule.upcast_ref(), target_w, target_h, duration_ms);
island_zoom_out(capsule.upcast_ref(), current_w, duration_ms, remove_after);
island_animate_size(capsule.upcast_ref(), cur_w, target_w, cur_h, target_h, duration_ms, on_complete);
island_animate_width(capsule.upcast_ref(), cur_w, target_w, duration_ms, on_complete);
```

---

## Best Practices

### 1. Priority Design
- Luôn set priority rõ ràng, không dựa vào default (50)
- System overlays (volume, brightness) > User popups > Notifications > Media player
- Test arbitration với multiple features active

### 2. Handle Management
- Clone handle freely (cheap, Rc-based)
- Store handle trong feature struct để dùng ở tick/click
- `get_handle(id)` để tìm handle đã đăng ký

### 3. State Synchronization
- `tick()` là nơi duy nhất quyết định `show()/hide()` dựa trên service state
- Không gọi `show()` từ UI callbacks trực tiếp (trừ user action như click)
- Dùng `override_show_for()` cho temporary system overlays

### 4. Resource Cleanup
- `attach()`: Start receivers, build popovers
- `on_hide()`: Popdown popovers, pause heavy operations
- `dispose()`: Island tự dọn dẹp controller loop, nhưng feature nên tự cleanup ở `on_hide()`

### 5. Testing Features
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feature_priority() {
        let feature = MyFeature::new();
        assert_eq!(feature.priority(), 60);
    }
    
    #[test]
    fn test_tick_updates_handle() {
        // Mock service, verify handle.show()/hide() calls
    }
}
```

---

## Troubleshooting

| Issue | Nguyên nhân | Fix |
|-------|-------------|-----|
| View không hiển thị | Priority quá thấp, bị override | Kiểm tra priority, dùng `override_show_for` |
| Capsule không animate | `animating` flag không clear | Check `animate_collapse/expand` callback |
| Feature tick không chạy | Chưa đăng ký qua `register_feature` | Đảm bảo add vào `IslandBuilder::feature()` |
| Popover không hiện | `attach()` chưa gọi hoặc capsule chưa ready | Đảm bảo `attach` dùng `ctx.capsule()` |
| Memory leak | Receiver không drop, popover không popdown | Cleanup ở `on_hide()`, drop receiver khi feature drop |

---

## API Reference (Quick)

```rust
// Island
Island::builder() → IslandBuilder
IslandBuilder::config(cfg) → Self
IslandBuilder::idle_visible(bool) → Self
IslandBuilder::idle(widget) → Self
IslandBuilder::view(IslandView) → Self
IslandBuilder::feature(Box<dyn IslandFeature>) → Self
IslandBuilder::build() → Island

Island::capsule() → gtk4::Box
Island::register_view(IslandView) → IslandViewHandle
Island::register_feature(Box<dyn IslandFeature>) → IslandViewHandle
Island::get_handle(&str) → Option<IslandViewHandle>
Island::show(&str), hide(&str), override_view(&str, Option<Duration>)
Island::dispose()

// IslandView
IslandView::new(id, widget) → Self
IslandView::with_builder(id, closure) → Self
.priority(u8), .size(w,h), .hover_keep(bool), .capsule_class(str)
.on_show(fn), .on_hide(fn), .on_click(fn)

// IslandViewHandle
.show(), .show_for(Duration), .hide()
.override_show(), .override_show_for(Duration), .release_override()
.is_active(), .is_requested(), .id()
.set_content(Widget)

// IslandFeature (trait)
fn id(&self) -> &str
fn priority(&self) -> u8 { 50 }
fn size(&self) -> (i32, i32) { (200, 30) }
fn hover_keep(&self) -> bool { false }
fn capsule_class(&self) -> Option<String> { None }
fn build_view(&mut self) -> gtk4::Widget
fn init(&mut self, handle: &IslandViewHandle) {}
fn attach(&mut self, ctx: &IslandCtx) {}
fn on_show(&mut self) {}
fn on_hide(&mut self) {}
fn on_click(&mut self) {}
fn tick(&mut self, ctx: &IslandCtx) {}
```