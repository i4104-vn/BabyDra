# Hướng dẫn Thiết kế: Navbar (Header)

---

## 1. Bố cục tổng thể

Chia header thành 3 vùng ngang: trái (logo), giữa (nav pill), phải (tiện ích cá nhân). Dùng Flexbox với `justify-content: space-between` và `align-items: center`.

Nền header trong suốt, không viền dưới, hòa vào phông nền ứng dụng.

---

## 2. Cách tạo Logo (vùng trái)

Đặt icon biểu tượng thương hiệu ở góc trái. Dùng màu `text-primary` cho icon. Kích thước khoảng `28px` - `32px`.

---

## 3. Cách tạo Center Nav Pill (vùng giữa)

Tạo khung chứa nằm ngang, đặt `border-radius: 9999px`. Nền dùng `surface` token với `blur: 24` (Glassmorphism). Viền dùng `border` token.

Bên trong xếp 5 icon danh mục ngang đều nhau. Mỗi icon dùng màu `text-secondary` (trạng thái không active).

- **Icon active**: đổi màu sang `text-primary`, thêm nền tròn nhỏ `hover-bg` phía sau icon.
- **Hover**: đổi icon sang `text-primary`, `transition: all 200ms ease`. Không dùng `translateY` hay `scale`.

---

## 4. Cách tạo nhóm tiện ích (vùng phải)

Xếp ngang từ trái sang phải: Theme Toggle -> Share -> Credit Meter -> Upgrade -> Avatar.

- **Theme Toggle**: nút `border-radius: 50%`, icon Mặt trời (Light) / Mặt trăng (Dark).
- **Nút Share**: tạo theo hướng dẫn `buttons.md` mục 2.2 (Share Pill Button).
- **Credit Meter**: khung `border-radius: 9999px`, nền mờ nhẹ, bên trong chứa vòng tròn tiến trình (xem `progress.md`) và nhãn chữ "18% Daily Credits".
- **Nút Upgrade**: tạo theo hướng dẫn `buttons.md` mục 2.3 (Secondary Upgrade).
- **Avatar**: đặt `border-radius: 50%`. Bọc ngoài dùng `background: conic-gradient(#3b82f6, #f472b6, #fbbf24, #3b82f6)` để tạo Rainbow Gradient Ring. Khi click mở dropdown (xem `dropdowns.md`).
