# Hướng dẫn Thiết kế: Tooltips

---

## 1. Cách tạo Tooltip

Đặt `border-radius: 8px`. Padding `4px 8px` - `6px 12px`.

- **Dark Theme**: nền `rgba(14, 14, 18, 0.95)`, chữ trắng, viền `rgba(255, 255, 255, 0.10)`.
- **Light Theme**: nền `rgba(255, 255, 255, 0.98)`, chữ `text-primary`, viền `rgba(0, 0, 0, 0.06)`.

Thêm shadow nhẹ. Chữ `font-size: 12px`, `font-weight: 400`.

---

## 2. Cách hiển thị

Hiện sau `300ms` hover (delay). Dùng hiệu ứng mờ hiện dần: `opacity: 0` -> `1`, `transition: opacity 150ms ease`. Đặt vị trí tự động phía trên hoặc phía dưới phần tử kích hoạt.
