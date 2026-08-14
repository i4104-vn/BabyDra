# Component: Alerts & Placeholder Message

Tài liệu quy định cách sử dụng và mã nguồn chuẩn cho **Message / Alert** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `libs/babydra-utils/src/components/alerts/mod.rs`

---

## 1. Tổng quan

Module `alerts` hiện cung cấp một helper tạo label thông báo chiếm chỗ (placeholder message) — dùng cho các vùng nội dung tạm thời chưa có dữ liệu.

---

## 2. API

```rust
pub fn create_placeholder_message(text: &str) -> gtk4::Label
```

- Label class `settings-desc` (màu mô tả phụ).
- Margin dọc 20px trên/dưới.

> [!NOTE]
> Khác với [placeholder.md](./placeholder.md) (dòng trạng thái trong ListBox), helper này tạo **label đơn lẻ** cho vùng nội dung.

---

## 3. Ví dụ sử dụng

```rust
let msg = create_placeholder_message("No results found");
content_box.append(&msg);
```

---

## 4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Thông báo vùng nội dung trống dùng `create_placeholder_message` |
| DO | Text đi qua i18n trước khi truyền vào |
| DO NOT | Không dùng label này cho trạng thái lỗi nghiêm trọng (chưa có error variant) |
