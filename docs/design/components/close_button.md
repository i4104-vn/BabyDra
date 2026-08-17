# Component: Close Button

Tài liệu quy định cách sử dụng, kiểu dáng, và mã nguồn chuẩn cho **Close Button** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `libs/babydra-ui-kit/src/components/close_button/mod.rs`

---

## 1. Tổng quan

Close button là nút icon "×" (window-close) dùng để đóng dialog, popover, overlay. Có 2 biến thể: icon thuần và icon + nhãn.

---

## 2. API

```rust
pub fn create_close_button(css_class: &str) -> gtk4::Button
pub fn create_close_button_with_label(label_text: &str, css_class: &str) -> gtk4::Button
```

| Hàm | Mô tả |
| :--- | :--- |
| `create_close_button` | Nút chỉ icon 12px |
| `create_close_button_with_label` | Nút icon 12px + nhãn (spacing 6) |

- Nếu `css_class` rỗng → class mặc định `close-btn`.
- Luôn `set_cursor_from_name("pointer")`.
- Icon lấy qua `get_system_or_file_icon("window-close", ...)`.

---

## 3. Ví dụ sử dụng

```rust
// Nút đóng icon-only trong overlay
let close = create_close_button("close-btn");
close.connect_clicked(|_| { overlay.set_visible(false); });

// Nút đóng có nhãn
let close_lbl = create_close_button_with_label("Close", "");
```

---

## 4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Nút đóng overlay/dialog dùng `create_close_button` |
| DO | Luôn gắn `connect_clicked` để ẩn đúng container |
| DO NOT | Không dùng `gtk4::Button` thô với icon tự vẽ khác `window-close` |
