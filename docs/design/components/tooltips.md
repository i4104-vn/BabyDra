# Component: Tooltips

Tài liệu quy định cách sử dụng và mã nguồn chuẩn cho **Tooltip** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `libs/babydra-utils/src/components/tooltips/mod.rs`

---

## 1. Tổng quan

Tooltip hiển thị mô tả ngắn khi rê chuột lên widget (thường là icon button). BabyDra dùng cơ chế tooltip mặc định của GTK4 qua một helper thống nhất.

---

## 2. API

```rust
pub fn set_tooltip(widget: &impl IsA<gtk4::Widget>, text: &str)
```

- Gọi `widget.set_tooltip_text(Some(text))`.
- Được re-export tại `components::set_tooltip`.

---

## 3. Ví dụ sử dụng

```rust
use babydra_utils::components::set_tooltip;

let btn = create_icon_button("edit-delete", ...);
set_tooltip(&btn, "Forget Network");
```

---

## 4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Icon button **bắt buộc** có tooltip mô tả chức năng |
| DO | Dùng `set_tooltip` để thống nhất cách gọi |
| DO | Tooltip ngắn gọn, không lặp lại nhãn đã hiển thị cạnh đó |
| DO NOT | Không để icon-only button thiếu tooltip |
