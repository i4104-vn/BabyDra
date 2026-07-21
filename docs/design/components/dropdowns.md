# Thiết kế Component: Dropdowns & Popovers (`dropdowns.md`)

Tài liệu này đặc tả chi tiết hướng thiết kế cho Menu xổ nổi (Popover Dropdown) lựa chọn các thông số cấu hình.

---

## 📌 1. Định hướng Thiết kế (Design Concept)

khi nhấp vào các chip thông số ở thanh công cụ dưới (`4:3`, `4K`, `Style`), ứng dụng sẽ hiển thị một Popover Menu nhỏ xổ lên ngay phía trên nút bấm.

---

## 🎨 2. Phân loại & Đặc tả Trực quan

- **Màu nền & Bo góc**: Nền trắng tuyền, bo góc tròn dịu (`14px`).
- **Đổ bóng Nổi (Elevation)**: Đổ bóng nổi lơ lửng rộng giúp tách rời menu khỏi các thành phần phía dưới.
- **Danh sách Lựa chọn (Menu Items)**: Các dòng tùy chọn nằm ngang gồm icon minh họa và nhãn chữ ngắn gọn.

---

## 👆 3. Trạng thái Tương tác (UX States)

- **Hover dòng tùy chọn**: Nền dòng chuyển sang xám mờ nhạt.
- **Dòng đang được chọn (Selected Item)**: Có biểu tượng dấu tích xanh/đen hoặc nét chữ đậm nổi bật.
