# Thiết kế Component: Cụm Nhập liệu & Control Center (`input_group.md`)

Tài liệu này đặc tả chi tiết hướng thiết kế cho Cụm khung nhập liệu Prompt chính và Thanh công cụ điều chỉnh thông số ở mép dưới màn hình.

---

## 📌 1. Ý tưởng & Định hướng Thiết kế (Design Concept)

**Input Group** là trung tâm điều khiển chính tương tác với ứng dụng. Thiết kế áp dụng phong cách **Khung nổi (Floating Card Panel)** đặt lơ lửng ở giữa phía dưới của màn hình làm việc.

Cụm điều khiển được chia thành 2 tầng phân cấp rõ ràng:
1. **Tầng trên (Dòng Nhập liệu & Nút Sinh ảnh)**: Nơi người dùng gõ ý tưởng văn bản và nhấn nút `Generate` màu đen nổi bật.
2. **Tầng dưới (Thanh Công cụ Thông số)**: Dãy các nút dạng chip cho phép nhanh chóng chọn tỷ lệ khung hình, độ phân giải, phong cách nghệ thuật và ảnh nguồn.

---

## 🎨 2. Đặc tả Trực quan & Bố cục (Visual & Layout Specs)

### 2.1. Khung chứa Nổi ngoài cùng (Outer Floating Shell)
- **Hình dáng**: Hộp chữ nhật bo góc tròn lớn (`24px`), bề mặt màu trắng tuyền.
- **Đổ bóng Nổi (Floating Elevation)**: Sử dụng độ đổ bóng rộng và sâu bên dưới khung, tạo hiệu ứng thị giác lơ lửng trên phông nền chính.
- **Bố cục bên trong**: Khoảng cách căn lề trong vừa vặn, phân tách 2 tầng bằng khoảng trống tự nhiên.

### 2.2. Tầng Nhập liệu Prompt & Nút Generate
- **Ô văn bản Prompt (Prompt Text Field)**:
  - Nằm ở bên trái tầng trên, giao diện không đường viền phẳng hoàn toàn.
  - Chữ gợi ý mờ (Placeholder): `"What do you want to see?"` sử dụng tông màu xám nhạt dịu mắt.
- **Nút bấm Generate**:
  - Đặt ở mép phải tầng trên, có kiểu dáng khoang nhộng (Pill shape) màu đen tuyền tương phản cao.
  - Văn bản chữ màu trắng tinh khiết, nét chữ đậm nét giúp nổi bật hành động quan trọng nhất.

### 2.3. Tầng Thanh Công cụ Thông số (Parameter Toolbar Row)
- **Các chip tính năng (Setting Chips)**:
  - Xếp thành hàng ngang gồm các tùy chọn: Aspect Ratio `4:3`, Quality `4K`, `Style`, `Image prompt`, `Image style`.
  - Mỗi chip có khung nền màu xám kem mờ nhạt, bo góc khoang nhộng tròn.
  - Bên trong tích hợp biểu tượng minh họa nhỏ (như icon khung hình, icon kim cương, icon đồng hồ, icon nét vẽ) đi kèm nhãn chữ ngắn gọn.

---

## 👆 3. Trạng thái Tương tác & Trải nghiệm (UX States)

- **Trạng thái Focus vào ô nhập liệu**: Dòng chữ mờ ẩn đi khi người dùng bắt đầu gõ, chữ gõ hiển thị nét rõ ràng màu đen.
- **Hover vào nút Generate**: Nút chuyển sang tông đen nhạt hơn một chút kèm độ nảy nhẹ, báo hiệu sẵn sàng kích hoạt lệnh sinh ảnh.
- **Click vào chip công cụ thông số**: Mở một menu dạng Popover nhỏ xổ lên ngay phía trên chip tương ứng để người dùng lựa chọn các tùy chọn con.
