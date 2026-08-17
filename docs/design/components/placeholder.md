# Component: Placeholder

Tài liệu quy định cách sử dụng, kiểu dáng, và mã nguồn chuẩn cho **Placeholder State** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `kits/babydra-ui-kit/src/components/placeholder/mod.rs`

---

## 1. Tổng quan

Placeholder là dòng chiếm chỗ thống nhất cho 3 trạng thái của danh sách Settings:

| Trạng thái | Ý nghĩa |
| :--- | :--- |
| `Disabled` | Phần cứng/tính năng không khả dụng — icon + title + desc |
| `Loading` | Đang tải — spinner 32px + label "Loading" |
| `Empty` | Không có dữ liệu — icon + title (+ desc tùy chọn) |

Mọi chuỗi hiển thị đều đi qua i18n (`babydra_core::i18n::t`).

---

## 2. API

```rust
pub enum PlaceholderState<'a> {
    Disabled { title_key: &'a str, desc_key: &'a str, icon_name: &'a str },
    Loading,
    Empty { title_key: &'a str, desc_key: Option<&'a str>, icon_name: &'a str },
}

pub fn create_placeholder_row(state: PlaceholderState) -> gtk4::ListBoxRow
```

- Kết quả là `ListBoxRow` class `settings-card-row`, không chọn/không kích hoạt được.
- Nội dung căn giữa, margin dọc 40px, icon qua `create_icon_badge(icon_name, 24, false)` — badge tròn 44px chứa icon 24px.

---

## 3. Ví dụ sử dụng

```rust
// Trạng thái rỗng
let row = create_placeholder_row(PlaceholderState::Empty {
    title_key: "settings.no_bluetooth_devices",
    desc_key: Some("settings.no_bluetooth_desc"),
    icon_name: "bluetooth-disabled",
});
list_box.append(&row);

// Trạng thái loading
list_box.append(&create_placeholder_row(PlaceholderState::Loading));

// Trạng thái vô hiệu
let row = create_placeholder_row(PlaceholderState::Disabled {
    title_key: "settings.bluetooth_off",
    desc_key: "settings.bluetooth_off_desc",
    icon_name: "bluetooth-disabled",
});
```

---

## 4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Danh sách Settings phải có placeholder cho cả 3 trạng thái khi phù hợp |
| DO | Truyền **i18n key**, không truyền chuỗi đã dịch |
| DO | Icon dùng `icon_name` chuẩn của hệ thống icon |
| DO NOT | Không tự dựng placeholder tay với layout khác (phá vỡ đồng nhất) |
