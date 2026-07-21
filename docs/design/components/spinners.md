# Thiết kế Component: Spinners & Loaders (`spinners.md`)

Tài liệu này đặc tả chi tiết hướng thiết kế cho các hiệu ứng phản hồi trạng thái chờ khi AI đang trong quá trình sinh ảnh.

---

## 📌 1. Định hướng Thiết kế (Design Concept)

Khi người dùng nhấn nút `Generate`, giao diện cần phản hồi ngay lập tức để người dùng biết hệ thống đang xử lý:
1. **Pulse Skeleton Loading**: Khung ảnh chính chuyển sang nền mờ nhấp nháy nhịp nhàng (Pulse effect).
2. **Button Spinner**: Nút `Generate` hiển thị icon xoay tròn nhỏ thay cho dòng chữ.

---

## 🎨 2. Phân loại & Đặc tả Trực quan

- **Hiệu ứng Skeleton Pulse**: Nền khung ảnh mờ dần và sáng lên liên tục nhẹ nhàng.
- **Biểu tượng Spinner**: Vòng tròn nhỏ xoay liên tục 360 độ màu trắng.
