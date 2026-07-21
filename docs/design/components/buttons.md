# Thiết kế Component: Nút bấm (`buttons.md`)

Tài liệu này đặc tả chi tiết hướng thiết kế cho tất cả các loại Nút bấm (Buttons) trong ứng dụng.

---

## 📌 1. Ý tưởng & Định hướng Thiết kế (Design Concept)

Nút bấm được thiết kế theo phong cách **Khoang nhộng bo tròn hiện đại (Modern Rounded Pill Button)** với độ tương phản thị giác rõ ràng để định hướng hành động chính và hành động phụ của người dùng một cách hiệu quả.

Các nhóm nút bấm bao gồm:
1. **Primary Button (`Generate`)**: Nút bấm quan trọng nhất đặt ở mép phải thanh nhập liệu.
2. **Secondary Button (`Upgrade now`)**: Nút kêu gọi hành động nâng cấp dịch vụ trên Header.
3. **Icon Buttons (`+`, Theme Toggle)**: Các nút dạng tròn hoặc vuông bo góc chứa biểu tượng đơn.

---

## 🎨 2. Phân loại & Đặc tả Trực quan (Button Variants)

### 2.1. Nút bấm Chức năng chính (Primary Generate Button)
- **Tông màu**: Nền màu đen tuyền tương phản cao hoàn toàn với nền trắng của thanh nhập liệu.
- **Chữ**: Màu trắng sáng, font chữ đậm vừa (Semi-bold).
- **Kiểu dáng**: Bo góc dạng khoang nhộng tròn tuyệt đối ở 2 đầu.

### 2.2. Nút bấm Nâng cấp (Secondary Upgrade Button)
- **Tông màu**: Nền màu đen tuyền.
- **Biểu tượng & Chữ**: Tích hợp icon kim cương/trái tim tinh tế đi kèm dòng chữ `"Upgrade now"`.

### 2.3. Nút bấm Biểu tượng (Icon Buttons)
- **Nút Tạo mới Session (`+`)**: Hình tròn màu trắng đặt trên thanh Session bar.
- **Nút Chuyển đổi Chế độ (`Theme Toggle`)**: Hình tròn nhỏ chứa icon Mặt trời màu đen mờ.

---

## 👆 3. Trạng thái Tương tác (UX States)

- **Trạng thái Thường (Normal)**: Nút hiển thị màu sắc chuẩn, sắc nét.
- **Trạng thái Hover**: Nền nút chuyển màu nhạt hơn 10%, nút nảy nhẹ tạo cảm giác phản hồi tích cực.
- **Trạng thái Click (Pressed)**: Nút hơi co nhẹ lại (Scale down) thể hiện sự nhấp bấm vật lý.
- **Trạng thái Vô hiệu hóa (Disabled)**: Nút chuyển sang tông xám mờ đục, không nhận thao tác click.
