# Hướng dẫn Thiết kế: Badges & Chips

---

## 1. Nguyên tắc chung

Badge và Chip đều dùng `border-radius: 9999px` (dạng khoang nhộng). Không dùng bo góc vuông.

Đặt `transition: all 200ms ease`. Khi hover chỉ đổi `background-color`, không dùng `translateY` hay `scale`.

---

## 2. Cách tạo Badge PRO

Đặt `border-radius: 9999px`, `padding: 2px 8px`. Bên trong gồm icon tia sét/bolt (bên trái) và chữ "PRO" viết hoa (`font-size: 10px` - `11px`, `font-weight: 800`).

Chọn một trong các biến thể:

| Biến thể | background | color (chữ & icon) |
| :--- | :--- | :--- |
| Hồng Pastel | `#fce7f3` | `#be185d` |
| Xanh lá Pastel | `#dcfce7` | `#15803d` |
| Xanh dương Accent | `#dbeafe` | `#1e40af` |

---

## 3. Cách tạo Keyword Chip (Prompt Token)

### Chip có khung nền (Highlight Chip)

Đặt `border-radius: 9999px`, `padding: 4px 12px`. Nền dùng `rgba(0, 0, 0, 0.05)` (Light) hoặc `rgba(255, 255, 255, 0.08)` (Dark). Chữ dùng `font-weight: 600`, màu `text-primary`.

Khi hover: đậm nền thêm 6%.
Khi được chọn (active): đổi nền sang `#1c1c1e` (Light) hoặc `accent` `#3b82f6`, chữ trắng.

### Chip không khung (Static Text)

Không đặt nền. Chữ dùng `text-secondary`, `font-weight: 400`. Dùng cho các cụm từ nối giữa các keyword chip.

---

## 4. Cách tạo Floating Icon Badge

Dùng `border-radius: 50%`, kích thước cố định (ví dụ `24px` x `24px`). Nền trắng `#ffffff`. Icon đen sẫm căn giữa. Đặt `position: absolute` vào góc dưới trái của ảnh tham chiếu.
