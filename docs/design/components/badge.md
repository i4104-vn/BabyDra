# Component: Badge

Tài liệu quy định cách sử dụng, kiểu dáng, và mã nguồn chuẩn cho **Badge** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `kits/babydra-ui-kit/src/components/badge/mod.rs`

---

## 1. Tổng quan

Badge trong BabyDra là thành phần nhỏ dùng để:

- Hiển thị trạng thái ngắn gọn (thành công / thông thường).
- Đóng khung một icon thành hình tròn kính mờ (icon badge) — dùng làm avatar, biểu tượng chức năng.

Có **2 loại badge** chính, mỗi loại phục vụ một mục đích riêng:

| Loại | Hàm tạo | Công dụng |
| :--- | :--- | :--- |
| Status Badge | `create_status_badge(text, is_success)` | Label trạng thái (thành công / mô tả) |
| Icon Badge | `create_icon_badge(icon_name, icon_size, is_small)` | Vòng tròn kính mờ chứa icon (44px hoặc 34px) |

---

## 2. API

### 2.1. Status Badge

```rust
pub fn create_status_badge(text: &str, is_success: bool) -> gtk4::Label
```

- `is_success = true` → class `success-text` (màu xanh lá thành công).
- `is_success = false` → class `settings-desc` (màu mô tả phụ).
- Luôn `set_valign(Align::Center)` — nằm giữa theo chiều dọc.

### 2.2. Icon Badge

```rust
pub fn create_icon_badge(icon_name: &str, icon_size: i32, is_small: bool) -> gtk4::Box
```

| Tham số | Ý nghĩa |
| :--- | :--- |
| `icon_name` | Tên icon (được resolve qua `crate::ui::icon::get_icon`) |
| `icon_size` | Kích thước icon (pixel) |
| `is_small` | `false` → badge 44×44, căn giữa; `true` → badge 34×34 (`blue-icon-badge-sm`), căn trái |

**CSS classes:**

| Class | Kích thước | Mô tả |
| :--- | :--- | :--- |
| `blue-icon-badge` | `min-width/height: 44px` | Badge tròn 44px, căn giữa |
| `blue-icon-badge-sm` | `min-width/height: 34px` | Badge tròn 34px, căn trái |

### 2.3. Style (dark theme)

```css
.blue-icon-badge {
    background-color: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-top: 1px solid rgba(255, 255, 255, 0.22);   /* bevel ánh sáng trên */
    color: rgba(255, 255, 255, 0.9);
    border-radius: 9999px;                               /* tròn hoàn toàn */
    min-width: 44px;
    min-height: 44px;
}
```

> [!NOTE]
> Vị trí CSS: `themes/babydra-default/css/dark.css` và `themes/babydra-default/css/light.css` (dark/light tương ứng).

---

## 3. Ví dụ sử dụng

```rust
// Badge trạng thái thành công
let ok_badge = create_status_badge("Connected", true);

// Icon badge 44px căn giữa (dùng cho placeholder, avatar)
let badge = create_icon_badge("folder-open", 24, false);

// Icon badge nhỏ 34px căn trái (dùng trong dòng danh sách)
let small = create_icon_badge("wifi", 16, true);
```

---

## 4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Icon badge phải dùng `create_icon_badge` để đảm bảo đồng nhất hình tròn kính mờ |
| DO | Status badge dùng `create_status_badge` thay vì tạo `Label` thủ công |
| DO NOT | Không tùy ý đổi màu badge bằng class mới — màu chỉ nằm trong `settings.css` dark/light |
| DO NOT | Không dùng badge cho nội dung dài (nhiều hơn vài từ) |
