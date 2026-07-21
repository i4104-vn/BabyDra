# Thiết kế Component: Navs & Tabs (`navs_tabs.md`)

Tài liệu này đặc tả chi tiết hướng thiết kế cho Thanh điều hướng danh mục chính (Center Nav Pill) và Thanh quản lý phiên làm việc dọc ở mép phải (Session Sidebar Bar).

---

## 📌 1. Ý tưởng & Định hướng Thiết kế (Design Concept)

Hệ thống điều hướng sử dụng ngôn ngữ thiết kế dạng **Khoang nhộng nổi (Pill-shaped Navigation)** nhằm tối giản các chi tiết rườm rà, tạo cảm giác di chuyển mượt mà và trực quan cho người dùng:
1. **Center Navigation Pill**: Thanh điều hướng ngang ở Header cho phép chuyển đổi giữa các góc độ làm việc chính.
2. **Right Session Sidebar Bar**: Thanh bên dọc đặt ở sát mép phải màn hình giúp người dùng chuyển đổi nhanh giữa các phiên sinh ảnh nghệ thuật khác nhau.

---

## 🎨 2. Đặc tả Trực quan & Bố cục (Visual Specs)

### 2.1. Thanh Điều hướng Trung tâm (Center Nav Pill)
- **Kiểu dáng**: Khung nhộng nằm ngang bo tròn tuyệt đối ở hai đầu.
- **Tông màu**: Khung màu xám tối mờ, các icon đại diện có tông màu trắng sáng dịu.
- **Biểu tượng**: 5 biểu tượng danh mục chính xếp đều từ trái qua phải (Trang chủ, Chat, Thư viện ảnh, Video, Thư mục tệp).

### 2.2. Thanh Quản lý Phiên bên phải (Right Session Sidebar Bar)
- **Kiểu dáng**: Một thanh đứng hình nhộng màu trắng tinh khiết đặt nổi ở rìa phải màn hình.
- **Nút tạo mới (`+`)**: Đặt ở đầu trên cùng của thanh, hiển thị biểu tượng cộng đen đơn giản để mở một phiên làm việc mới.
- **Danh sách Avatar Phiên (Session Items)**:
  - Xếp dọc bên dưới nút `+` là các ảnh đại diện tròn của các dự án hiện tại (Ví dụ: Chú cáo suit xanh, Chú gấu đội mũ, Chú tắc kè đeo kính).
  - Avatar đại diện cho phiên đang mở sẽ có chỉ báo viền hoặc hiệu ứng nổi bật.

---

## 👆 3. Trạng thái Tương tác (UX States)

- **Hover vào nút `+` trên Session Bar**: Nút đổi màu nhạt nhẹ báo hiệu hành động sẵn sàng tạo dự án mới.
- **Hover vào từng Avatar phiên**: Hiển thị hộp chú thích (Tooltip) ghi tên dự án tương ứng.
- **Switch Session**: Nhấp chuột vào bất kỳ Avatar nào để chuyển toàn bộ không gian làm việc sang dự án sinh ảnh đó.
