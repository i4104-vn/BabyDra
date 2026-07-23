# Hướng dẫn Thiết kế: Navs & Tabs

---

## 1. Cách tạo Center Nav Pill

Tạo khung ngang đặt `border-radius: 9999px`. Nền dùng `surface` token với blur `24` (Glassmorphism). Viền dùng `border` token.

Bên trong xếp 5 icon danh mục ngang đều (Home, Chat, Gallery, Video, Files). Icon dùng `text-secondary`.

- **Tab active**: icon đổi sang `text-primary`, thêm nền tròn nhỏ `hover-bg` phía sau.
- **Hover**: đổi icon sang `text-primary`, `transition: all 200ms ease`. Không dùng `translateY` hay `scale`.

---

## 2. Cách tạo Session Sidebar (thanh dọc bên phải)

Tạo thanh đứng `border-radius: 9999px` (dạng pill dọc). Nền trắng `#ffffff` (Light) hoặc `surface` (Dark). Đặt cố định ở rìa phải màn hình.

### Nút tạo mới (`+`)

Đặt ở đầu trên cùng. `border-radius: 50%`, icon dấu cộng căn giữa. Hover: đổi nền sang `hover-bg`.

### Danh sách Avatar phiên

Xếp dọc bên dưới nút `+`. Mỗi avatar dùng `border-radius: 50%`.

- **Phiên đang mở**: thêm `border: 2px solid #3b82f6` (accent).
- **Hover**: hiển thị tooltip tên dự án (xem `tooltips.md`).

---

## 3. Sidebar Điều hướng Chuẩn (Shared Sidebar)

Sidebar điều hướng được sử dụng thống nhất.

- **Khung chứa (`.sidebar`)**: Dùng `ScrolledWindow` hoặc `Box` với bo góc `14px`, nền bán trong suốt, viền mờ `1px solid rgba(255, 255, 255, 0.06)`, chiều rộng cố định `180px` - `220px`.
- **Mục danh mục (`.sidebar-item`)**: Nút bấm kết hợp Icon (18px) và nhãn văn bản (Label) xếp ngang.
- **Trạng thái Active (`.sidebar-item.active-nav`)**: Nền xanh dương trong suốt `rgba(59, 130, 246, 0.15)`, chữ và icon đổi sang màu Accent `#60a5fa` (`#3b82f6` đối với Light Mode).
- **Vị trí stylesheet**: Được lưu trữ chung tại `libs/babydra-utils/src/styles/{dark,light}/shared/sidebar.css`.

