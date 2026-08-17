# Hướng dẫn tạo Island Feature mới

**Phạm vi:** Cấu trúc chuẩn của một feature folder trong `libs/babydra-island/src/features/`, cách tạo feature mới nhất quán.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Vì sao cần cấu trúc chung](#1-vì-sao-cần-cấu-trúc-chung)
- [2. Cấu trúc chuẩn của một feature](#2-cấu-trúc-chuẩn-của-một-feature)
- [3. Tạo feature mới từng bước](#3-tạo-feature-mới-từng-bước)
- [4. Checklist](#4-checklist)

---

## 1. Vì sao cần cấu trúc chung

Mỗi feature trong island là một **thư mục riêng** (`features/<feature>/`) thay vì một file dài. Lợi ích:

- File không quá dài (dưới ~200 dòng/file) — dễ đọc, dễ review.
- Người mới đã quen một feature sẽ hiểu ngay feature khác.
- Code tái sử dụng (view, render, service) nằm đúng chỗ, không lẫn logic.

> [!IMPORTANT]
> Quy tắc này áp dụng cho **mọi feature** trong `features/` — built-in lẫn do bạn thêm.

---

## 2. Cấu trúc chuẩn của một feature

```text
features/<feature>/
├── mod.rs        # Struct + constructor + impl IslandFeature (vòng đời + tick)
├── view.rs       # Xây dựng cây widget — struct <Feature>View + View::build()
├── render.rs     # Đẩy dữ liệu vào widget — impl <Feature> { fn update_*/render() }
└── service.rs    # (tùy chọn) Service nền — polling, D-Bus, thread worker
```

### 2.1. Trách nhiệm từng file

| File | Bắt buộc | Trách nhiệm | Ví dụ thật |
| :--- | :--- | :--- | :--- |
| `mod.rs` | ✅ | Struct feature, `new()`, `impl IslandFeature` (`tick`, `on_click`, `on_show`, `on_hide`, `init`, `attach`) | `media_player/mod.rs`, `notification/mod.rs` |
| `view.rs` | ✅ | Widget struct + `build()` — chỉ dựng widget, không có logic dữ liệu | `media_player/view.rs` (`PlayerWidgets`) |
| `render.rs` | ✅ | Hàm/method đẩy dữ liệu → widget; parse dữ liệu thô | `media_player/render.rs` (`update_player_view`) |
| `service.rs` | ⚠️ Chỉ khi có | Service nền: thread polling, D-Bus listener, channel bridge | `notification/service.rs` |

### 2.2. File helper riêng của feature

Nếu feature cần thêm module riêng, đặt chung trong folder feature — tên mô tả đúng vai trò:

```text
features/media_player/          # ví dụ feature phức tạp
├── mod.rs
├── view.rs                     # PlayerWidgets + build()
├── render.rs                   # update_player_view + parse_metadata
├── poll.rs                     # polling playerctl (service nền)
├── art.rs                      # tải artwork + retry + fallback
├── popover.rs                  # MediaPopover (widget riêng)
├── visualizer.rs               # thanh visualizer (widget riêng)
└── format.rs                   # format_time, get_player_icon_name (thuần)
```

> [!TIP]
> Feature nhỏ không cần chia nhỏ — chỉ cần `mod.rs` (xem `features/default/`).

### 2.3. Quy tắc chia file

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Mỗi feature = 1 thư mục trong `features/` |
| DO | Mọi feature đều có `mod.rs` + `view.rs` + `render.rs` (nếu dưới ~60 dòng có thể gộp vào `mod.rs`) |
| DO | Widget + dữ liệu dùng chung giữa các file đặt trong struct widget (`<Feature>View`) |
| DO | Service nền tách `service.rs` — không nhét thread/D-Bus vào `mod.rs` |
| DO NOT | File `mod.rs` dài hơn ~250 dòng — tách render/view/service khi vượt |
| DO NOT | Để widget của feature này trong `widgets/` (đó là nơi re-export dùng chung) |

---

## 3. Tạo feature mới từng bước

### Bước 1 — Tạo folder + khai báo module

```bash
mkdir -p libs/babydra-island/src/features/my_feature
```

```rust
// features/mod.rs — khai báo feature mới
pub mod my_feature;
```

### Bước 2 — `view.rs`: dựng widget

```rust
// features/my_feature/view.rs
pub(crate) struct MyFeatureView {
    pub root: gtk4::Box,
    pub label: gtk4::Label,
}

impl MyFeatureView {
    pub(crate) fn build() -> Self {
        let root = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        root.set_valign(gtk4::Align::Center);
        let label = gtk4::Label::new(None);
        root.append(&label);
        Self { root, label }
    }
}
```

### Bước 3 — `render.rs`: đẩy dữ liệu vào widget

```rust
// features/my_feature/render.rs
use super::MyFeature;

impl MyFeature {
    pub(crate) fn render(&mut self, data: &str) {
        self.view.label.set_text(data);
        // Có thể đo lại kích thước mong muốn cho capsule.
        self.desired = (220, 40);
    }
}
```

### Bước 4 — `mod.rs`: struct + `IslandFeature`

```rust
// features/my_feature/mod.rs
mod render;
mod view;

use std::time::Duration;

use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use view::MyFeatureView;

pub const PRIORITY: u8 = 60;

pub struct MyFeature {
    handle: Option<IslandViewHandle>,
    view: MyFeatureView,
    desired: (i32, i32),
}

impl MyFeature {
    pub fn new() -> Self {
        Self {
            handle: None,
            view: MyFeatureView::build(),
            desired: (220, 40),
        }
    }
}

impl Default for MyFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl IslandFeature for MyFeature {
    fn id(&self) -> &str {
        "my_feature"
    }
    fn priority(&self) -> u8 {
        PRIORITY
    }
    fn size(&self) -> (i32, i32) {
        self.desired
    }
    fn build_view(&mut self) -> gtk4::Widget {
        self.view.root.clone().upcast()
    }
    fn init(&mut self, handle: &IslandViewHandle) {
        self.handle = Some(handle.clone());
    }

    fn tick(&mut self, _ctx: &IslandCtx) {
        // Đọc dữ liệu, render, rồi show/hide qua handle.
        if let Some(h) = &self.handle {
            h.show_for(Duration::from_secs(3));
        }
    }
}
```

### Bước 5 — Đăng ký vào island

```rust
let island = babydra_island::default_island().unwrap();
island.register_feature(Box::new(MyFeature::new()));
```

> [!NOTE]
> Chi tiết từng method của `IslandFeature` xem [island.md](./island.md).

---

## 4. Checklist

| # | Kiểm tra |
| :--- | :--- |
| 1 | Feature nằm trong thư mục riêng `features/<feature>/` |
| 2 | Có đủ `mod.rs` + `view.rs` + `render.rs` (và `service.rs` nếu có service nền) |
| 3 | Không file nào dài quá ~200 dòng |
| 4 | `id()` duy nhất, không trùng feature khác |
| 5 | Chạy được: `cargo check -p babydra-island && cargo clippy -p babydra-island --no-deps -- -D warnings` |
| 6 | Có bảng cấu trúc module ở đầu `mod.rs` (như `media_player/mod.rs`) |
