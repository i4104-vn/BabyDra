# Thiết kế Component: Panel Ngữ cảnh & Prompt (`prompt_panel.md`)

Tài liệu này đặc tả chi tiết hướng thiết kế cho Bảng thông tin Prompt và ảnh tham chiếu ở cột bên trái màn hình.

---

## 📌 1. Ý tưởng & Định hướng Thiết kế (Design Concept)

**Prompt Panel** đóng vai trò là nơi hiển thị nguồn cảm hứng và thông tin ngữ cảnh tạo ra tác phẩm nghệ thuật. Thiết kế hướng tới việc trình bày thông tin một cách mạch lạc, trực quan, kết hợp giữa hình ảnh tham chiếu, các thẻ từ khóa và mô tả chi tiết.

Bố cục bao gồm 3 phần chính từ trên xuống dưới:
1. **Thẻ Ảnh tham chiếu (Reference Image Card)**: Thể hiện hình ảnh nguồn hoặc góc xem kỹ thuật.
2. **Huy hiệu Biểu tượng (Floating Palette Badge)**: Huy hiệu biểu tượng bảng màu đính nổi ở góc ảnh tham chiếu.
3. **Dãy Tag từ khóa (Prompt Keyword Chips)**: Các thẻ nhãn chứa từ khóa cốt lõi của Prompt.
4. **Đoạn mô tả Ngữ cảnh (Narrative Text Paragraph)**: Đoạn văn bản tự do thuyết minh về phong cách nhiếp ảnh và thẩm mỹ.

---

## 🎨 2. Đặc tả Trực quan & Bố cục (Visual & Layout Specs)

### 2.1. Thẻ Ảnh tham chiếu (Reference Image Card)
- **Tỷ lệ Khung hình**: Tỷ lệ chữ nhật ngang dạng góc nhìn ống kính fisheye (`16:9`).
- **Kiểu dáng**: Bo góc mềm mại, độ nổi nhẹ so với phông nền, hình ảnh hiển thị rõ nét với ánh sáng ấm.
- **Huy hiệu Palette đính kèm**: Một hình tròn nhỏ màu trắng tinh khiết đặt nổi ở góc dưới bên trái của bức ảnh tham chiếu, bên trong chứa biểu tượng bảng màu đen.

### 2.2. Dãy Tag từ khóa (Prompt Keyword Chips)
- **Dạng trình bày**: Xếp nối tiếp nhau từ trái sang phải, tự động xuống dòng khi hết không gian.
- **Phân loại thẻ**:
  - **Thẻ văn bản tĩnh (`Create a`)**: Chữ nét thường màu xám tối, không có khung nền bao quanh.
  - **Thẻ từ khóa chính (`dapper fox`, `in a green suit`)**: Được bao bọc trong khung nhộng bo tròn (Pill shape), nền màu xám kem mờ nhạt, chữ nét đậm nổi bật giúp người dùng nhận biết ngay đối tượng chính và trang phục.

### 2.3. Đoạn mô tả Ngữ cảnh (Narrative Text Block)
- **Định dạng văn bản**: Đoạn văn bản lùi lề căn trái, cỡ chữ nhỏ gọn gàng, khoảng cách giữa các dòng vừa phải.
- **Tông màu chữ**: Màu xám trung tính, không quá đậm để tránh tranh chấp thị giác với bức ảnh chính ở trung tâm.

---

## 👆 3. Trạng thái Tương tác & Trải nghiệm (Interaction & UX States)

- **Trạng thái Thường (Default)**: Hiển thị đầy đủ thông tin với độ tương phản dịu mắt.
- **Trạng thái Hover vào Tag Chip**: Nền thẻ chip chuyển sang tông xám đậm hơn một chút, con trỏ chuột chuyển thành dạng bàn tay gợi ý khả năng click để lọc hoặc chỉnh sửa từ khóa.
- **Trạng thái Hover vào Ảnh tham chiếu**: Ảnh phóng to nhẹ (Zoom in 2%) tạo cảm giác sống động.
