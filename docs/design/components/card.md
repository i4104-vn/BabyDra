# Thiết kế Component: Khung thẻ Card (`card.md`)

Tài liệu này đặc tả chi tiết hướng thiết kế cho các Khung thẻ chứa nội dung (Card Containers) trên giao diện.

---

## 📌 1. Ý tưởng & Định hướng Thiết kế (Design Concept)

Các thẻ **Card** đóng vai trò đóng gói và phân định ranh giới không gian cho từng nội dung hình ảnh hoặc cụm tính năng. Định hướng thiết kế nhất quán với nguyên tắc bo góc lớn, đổ bóng mềm và nền màu trắng tuyền để làm nổi bật tác phẩm nghệ thuật.

---

## 🎨 2. Phân loại các Khung Thẻ Card

1. **Khung Xem Ảnh chính (Main Preview Card)**:
   - Khung hình trung tâm lớn nhất, bo góc cực đại (`24px`).
   - Đổ bóng phân tầng giúp tách biệt tác phẩm chính khỏi bề mặt nền ứng dụng.
2. **Khung Ô ảnh Biến thể (Variation Cards)**:
   - Các ô hình vuông bo góc vừa (`16px`), xếp thành cột bên phải.
   - Thẻ đang được chọn có viền màu đen đậm sắc nét bao quanh.
3. **Khung Ảnh Tham chiếu (Reference Image Card)**:
   - Khung nằm ở cột bên trái góc nhìn fisheye lens, tỷ lệ chữ nhật ngang.
4. **Khung Nhập liệu Nổi (Input Floating Card)**:
   - Khung nổi hình chữ nhật bo tròn chứa cụm nhập prompt và thanh công cụ.

---

## 👆 3. Trạng thái Trực quan (Visual States)

- **Unselected Card**: Mặt phẳng trắng mịn, viền mờ hoặc không viền.
- **Selected Active Card**: Xuất hiện đường viền bao quanh màu đen đậm 2.5px nổi bật.
- **Hover Card**: Thẻ nhấc nhẹ lên khỏi bề mặt phông nền.
