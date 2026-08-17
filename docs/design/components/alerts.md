# Component: Placeholder Message (formerly Alerts)

Tài liệu quy định cách sử dụng và mã nguồn chuẩn cho **Message / Placeholder label** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `libs/babydra-utils/src/components/placeholder/mod.rs` (moved from `alerts/` — Phase 2 T2.4)

---

## 1. Tổng quan

Module `alerts` đã được **gộp vào `placeholder`** trong Phase 2 (T2.4) vì chỉ chứa một helper tạo label chiếm chỗ trùng với vai trò của placeholder. Giờ chỉ còn **một nơi** chứa tất cả trạng thái placeholder.

---

## 2. API

```rust
#[deprecated(note = "use create_placeholder_row with PlaceholderState instead")]
pub fn create_placeholder_message(text: &str) -> gtk4::Label
```

- Label class `settings-desc` (màu mô tả phụ).
- Margin dọc 20px trên/dưới.
- **Deprecated** — dùng `create_placeholder_row` (xem [placeholder.md](./placeholder.md)) cho trạng thái ListBox.

---

## 3. Ví dụ sử dụng

```rust
// Ưu tiên dùng placeholder row cho trạng thái trong ListBox
let row = create_placeholder_row(PlaceholderState::Empty {
    title_key: "settings.no_results",
    desc_key: None,
    icon_name: "search",
});
```

---

## 4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Trạng thái vùng nội dung trong ListBox dùng `create_placeholder_row` |
| DO | Text đi qua i18n trước khi truyền vào |
| DO NOT | Không gọi `create_placeholder_message` (deprecated) trong code mới |
| DO NOT | Không tạo module alert mới — mọi placeholder gom về `placeholder/` |
