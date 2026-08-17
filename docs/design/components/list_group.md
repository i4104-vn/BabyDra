# Component: List Group

Tài liệu quy định cách sử dụng, kiểu dáng, và mã nguồn chuẩn cho **List Row** và helper danh sách trong hệ thống BabyDra.

**Vị trí mã nguồn:** `kits/babydra-ui-kit/src/components/list_group/mod.rs`

---

## 1. Tổng quan

List Group cung cấp:

- **List Row chuẩn**: icon + title + subtitle + widget phải (tùy chọn).
- **Helper dọn danh sách**: xóa toàn bộ con của `ListBox` / `Box` (dùng khi refresh dữ liệu).

---

## 2. API

```rust
pub fn create_list_row(
    icon_name: &str,
    title: &str,
    subtitle: &str,
    right_widget: Option<&impl IsA<gtk4::Widget>>,
) -> gtk4::Box

pub fn clear_list_box(list_box: &gtk4::ListBox)
pub fn clear_box(box_container: &gtk4::Box)
```

**Cấu trúc `create_list_row`:**

- Margin đều 8px bốn cạnh, spacing 12.
- Icon 20px (nếu `icon_name` rỗng → bỏ qua icon).
- Cột text: title `settings-label` + subtitle `settings-desc`, căn trái.
- Spacer `hexpand` đẩy widget phải về cuối hàng.

---

## 3. Ví dụ sử dụng

```rust
// Refresh danh sách ứng dụng
clear_list_box(&app_list);

for app in apps {
    let row = create_list_row(&app.icon, &app.name, &app.description, Some(&app.switch));
    app_list.append(&row);
}
```

---

## 4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Dòng danh sách (icon + title + desc) dùng `create_list_row` |
| DO | Khi refresh danh sách phải `clear_list_box` trước khi thêm lại |
| DO | Widget phải (switch, nút) truyền qua `right_widget` — không tự append sau |
| DO NOT | Không để icon trống mà không truyền `""` (hàm tự bỏ qua) |
