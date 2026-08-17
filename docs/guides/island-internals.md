# Island hoạt động như thế nào — Kiến trúc runtime & luồng

**Phạm vi:** Cách `babydra-island` vận hành bên trong: các thành phần runtime, controller loop, arbitration, transition, và luồng dữ liệu hiện tại của từng feature built-in.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

> [!NOTE]
> Tài liệu này mô tả **luồng hoạt động hiện tại của code** (`libs/babydra-island/src/island/`),
> không phải hướng dẫn sử dụng API. Muốn biết cách dùng → xem [island.md](./island.md).

---

## Mục lục

- [1. Các thành phần runtime](#1-các-thành-phần-runtime)
- [2. Luồng khởi tạo](#2-luồng-khởi-tạo)
- [3. Vòng đời một view / feature](#3-vòng-đời-một-view--feature)
- [4. Controller loop — 4 bước mỗi tick](#4-controller-loop--4-bước-mỗi-tick)
- [5. Arbitration — cách chọn view thắng](#5-arbitration--cách-chọn-view-thắng)
- [6. Transition & animation](#6-transition--animation)
- [7. Luồng media player](#7-luồng-media-player)
- [8. Luồng notification](#8-luồng-notification)
- [9. Luồng idle logo](#9-luồng-idle-logo)
- [10. Thread, channel & dọn dẹp](#10-thread-channel--dọn-dẹp)

---

## 1. Các thành phần runtime

Island là một **view stack**: tại mỗi thời điểm chỉ hiển thị **một view** trong capsule notch, chọn bởi controller loop.

| Thành phần | Vai trò |
| :--- | :--- |
| `Island` | Handle clone rẻ (chia sẻ `Rc<RefCell<IslandCore>>` + `source`). API công khai: `register_view`, `register_feature`, `get_handle`, `show/hide/override_view`, `capsule()`, `dispose()` |
| `IslandCore` | Toàn bộ trạng thái: config, capsule, danh sách `ViewRecord`, view đang hiển thị, cờ `animating`/`hovered`, `pending` |
| `ViewRecord` | Một view đã đăng ký: id, priority, size, container widget, `ViewState`, callbacks, feature ref |
| `ViewState` | Trạng thái per-view chia sẻ giữa manager và handle: `requested`, `override_active`, `active`, deadline `auto_hide_at`/`release_at`, `request_seq` |
| `IslandViewHandle` | Clone rẻ, chỉ giữ `id` + `Rc<ViewState>` — gọi từ callback/tick **không** lock manager |
| Controller loop | `glib::timeout_add_local(poll_interval)` gọi `island_tick` lặp vô hạn |

Sơ đồ:

```text
Island (clone rẻ)
   │
   ├── core: Rc<RefCell<IslandCore>>
   │          ├── capsule (gtk4::Box)  ── gắn vào panel
   │          ├── views: Vec<ViewRecord>
   │          │          └── state: Rc<ViewState>  ◄── IslandViewHandle giữ tham chiếu này
   │          ├── displayed / pending / animating / hovered
   │          └── cfg (poll_interval_ms, expand_ms, collapse_ms, idle_visible)
   │
   └── source: Rc<RefCell<Option<glib::SourceId>>>  ── controller loop (dispose() = remove)
```

---

## 2. Luồng khởi tạo

`build_default_island()` (trong `render.rs`) gọi `Island::builder()...build()`:

```text
Island::builder()
   .feature(NotificationFeature)      ── register_feature
   .feature(MediaPlayerFeature)       ── register_feature
   .idle(idle_logo_view())
   .build()
```

Trong `build_island()`:

1. Dựng cây widget: `capsule` (class `panel-notch`) → `content_box` (class `notch-content`).
2. Tạo `IslandCore`, gắn `EventControllerMotion` (theo dõi hover → `core.hovered`) và `GestureClick` (dispatch click tới view đang hiển thị).
3. `register_view`/`register_feature` từng view/feature trong builder.
4. Khởi động **controller loop**: `glib::timeout_add_local(poll_interval_ms=150ms, island_tick)`.
5. `set_default_island()` — đăng ký vào `DEFAULT_ISLAND` (thread_local); nếu đã có island cũ → `dispose()` island cũ trước.

> [!IMPORTANT]
> `default_island()` trả `Some` ngay sau bước 5 — mọi nơi trong process (panel, settings, feature khác)
> lấy được manager này để đăng ký thêm view/feature.

---

## 3. Vòng đời một view / feature

### 3.1. Descriptor view (`register_view`)

```text
register_view(IslandView) 
  → dựng container (Box, ẩn sẵn, append content)
  → tạo ViewState
  → append container vào content_box, push ViewRecord
  → trả IslandViewHandle { id, state }
```

### 3.2. Feature (`register_feature`)

```text
register_feature(Box<dyn IslandFeature>)
  → đọc id / priority / size / hover_keep / capsule_class / build_view()
  → register_view_inner (như trên, feature: Some(feature_rc))
  → feature.init(&handle)          ── feature lưu handle
  → feature.attach(&ctx)           ── feature nhận capsule, gắn popover, spawn receiver
```

### 3.3. Luồng runtime mỗi vòng đời

```text
[đăng ký] register → init → attach
     │
     ▼
[loop] tick(ctx) ── feature cập nhật trạng thái + gọi handle.show()/hide()
     │
     ▼
[arbitration] select_winner → nếu đổi view → apply_transition
     │
     ▼
[transition] on_hide(view cũ) → animate → on_show(view mới)
     │
     ▼
[tick tiếp] feature đọc ctx.is_current() → render nội dung thật
     │
     ▼
[thoát] island.dispose() → source.remove() → loop dừng; receiver drop → thread thoát
```

---

## 4. Controller loop — 4 bước mỗi tick

`island_tick(core)` chạy mỗi `poll_interval_ms` (150ms), theo đúng thứ tự:

### Bước 1 — Xử lý timer auto-hide / auto-release

Với mỗi view có deadline:

```text
nếu now >= auto_hide_at  → requested = false, xóa deadline   (hết show_for)
nếu now >= release_at    → override_active = false, xóa deadline  (hết override_show_for)
```

### Bước 2 — Feature ticks

Gom danh sách feature (kèm `IslandCtx { capsule, is_current, is_hovered }`) **trước**, rồi gọi `tick()` từng feature sau khi đã nhả borrow của core:

```rust
let ctxs = core.views.iter().filter_map(|v| v.feature.clone().map(|f| (f, ctx)));
for (f, ctx) in ctxs { f.borrow_mut().tick(&ctx); }
```

> [!IMPORTANT]
> Việc gom context trước rồi mới gọi tick là để tránh double-borrow — feature gọi
> `handle.show()/hide()` (chạm `ViewState`) an toàn, nhưng **không được** gọi
> `register_view`/`register_feature` (chạm `core`) từ trong `tick`.

### Bước 3 — Arbitration

`select_winner(&core)` chọn view thắng (chi tiết mục 5), rồi tính trạng thái mong muốn:

```rust
let desired = match winner {
    Some(w) => View(w),
    None if idle_visible && idle.is_some() => Idle,
    None => Hidden,
};
```

### Bước 4 — Transition (deferred khi đang animate)

```text
nếu animating:
    nếu desired != displayed → lưu vào pending (chờ)
    return
nếu có pending và khác displayed → apply_transition(pending)
nếu desired != displayed      → apply_transition(desired)
```

> [!TIP]
> `pending` là cơ chế chống kẹt: nếu người dùng đổi ý giữa lúc animation chạy,
> mong muốn mới được ghi nhận và thực thi ngay khi animation kết thúc.

---

## 5. Arbitration — cách chọn view thắng

Thuật toán trong `select_winner`, 2 vòng:

### Vòng 1 — Override thắng tuyệt đối

```text
chỉ xét view có override_active = true
chọn view có request_seq lớn nhất (yêu cầu gần đây nhất)
nếu có override → trả về ngay (bỏ qua priority)
```

### Vòng 2 — Priority thường

```text
view "muốn hiển thị" khi: requested = true
  HOẶC (hover_keep && active && core.hovered)   ← đang hiển thị + hover giữ

chọn key (priority, request_seq, index) lớn nhất:
  1. priority cao nhất
  2. hòa → request_seq lớn nhất (yêu cầu gần đây nhất thắng)
  3. vẫn hòa → index (thứ tự đăng ký)
```

### Bảng priority mặc định hiện tại

| View | Priority | Ghi chú |
| :--- | :--- | :--- |
| `notification` | `90` | Thắng player, tự ẩn sau 5s (kéo dài khi hover) |
| `media_player` | `50` | Hiển thị khi có player đang chạy (luôn request khi Playing/Paused) |
| `default` (idle) | — | Logo nhỏ chỉ khi `idle_visible` |

> [!IMPORTANT]
> Vì player **luôn giữ requested** khi đang chạy, overlay tạm thời (volume, clipboard…)
> phải dùng `override_show_for(duration)` để chiếm chỗ rồi tự trả lại — priority thường
> (dưới 90) không thắng được player đang request.

---

## 6. Transition & animation

`apply_transition(core, desired)` — thứ tự bắt buộc:

### 6.1. Ẩn view cũ

```text
nếu đang hiển thị View(prev):
    prev.active = false
    prev.container.set_visible(false)
    bỏ capsule_class của prev (nếu có)
    feature.on_hide() / on_click callback on_hide
```

### 6.2. Hiển thị view mới

```text
View(w):
    w.active = true; container visible; thêm capsule_class
    feature.on_show() / on_show callback
    ẩn idle logo
    size = feature.size() (đọc LẠI mỗi transition — notification co giãn theo nội dung)
          hoặc size tĩnh của descriptor
    animate_expand(size)
Idle:    hiện idle logo → animate_expand(IDLE_SIZE=(28,16))
Hidden:  animate_collapse
```

### 6.3. Animation

| Hàm | Khi nào | Cơ chế |
| :--- | :--- | :--- |
| `animate_expand` | Chuyển sang View/Idle | Nếu capsule đang 0×0 → `island_zoom_in`; ngược lại `island_animate_size` (từ kích thước hiện tại → kích thước mới). Đặt `animating = true`; reset sau `expand_ms` (+60ms) hoặc qua `on_complete` + timeout dự phòng `expand_ms+120ms` |
| `animate_collapse` | Chuyển sang Hidden | `island_zoom_out`; sau `collapse_ms` reset `animating`, bỏ class `active-music`/`notification-mode`, ẩn capsule |

> [!NOTE]
> Có **2 cơ chế reset `animating`** (on_complete + timeout dự phòng) để phòng frame clock
> stall giữa chừng (vd panel rebuild) làm transition kẹt vĩnh viễn.

---

## 7. Luồng media player

Feature `media_player` (priority 50) — toàn bộ luồng hiện tại:

### 7.1. Polling playerctl (nền)

```text
poll.rs: spawn_playerctl_polling()
  → thread riêng: mỗi 1s gọi `playerctl metadata --format "<status>|//|<title>|//|<artist>|//|<playerName>|//|<mpris:artUrl>|//|<position>|//|<mpris:length>"`
  → gửi raw line qua tokio channel (unbounded)
  → main-thread task nhận, cache vào latest_metadata (Rc<RefCell<Option<String>>>)
  → thread tự thoát khi channel đóng (receiver drop = feature dispose)
```

### 7.2. Tick (mỗi 150ms)

```text
refresh(ctx):
  đọc latest_metadata (cache — không gọi playerctl trên UI thread)
  parse_metadata(line) → (PlayerMeta, player_active)
  player_active (Playing hoặc Paused)?
      → handle.show()                ── giữ requested (media player luôn hiện khi chạy)
      : → handle.hide()
  is_playing = player_active && playing
  nếu player_active && ctx.is_current() → update_player_view(&meta)
```

### 7.3. Render (chỉ khi view đang hiển thị)

```text
update_player_view:
  - progress bar popover: fraction = pos/len (nếu len > 0)
  - label: "<artist> - <title>" (truncate 15 ký tự + "..." nếu quá 18)
  - chỉ cập nhật text khi song_changed (meta_key đổi) HOẶC mỗi 7 tick (đếm poll_counter)
  - artwork: nếu chưa load cho bài này → spawn thread tải (file:// đọc trực tiếp, http(s) qua curl --max-time 5)
    → gửi bytes qua art channel → main-thread art receiver scale + gán vào notch (16px) + popover (240px)
    → lỗi: retry tối đa 3 lần, sau đó fallback icon player
```

### 7.4. Popover (gắn trong `attach`)

```text
attach: MediaPopover::new(capsule) — popover thả xuống từ capsule
on_click: popover.toggle()
on_hide:  popover.popdown()  (tự đóng khi notification chiếm chỗ)
```

---

## 8. Luồng notification

Feature `notification` (priority 90, hover_keep, capsule_class `notification-mode`) — luồng hiện tại:

### 8.1. D-Bus → SHARED_NOTIFICATION

```text
service.rs: spawn_notification_dbus_service()
  → spawn_dbus_listener (từ babydra-core) host org.freedesktop.Notifications
  → message qua tokio channel
  → main-thread task: New → show_notification_popup(summary, body, icon, app_name, timeout)
                            → ghi vào SHARED_NOTIFICATION (thread_local, ActiveNotification + timestamp)
                       Close → close_notification_popup() → xóa SHARED_NOTIFICATION
```

### 8.2. Tick (mỗi 150ms)

```text
tick(ctx):
  đọc SHARED_NOTIFICATION
  không có notification? → handle.hide(); return

  key = "title|body|icon" — nếu đổi → render(&n) (đẩy text + icon + đo lại chiều cao)

  đang hover?
      → refresh n.timestamp = now  (kéo dài vòng đời khi hover)
  hết hạn (không hover && timestamp.elapsed() >= 5s)?
      → xóa SHARED_NOTIFICATION; handle.hide()
  còn hạn → handle.show()
```

### 8.3. Render + đo chiều cao động

```text
render(notif):
  title truncate 35 ký tự, body truncate 80 ký tự
  icon: nếu icon rỗng / path không tồn tại / tên icon không có trong theme → logo
        ngược lại → get_system_or_file_icon
  measure chiều cao tự nhiên (TARGET_WIDTH 280 - 32) → desired = (280, h) với h ≥ 48
```

> [!TIP]
> `size()` của feature được **đọc lại mỗi transition** (mục 6.2) nên notification
> capsule tự co giãn theo độ dài nội dung — descriptor view thì dùng size cố định.

### 8.4. Click → focus app

```text
on_click: lấy app_name từ SHARED_NOTIFICATION
  → tìm trong find_desktop_apps (khớp tên chính xác, rồi khớp contains)
  → babydra_core::helper::window::focus_app(...)
```

---

## 9. Luồng idle logo

- `default::idle_logo_view()` — logo pill nhỏ (28×16), chỉ hiển thị khi `idle_visible = true`.
- Không phải feature — là content idle của island, được `content_box` append lúc build.
- Khi arbitration không chọn view nào và `idle_visible` → `IslandDisplay::Idle` → hiện logo + `animate_expand(IDLE_SIZE)`.
- Khi chuyển sang view khác → ẩn idle trước khi expand (mục 6.2).

---

## 10. Thread, channel & dọn dẹp

| Luồng nền | Ai tạo | Thoát khi |
| :--- | :--- | :--- |
| Thread poll playerctl | `MediaPlayerFeature::new()` | channel receiver drop (feature dispose) |
| Thread tải artwork | `update_player_view` (mỗi bài mới) | thread ngắn hạn, tự kết thúc |
| Task nhận art (main thread) | `attach` → `spawn_art_receiver` | receiver drop |
| D-Bus notification listener | `NotificationFeature::new()` | channel đóng |
| Controller loop (main thread) | `build_island` | `island.dispose()` → `source.remove()` |

**Kịch bản panel rebuild:**

```text
panel rebuild → set_default_island(island mới)
  → island cũ: dispose() → controller loop dừng
  → island mới: register lại features → init/attach chạy lại → receiver mới
```

> [!WARNING]
> Sau mỗi rebuild phải **re-resolve `default_island()` và đăng ký lại** view/feature —
> island cũ đã dispose, các handle cũ không còn tác dụng.
