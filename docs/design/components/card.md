# Hướng dẫn Thiết kế: Cards

---

## 1. Nguyên tắc chung

Mọi card đều dùng nền `surface` token, viền `border` token, shadow hệ thống và `-gtk-background-blur: 24` khi cần Acrylic. Không dùng bóng đổ tự ý ngoài token.

Khi hover lên card: chỉ đổi `background-color` hoặc `border-color`, `transition: all 200ms ease`. Không dùng `translateY`, `scale`.

---

## 2. Cách tạo từng loại card

### 2.1. Khung Ảnh chính (Main Preview Card)

Đặt `border-radius: 24px`. Thêm shadow hệ thống. Kích thước lớn, chiếm trung tâm màn hình. Tỷ lệ hiển thị tự động theo lựa chọn (1:1, 4:3, 3:4).

### 2.2. Khung Popover Dropdown

Đặt `border-radius: 20px`. Nền `surface`, viền `border`, shadow hệ thống, blur `24`. Padding `12px` - `16px`. Chi tiết bên trong xem `dropdowns.md`.

### 2.3. Khung Ảnh Biến thể (Variation Card)

Đặt `border-radius: 16px`. Hình vuông, xếp cột dọc 4 ô bên phải khung ảnh chính.

- **Ô đang chọn (active)**: thêm `border: 2px solid #3b82f6`.
- **Ô chưa chọn**: không viền màu nhấn, chỉ hiển thị ảnh bình thường.
- **Hover**: đậm viền nhẹ hoặc tăng brightness, `transition: all 200ms ease`. Không dùng `translateY`.

### 2.4. Khung Ảnh Tham chiếu (Reference Image Card)

Đặt `border-radius: 16px`. Tỷ lệ ngang `16:9` (fisheye lens view). Đặt Floating Icon Badge ở góc dưới trái (xem `badge.md` mục 4).

### 2.5. Khung Nhập liệu Nổi (Input Floating Shell)

Đặt `border-radius: 24px`. Nền `surface`, viền `border`, shadow hệ thống. Chi tiết bên trong xem `input_group.md`.
