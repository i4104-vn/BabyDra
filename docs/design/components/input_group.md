# Hướng dẫn Thiết kế: Input Group & Control Center

---

## 1. Bố cục tổng thể

Đặt cụm điều khiển nổi lơ lửng ở giữa phía dưới màn hình (`position: fixed` hoặc `absolute`, `bottom`, `left: 50%`, `transform: translateX(-50%)`). Chia làm 2 tầng xếp dọc bên trong.

---

## 2. Cách tạo khung nổi ngoài cùng (Outer Shell)

Đặt `border-radius: 24px`. Nền dùng `surface` token, viền `border` token, shadow hệ thống, blur `24`. Padding bên trong `12px` - `16px`.

---

## 3. Cách tạo Tầng trên (Prompt Input + Generate Button)

Bố cục ngang: ô nhập liệu bên trái + nút Generate bên phải.

- **Ô nhập liệu Prompt**: không viền, không nền riêng, phẳng hoàn toàn. Placeholder dùng `text-secondary`. Chữ gõ dùng `text-primary`.
- **Nút Generate**: tạo theo hướng dẫn `buttons.md` mục 2.1 (Primary Button). Đặt `border-radius: 9999px`, nền `accent` hoặc `#1c1c1e`.

---

## 4. Cách tạo Tầng dưới (Parameter Toolbar)

Xếp ngang các chip tùy chọn (Aspect Ratio, Quality, Style, Image prompt, Image style). Mỗi chip dùng `border-radius: 9999px`, nền mờ nhẹ, bên trong chứa icon nhỏ + nhãn chữ ngắn.

Hover: đậm nền 6% - 8%, `transition: all 200ms ease`. Không dùng `translateY` hay `scale`.

Click chip: mở Popover nhỏ ngay phía trên chip (xem `dropdowns.md`).
