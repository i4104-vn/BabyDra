# Hướng dẫn Thiết kế: Dropdowns & Popovers

---

## 1. Nguyên tắc chung

Dropdown/Popover là khung nổi lơ lửng, xuất hiện khi click vào avatar hoặc chip thông số. Luôn đặt `border-radius: 20px`, áp dụng shadow và viền theo token hệ thống.

---

## 2. Cách tạo khung chứa (Container)

Đặt `border-radius: 20px`. Nền dùng token `surface` tương ứng theo theme. Viền dùng token `border`. Thêm `-gtk-background-blur: 24` để tạo Acrylic. Đổ bóng dùng token shadow (Light: `0 10px 30px rgba(0, 0, 0, 0.08)`, Dark: `0 10px 30px rgba(0, 0, 0, 0.35)`).

Padding bên trong: `12px` - `16px`.

---

## 3. Cách tạo khối Profile Header

Đặt ở vùng trên cùng của dropdown khi mở từ avatar:

- **Avatar**: Dùng `border-radius: 50%`. Bọc ngoài một khung viền dùng `background: conic-gradient(#3b82f6, #f472b6, #fbbf24, #3b82f6)` để tạo rainbow ring. Avatar nằm bên trong trên nền `surface`.
- **Tên người dùng**: `font-size: 14px` - `15px`, `font-weight: 700`, màu `text-primary`.
- **Email**: `font-size: 12px`, `font-weight: 400`, màu `text-secondary`.

---

## 4. Cách tạo dòng menu (Menu Row)

Mỗi dòng đặt `padding: 8px 12px`, `border-radius: 10px`. Bố cục ngang: icon bên trái (kích thước `16px` - `20px`) + nhãn chữ `font-size: 13px` - `14px`, `font-weight: 500` + thành phần phụ bên phải (badge hoặc icon mở rộng).

Khi hover: đổi nền sang `hover-bg` token, `transition: all 200ms ease`. Không dùng `translateY` hay `scale`.

Dòng đang active: đặt nền `hover-bg` cố định, chữ và icon chuyển sang `text-primary` đậm rõ. Thêm dấu checkmark bên trái hoặc dùng nền `accent` (`#3b82f6`) cho các tùy chọn thông số.

---

## 5. Cách tạo badge PRO trong menu

Dùng `border-radius: 9999px`, `padding: 2px 8px`. Chữ "PRO" viết hoa, `font-size: 10px` - `11px`, `font-weight: 800`. Icon tia sét/bolt đặt bên trái chữ.

Biến thể hồng: nền `#fce7f3`, chữ/icon `#be185d`.
Biến thể xanh lá: nền `#dcfce7`, chữ/icon `#15803d`.

---

## 6. Đường phân cách

Dùng `border-bottom: 1px solid` với token `separator` để phân chia nhóm tài khoản và nhóm trợ giúp/đăng xuất. Đặt `margin: 4px 0`.
