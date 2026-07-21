# Thiết kế Component: Vùng Hiển thị Ảnh & Cột Biến thể (`preview_panel.md`)

Tài liệu này đặc tả chi tiết hướng thiết kế cho Vùng hiển thị ảnh kết quả chính (Center Output View) và Cột các phương án biến thể (Right Variations Feed).

---

## 📌 1. Ý tưởng & Định hướng Thiết kế (Design Concept)

Đây là khu vực trọng tâm nhất của ứng dụng, nơi thể hiện kết quả của quá trình sinh ảnh AI. Thiết kế nhằm tôn vinh tác phẩm nghệ thuật bằng cách dành tối đa diện tích hiển thị, bo góc lớn mềm mại và đi kèm cột biến thể bên phải để người dùng dễ dàng so sánh các phương án.

Bố cục gồm 2 vùng liên kết với nhau:
1. **Khung Ảnh chính (Main Preview Card)**: Chiếm vị trí trung tâm màn hình, thể hiện bức ảnh chi tiết sắc nét với kích thước lớn.
2. **Cột Biến thể (Variations Feed Column)**: Dãy 4 ô xem trước ảnh vuông xếp theo chiều dọc ở mép phải của khung ảnh chính.

---

## 🎨 2. Đặc tả Trực quan & Bố cục (Visual & Layout Specs)

### 2.1. Khung Ảnh chính (Main Preview Card)
- **Kích thước & Bo góc**: Kích thước lớn, bo góc rộng (`24px`), đường viền mượt mà hòa hợp với tổng thể phông nền ứng dụng.
- **Đổ bóng & Phân tầng**: Đổ bóng đổ dịu xung quanh khung hình, tạo cảm giác bức ảnh nổi nhẹ phía trên phông nền màu kem hồng.
- **Tỷ lệ hiển thị**: Tự động linh hoạt theo tùy chỉnh của người dùng (Tỷ lệ vuông `1:1`, chữ nhật ngang `4:3` hoặc chữ nhật dọc `3:4`).

### 2.2. Cột Biến thể (Variations Feed Column)
- **Hình dáng mỗi ô biến thể**: Ô vuông nhỏ bo góc vừa (`16px`), xếp thành một cột dọc 4 ô đều đặn.
- **Chỉ báo Ô đang chọn (Active Selection Indicator)**:
  - Ô biến thể đại diện cho bức ảnh đang hiển thị ở Khung ảnh chính sẽ có một **đường viền màu đen đậm sắc nét** bao quanh.
  - Hiệu ứng này giúp người dùng lập tức biết bức ảnh nào đang được xem chi tiết.
- **Các ô chưa chọn (Inactive Variations)**: Không có đường viền đen, hiển thị hình ảnh bình thường với độ bo góc đồng nhất.

---

## 👆 3. Trạng thái Tương tác & Trải nghiệm (UX States)

- **Hover vào ô biến thể phụ**: Ô ảnh hơi nhấc nhẹ lên (Translate Y) và độ sáng tăng nhẹ, báo hiệu người dùng có thể nhấp chuột để chuyển đổi ảnh trình chiếu.
- **Click chọn ô biến thể**: Viền đen đậm lập tức di chuyển sang ô mới được chọn, hình ảnh ở Khung xem chính chuyển đổi mượt mà sang bức ảnh tương ứng.
- **Trạng thái đang sinh ảnh mới (Loading State)**: Khung ảnh chính hiển thị nền mờ nhấp nháy nhịp nhàng (Pulse effect) báo hiệu hệ thống đang xử lý tác phẩm.
