# Hướng dẫn sử dụng & mở rộng Dynamic Island

**Crate:** `libs/babydra-island/`
**Phạm vi:** Kiến trúc view stack của island, cách đăng ký view/feature mới, điều khiển hiển thị (show/hide/override), truy cập toàn cục.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Tổng quan](#1-tổng-quan)
- [2. Cách hoạt động & luồng](#2-cách-hoạt-động--luồng)
- [3. Bắt đầu nhanh](#3-bắt-đầu-nhanh)
- [4. Cách 1 — View descriptor + Handle](#4-cách-1--view-descriptor--handle)
- [5. Cách 2 — Trait `IslandFeature`](#5-cách-2--trait-islandfeature)
- [6. Điều khiển hiển thị](#6-điều-khiển-hiển-thị)
- [7. Arbitration & Priority](#7-arbitration--priority)
- [8. Ghi đè media player (override)](#8-ghi-đè-media-player-override)
- [9. Truy cập toàn cục `default_island()`](#9-truy-cập-toàn-cục-default_island)
- [10. Dựng island tùy chỉnh](#10-dựng-island-tùy-chỉnh)
- [11. Vòng đời & lưu ý](#11-vòng-đời--lưu-ý)
- [12. Quy tắc](#12-quy-tắc)

---

## 1. Tổng quan

`babydra-island` là widget **Dynamic Island** — notch capsule trên thanh panel. Nó là một **view stack**: tại mỗi thời điểm chỉ có **một view** được hiển thị, chọn bởi controller loop theo **priority** (xem mục 7).

Hai cách thêm view:

| Cách | Dành cho | API |
| :--- | :--- | :--- |
| **Descriptor + Handle** | Overlay đơn giản (volume, brightness, clipboard, timer…) | `IslandView` → `IslandViewHandle` |
| **Trait `IslandFeature`** | Feature phức tạp, tự vận hành (media player, notification) | `impl IslandFeature` |

Các feature mặc định đi kèm: `media_player` (playerctl + visualizer + popover điều khiển), `notification` (thông báo desktop), `default` (idle logo).

> [!NOTE]
> Hướng dẫn tạo feature mới theo cấu trúc chuẩn (tách sub-folder) xem
> [island-features.md](./island-features.md). Muốn hiểu **luồng vận hành bên trong**
> (controller loop, arbitration, transition, luồng dữ liệu từng feature) xem
> [island-internals.md](./island-internals.md).

---

## 2. Cách hoạt động & luồng

Island là một **view stack** điều khiển bởi một **controller loop** chạy mỗi
`poll_interval_ms` (mặc định 150ms). Mỗi tick gồm 4 bước:

```text
1. Timer       → hết hạn show_for / override_show_for → rút yêu cầu / nhả override
2. Feature tick → mọi feature cập nhật trạng thái + gọi handle.show()/hide()
3. Arbitration → chọn view thắng (override > priority > gần đây nhất > thứ tự đăng ký)
4. Transition  → nếu đổi view: on_hide(view cũ) → animate expand/collapse → on_show(view mới)
```

Tóm tắt luồng dữ liệu từng feature built-in:

| Feature | Nguồn dữ liệu | Luồng |
| :--- | :--- | :--- |
| `media_player` | Thread poll `playerctl` mỗi 1s | raw line → channel → cache (main thread) → tick đọc cache → `show()/hide()` → render khi đang hiển thị → thread tải artwork → art receiver gán ảnh |
| `notification` | D-Bus daemon `org.freedesktop.Notifications` | message → channel → main thread → `SHARED_NOTIFICATION` → tick đọc → `show()` 5s (kéo dài khi hover) → `hide()` khi hết hạn |
| `default` (idle) | — | Hiện logo pill nhỏ khi không view nào thắng và `idle_visible = true` |

> [!IMPORTANT]
> Player giữ `requested` liên tục khi có player đang chạy → muốn overlay tạm thời
> chiếm chỗ rồi tự trả lại player phải dùng `override_show_for()` (mục 9),
> không thể dựa vào priority thường.

Mô tả chi tiết từng bước (thành phần runtime, thuật toán arbitration, transition,
toàn bộ luồng media player / notification): **[island-internals.md](./island-internals.md)**.

---

## 3. Bắt đầu nhanh

```toml
# Cargo.toml
[dependencies]
babydra-island = { workspace = true }
```

Đăng ký một overlay tạm thời và hiển thị nó trong 1.5 giây:

```rust
use babydra_island::default_island;
use std::time::Duration;

fn show_volume_overlay(percent: u8) {
    let island = default_island().expect("island chưa được khởi tạo");
    let handle = island.get_handle("volume").expect("view volume chưa được đăng ký");

    // Cập nhật nội dung rồi ghi đè hiển thị trong 1.5s.
    handle.set_content(build_volume_widget(percent));
    handle.override_show_for(Duration::from_millis(1500));
}
```

> [!IMPORTANT]
> `default_island()` chỉ trả về `Some` sau khi `create_system_island()` /
> `build_default_island()` chạy (panel khởi tạo). Nếu view chưa được đăng ký,
> hãy đăng ký trước (mục 4) rồi dùng handle.

---

## 4. Cách 1 — View descriptor + Handle

Phù hợp cho overlay đơn giản: bạn dựng widget, mô tả view (priority, kích thước, callback), đăng ký và nhận handle để điều khiển.

### 4.1. Tạo và đăng ký view

```rust
use babydra_island::{default_island, IslandView};
use gtk4::prelude::*;

fn register_volume_view() {
    let island = default_island().unwrap();

    let content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    content.set_valign(gtk4::Align::Center);
    // ... thêm icon + label ...

    let handle = island.register_view(
        IslandView::new("volume", content)
            .priority(70)               // cao hơn media_player (50)
            .size(220, 40)              // kích thước capsule khi hiển thị
            .capsule_class("volume-mode")
            .on_click(|| { /* click trên island */ }),
    );

    // Lưu handle để dùng sau (hoặc lấy lại qua island.get_handle("volume")).
}
```

Các builder method của `IslandView`:

| Method | Mặc định | Ý nghĩa |
| :--- | :--- | :--- |
| `priority(p: u8)` | `50` | Độ ưu tiên arbitration — cao hơn thắng |
| `size(w, h)` | `(200, 30)` | Kích thước capsule khi view hiển thị |
| `hover_keep(b)` | `false` | Giữ hiển thị khi pointer hover capsule |
| `capsule_class(c)` | — | CSS class thêm vào capsule khi view hiển thị |
| `on_show(f)` / `on_hide(f)` | — | Callback khi view vào/ra |
| `on_click(f)` | — | Callback khi click capsule lúc view hiển thị |

> [!TIP]
> `IslandView::with_builder("id", || widget)` trì hoãn việc dựng widget đến lúc đăng ký.

### 4.2. Handle

`IslandViewHandle` rẻ để clone và **không khóa manager** — an toàn khi gọi từ callback/tick.

| Method | Ý nghĩa |
| :--- | :--- |
| `show()` | Yêu cầu hiển thị (theo priority arbitration) |
| `show_for(d)` | Hiển thị rồi tự ẩn sau `d` |
| `hide()` | Rút yêu cầu hiển thị |
| `override_show()` | Hiển thị ngay, bỏ qua priority, đến khi `release_override()` |
| `override_show_for(d)` | Ghi đè trong `d`, sau đó tự trả lại view trước đó |
| `release_override()` | Kết thúc ghi đè (yêu cầu hiển thị giữ nguyên) |
| `set_content(widget)` | Thay nội dung widget runtime |
| `is_requested()` / `is_active()` | Trạng thái |

---

## 5. Cách 2 — Trait `IslandFeature`

Dành cho feature có trạng thái, tự quyết định khi nào hiển thị (media player, notification). Đăng ký bằng `island.register_feature(Box::new(my_feature))`.

```rust
use babydra_island::{IslandCtx, IslandFeature, IslandViewHandle};

pub struct BatteryFeature {
    handle: Option<IslandViewHandle>,
    label: gtk4::Label,
    level: u8,
}

impl IslandFeature for BatteryFeature {
    fn id(&self) -> &str { "battery" }

    fn priority(&self) -> u8 { 60 }

    fn size(&self) -> (i32, i32) { (180, 36) }

    fn build_view(&mut self) -> gtk4::Widget {
        self.label.clone().upcast()
    }

    fn init(&mut self, handle: &IslandViewHandle) {
        self.handle = Some(handle.clone());
    }

    /// Chạy mỗi tick (~150ms) cho mọi feature đã đăng ký.
    fn tick(&mut self, ctx: &IslandCtx) {
        if let Some(info) = babydra_core::get_battery_info() {
            let text = format!("Battery {}%", info.percent);
            self.label.set_text(&text);
            if let Some(h) = &self.handle {
                h.show();               // giữ yêu cầu hiển thị khi có pin
            }
        }
    }
}
```

Toàn bộ methods của trait:

| Method | Mặc định | Vai trò |
| :--- | :--- | :--- |
| `id()` | — | Định danh view (bắt buộc) |
| `priority()` | `50` | Độ ưu tiên |
| `size()` | `(200, 30)` | Kích thước hiện tại — đọc **mỗi lần transition** (cho phép co giãn động) |
| `hover_keep()` | `false` | Giữ hiển thị khi hover |
| `capsule_class()` | — | CSS class thêm vào capsule |
| `build_view()` | — | Dựng widget nội dung (bắt buộc) |
| `init(handle)` | no-op | Nhận handle tại lúc đăng ký |
| `attach(ctx)` | no-op | Nhận capsule (`ctx.capsule()`) — dùng để gắn popover |
| `on_show()` / `on_hide()` | no-op | Transition vào/ra |
| `on_click()` | no-op | Click trên capsule |
| `tick(ctx)` | no-op | Gọi mỗi poll interval cho **mọi** feature |

`IslandCtx`:

| Method | Ý nghĩa |
| :--- | :--- |
| `capsule() -> gtk4::Box` | Notch capsule (gắn popover, thêm controller…) |
| `is_current() -> bool` | View của feature đang được hiển thị |
| `is_hovered() -> bool` | Pointer đang hover capsule |

> [!TIP]
> `size()` được gọi lại mỗi transition nên feature có thể trả kích thước thay đổi
> theo nội dung (notification tự đo chiều cao). Descriptor view dùng kích thước cố định.

---

## 6. Điều khiển hiển thị

```rust
let island = default_island().unwrap();

island.show("volume");                    // yêu cầu theo priority
island.hide("volume");                    // rút yêu cầu
island.override_view("volume", None);     // force hiển thị
island.override_view("volume", Some(Duration::from_millis(1500))); // tự trả lại

// Qua handle (ưu tiên dùng):
let h = island.get_handle("volume").unwrap();
h.show_for(Duration::from_secs(3));
```

---

## 7. Arbitration & Priority

Mỗi poll interval (~150ms), controller loop:

1. Chạy `tick()` của mọi feature → các feature cập nhật trạng thái + `show()/hide()`.
2. Chọn view thắng:
   - **Override active** thắng tuyệt đối (gần đây nhất thắng khi có nhiều override).
   - Ngược lại: **priority cao nhất**; hòa → view **yêu cầu gần đây nhất**; vẫn hòa → thứ tự đăng ký.
   - View `hover_keep` đang hiển thị được giữ khi hover (không cần yêu cầu lại).
3. Nếu view thắng khác view hiện tại → chuyển transition (animate kích thước/zoom), gọi `on_hide`/`on_show`.

Thứ tự priority mặc định:

| View | Priority | Ghi chú |
| :--- | :--- | :--- |
| `notification` | `90` | Thắng player, tự ẩn sau 5s (kéo dài khi hover) |
| `volume` (ví dụ) | `70` | Overlay tạm thời |
| `media_player` | `50` | Hiển thị khi có player đang chạy |
| `default` (idle) | — | Logo nhỏ khi `idle_visible = true` |

---

## 8. Ghi đè media player (override)

Media player gần như **luôn hiển thị** khi có player đang chạy. Để tạm thời thay thế nó
(volume, brightness, clipboard, timer…) rồi **tự trả lại player**, dùng `override_show_for`:

```rust
// Ví dụ: người dùng đổi âm lượng → overlay volume chiếm chỗ 1.5s → player quay lại.
fn on_volume_changed(percent: u8) {
    let island = default_island().unwrap();
    let h = island.get_handle("volume").unwrap();
    h.set_content(build_volume_widget(percent));
    h.override_show_for(Duration::from_millis(1500));
}
```

Cơ chế:

- `override_show_for(d)` đặt cả `release_at` lẫn `auto_hide_at` = `now + d`.
- Hết `d`: override bị nhả, yêu cầu hiển thị bị rút → arbitration quay lại priority
  → media player (vẫn đang yêu cầu) được khôi phục.
- Nếu cần ghi đè lâu dài: `override_show()` + `release_override()` thủ công.
  Sau `release_override()`, yêu cầu hiển thị vẫn giữ — view tiếp tục tham gia arbitration
  bình thường.

> [!IMPORTANT]
> `show()` không reset deadline auto-hide khi view đã đang được yêu cầu — một feature
> gọi `show()` trong `tick()` sẽ **không** vô tình hủy `show_for`/`override_show_for`
> đang chờ.

---

## 9. Truy cập toàn cục `default_island()`

```rust
use babydra_island::default_island;

// Trả về manager của island mặc định (do panel tạo qua create_system_island()).
let island = default_island().unwrap();

// Đăng ký view/feature từ bất kỳ đâu trong process.
island.register_view(...);
island.register_feature(Box::new(MyFeature::new()));
```

> [!WARNING]
> Khi panel rebuild (đổi theme/locale), island cũ bị `dispose()` và một island mới
> được tạo — **hãy re-resolve `default_island()` và đăng ký lại** sau mỗi rebuild.

---

## 10. Dựng island tùy chỉnh

```rust
use babydra_island::{Island, IslandConfig, IslandFeature};

let island = Island::builder()
    .config(IslandConfig {
        idle_visible: true,          // hiện logo khi rảnh
        poll_interval_ms: 150,
        expand_ms: 350,
        collapse_ms: 500,
        ..Default::default()
    })
    .feature(Box::new(my_feature))
    .view(my_view)
    .idle(babydra_island::features::default::idle_logo_view())
    .build();

// Gắn capsule vào layout:
let capsule = island.capsule();      // gtk4::Box
```

API builder:

| Method | Ý nghĩa |
| :--- | :--- |
| `config(IslandConfig)` | Cấu hình loop + animation |
| `idle_visible(bool)` | Hiện idle logo khi không có view nào |
| `idle(widget)` | Nội dung idle logo |
| `view(IslandView)` | Thêm descriptor view |
| `feature(Box<dyn IslandFeature>)` | Thêm trait feature |
| `build() -> Island` | Dựng island + controller loop |

`Island` (clone rẻ, chia sẻ state):

| Method | Ý nghĩa |
| :--- | :--- |
| `capsule()` | Gắn capsule vào layout |
| `register_view(view)` / `register_feature(f)` | Đăng ký thêm lúc runtime |
| `get_handle(id)` / `handles()` | Lấy handle |
| `show(id)` / `hide(id)` / `override_view(id, d)` | Điều khiển nhanh |
| `dispose()` | Dừng controller loop (tự gọi khi rebuild) |

`IslandConfig`:

| Field | Mặc định | Ý nghĩa |
| :--- | :--- | :--- |
| `idle_visible` | `false` | Hiện logo idle khi rảnh |
| `poll_interval_ms` | `150` | Chu kỳ controller loop |
| `expand_ms` | `350` | Thời gian animation mở rộng |
| `collapse_ms` | `500` | Thời gian animation thu lại |

---

## 11. Vòng đời & lưu ý

- **Luồng GTK:** controller loop, tick, callback đều chạy trên main thread — handle
  an toàn gọi từ mọi callback.
- **Feature phức tạp:** dựng widget trong `build_view()`, gắn popover/capsule trong
  `attach()`, cập nhật trong `tick()` khi `ctx.is_current()`.
- **Thread nền:** polling (playerctl) dùng channel → main thread; thread tự thoát khi
  feature bị hủy (receiver drop).
- **Không gọi manager (register_view…) từ trong `tick()`/callback của feature** — chỉ
  gọi method handle (`show/hide/set_content…`).

---

## 12. Quy tắc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Overlay tạm thời dùng `override_show_for(duration)` để tự trả lại view trước đó |
| DO | Feature phức tạp implement `IslandFeature`; overlay đơn giản dùng `IslandView` |
| DO | Cập nhật nội dung qua `set_content()` hoặc trực tiếp widget trước khi `show()` |
| DO | Sau panel rebuild phải re-resolve `default_island()` |
| DO NOT | Gọi `register_view`/`register_feature` từ trong tick/callback của feature |
| DO NOT | Tạo nhiều island song song nếu không cần — dùng `default_island()` |

Xem thêm: [island-internals.md](./island-internals.md) — kiến trúc runtime & luồng hoạt động chi tiết,
[island-features.md](./island-features.md) — cấu trúc chuẩn khi tạo feature mới,
[structure](../structure/index.md) — cây thư mục đầy đủ của crate.
