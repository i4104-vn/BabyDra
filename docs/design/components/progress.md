# Component: Progress

Tài liệu quy định cách sử dụng, kiểu dáng, và mã nguồn chuẩn cho **Progress Bar** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `libs/babydra-ui-kit/src/components/progress/mod.rs`

---

## 1. Tổng quan

Progress bar dùng để hiển thị tiến trình tác vụ (cài đặt, cập nhật, quét...). BabyDra có 2 helper tạo `gtk4::ProgressBar` có style chuẩn.

---

## 2. API

```rust
pub fn create_progress_bar(fraction: f64, css_class: &str) -> gtk4::ProgressBar
pub fn create_disk_progress(fraction: f64, css_class: &str) -> gtk4::ProgressBar
```

| Hàm | Class gắn sẵn | Công dụng |
| :--- | :--- | :--- |
| `create_progress_bar` | class tùy chọn | Progress bar tổng quát |
| `create_disk_progress` | `disk-progress` + class tùy chọn | Hiển thị dung lượng ổ đĩa |

- `fraction` là giá trị 0.0–1.0 (tỷ lệ hoàn thành).
- `css_class` rỗng → không gắn thêm class.

---

## 3. Style tham chiếu

Mẫu progress bar cập nhật hệ thống (`update-progress-bar` trong `settings.css`):

```css
.update-progress-bar trough {
    min-height: 8px;
    border-radius: 9999px;
    background-color: rgba(255, 255, 255, 0.1);
}

.update-progress-bar progress {
    min-height: 8px;
    border-radius: 9999px;
    background-color: #3b82f6;   /* accent chuẩn */
}
```

> [!NOTE]
> Nguyên tắc: track nền mờ (alpha thấp), phần `progress` luôn dùng màu accent `#3b82f6`, bo tròn hoàn toàn (9999px).

---

## 4. Ví dụ sử dụng

```rust
let progress = create_progress_bar(0.5, "update-progress-bar");
container.append(&progress);

// Cập nhật tiến trình
progress.set_fraction(0.75);
```

---

## 5. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Dùng `create_progress_bar` / `create_disk_progress` cho mọi thanh tiến trình |
| DO | Phần điền tiến trình phải dùng màu `#3b82f6` |
| DO | Bo tròn track và phần progress (`border-radius: 9999px`) |
| DO NOT | Không tự tạo `gtk4::ProgressBar` thô không class |
| DO NOT | Không dùng màu khác accent cho phần progress |
