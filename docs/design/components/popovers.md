# Component: Popovers

Tài liệu quy định cách sử dụng, kiểu dáng, và mã nguồn chuẩn cho **Popover** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `libs/babydra-utils/src/components/popovers/`

---

## 1. Tổng quan

Popover là bề mặt nổi nhỏ gắn với một widget cha (anchor), dùng để hiển thị thông tin ngữ cảnh hoặc menu. BabyDra có 2 loại:

| Loại | Hàm | Công dụng |
| :--- | :--- | :--- |
| Popover chuẩn | `create_popover` / `create_popover_with_content` | Popover thường gắn vị trí cố định |
| Hover Popover | `attach_hover_popover` | Tự hiện khi rê chuột vào icon, tự ẩn khi rời — dùng cho status icon trên panel |

---

## 2. API

### 2.1. Popover chuẩn

```rust
pub fn create_popover(parent: &impl IsA<gtk4::Widget>, position: PositionType, css_class: &str) -> gtk4::Popover
pub fn create_popover_with_content(parent, position, css_class, content) -> gtk4::Popover
```

### 2.2. Hover Popover

```rust
pub struct HoverPopoverRow { pub key: String, pub val: String, pub css_class: Option<String> }

pub fn build_hover_popover_card(title: &str, rows: Vec<HoverPopoverRow>) -> gtk4::Box
pub fn attach_hover_popover(anchor_widget, popover, update_fn: Rc<dyn Fn()>)
```

**Cơ chế `attach_hover_popover`:**

1. Gắn `EventControllerMotion` lên anchor — khi chuột vào → gọi `update_fn()` (cập nhật dữ liệu mới) rồi `popup()`.
2. Khi chuột rời anchor → chờ **150ms** (`timeout_add_local`) rồi `popdown()` — cho phép chuột di chuyển vào popover.
3. Gắn `EventControllerMotion` lên chính popover — giữ mở khi chuột đang ở trong.
4. `popover.set_autohide(false)` — không tự ẩn theo hành vi mặc định.

### 2.3. Cấu trúc card hover

`build_hover_popover_card` tạo Box class `status-popover-card`:

- Header: `status-popover-header` (tiêu đề) + separator `status-popover-sep`.
- Mỗi dòng: key (`status-popover-key`, căn trái, hexpand) + value (`status-popover-val`, căn phải, có thể gắn thêm class).
- Margin 4px dọc / 6px ngang.

---

## 3. Style

```css
/* Dark — surface popover trạng thái */
popover.status-popover > contents {
    background-color: rgba(18, 18, 24, 0.94);
    border: 1px solid rgba(255, 255, 255, 0.14);
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
}

/* Panel popover mặc định */
popover > contents {
    background-color: rgba(14, 14, 18, 0.96);
    border: 1px solid rgba(255, 255, 255, 0.14);
    border-top: 1px solid rgba(255, 255, 255, 0.28);
}
```

---

## 4. Ví dụ sử dụng

```rust
// Tạo popover hover hiển thị thông tin pin
// Class tùy chọn cho value dùng token có thật: `success-text` hoặc `settings-desc`.
let rows = vec![
    HoverPopoverRow::new("Battery", "78%", Some("success-text")),
    HoverPopoverRow::new("Charging", "Yes", None),
];
let card = build_hover_popover_card("Power", rows);

let popover = create_popover_with_content(&battery_icon, PositionType::Top, "status-popover", &card);
attach_hover_popover(&battery_icon, &popover, Rc::new(|| {
    // cập nhật dữ liệu mới trước khi hiện
}));
```

---

## 5. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Popover trạng thái trên panel phải dùng `attach_hover_popover` + `build_hover_popover_card` |
| DO | Popover phải gắn `PositionType` phù hợp vị trí anchor (Top/Bottom/Left/Right) |
| DO | Hover popover phải gọi `update_fn` để dữ liệu luôn mới khi hiện |
| DO NOT | Không dùng `popover.set_autohide(true)` với hover popover (gây tắt khi chuyển chuột) |
| DO NOT | Không tự dựng cấu trúc key–value tay ngoài `HoverPopoverRow` |
