# Hướng dẫn Thiết kế: Spinners & Loaders

---

## 1. Cách tạo Skeleton Pulse Loading

Khi đang sinh ảnh, đặt khung ảnh chính sang trạng thái loading:

- Dùng nền xám mờ nhẹ.
- Áp dụng animation: `animation: pulse 1.2s ease-in-out infinite` (nền mờ dần sáng lên liên tục).
- Không hiển thị nội dung ảnh trong lúc loading.

---

## 2. Cách tạo Button Spinner

Khi nút Generate đang xử lý: ẩn chữ, hiển thị vòng tròn spinner nhỏ xoay 360 độ (`animation: spin 0.8s linear infinite`).

Spinner dùng màu trắng `#ffffff` (trên nền nút tối) hoặc `accent` `#3b82f6` (trên nền nút sáng). Đường viền spinner: `border: 2px solid`, `border-top-color: transparent`.
