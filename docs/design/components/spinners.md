# Component: Spinners & Loading

Tài liệu quy định cách sử dụng, kiểu dáng, và mã nguồn chuẩn cho **Spinner** và trạng thái **Loading** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `libs/babydra-utils/src/components/spinners/mod.rs`

---

## 1. Tổng quan

Spinner dùng cho trạng thái chờ ngắn (đang tải dữ liệu, đang xử lý). Xem thêm triết lý loading tại [motion.md](../motion.md) — ưu tiên skeleton cho vùng nội dung lớn, spinner cho icon/button.

---

## 2. API

```rust
pub fn create_spinner(size: i32) -> gtk4::Spinner
pub fn create_loading_box(text: &str) -> gtk4::Box
```

| Hàm | Mô tả |
| :--- | :--- |
| `create_spinner(size)` | Spinner kích thước `size×size`, tự động `start()` |
| `create_loading_box(text)` | Hàng ngang: spinner 16px + label (class `settings-desc`) — căn giữa |

---

## 3. Ví dụ sử dụng

```rust
// Spinner đơn
let spinner = create_spinner(24);
container.append(&spinner);

// Hàng loading có nhãn
let loading = create_loading_box("Scanning networks...");
container.append(&loading);
```

---

## 4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Dùng `create_spinner` — không tạo `gtk4::Spinner` thô quên `start()` |
| DO | Loading có text dùng `create_loading_box` |
| DO | Vùng nội dung lớn ưu tiên skeleton (xem motion.md) thay vì spinner |
| DO NOT | Không đặt nhiều spinner cùng lúc trong một màn hình |
