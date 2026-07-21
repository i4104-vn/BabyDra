# Thiết kế Component: Thanh Header Điều hướng (`navbar.md`)

Tài liệu này đặc tả chi tiết hướng thiết kế cho Thanh tiêu đề trên cùng (Header Navbar) của ứng dụng.

---

## 📌 1. Ý tưởng & Định hướng Thiết kế (Design Concept)

**Header Navbar** đóng vai trò định danh thương hiệu, điều hướng hệ thống chính và hiển thị thông tin tài khoản người dùng. Giao diện được thiết kế theo phong cách phẳng, tràn lề không đường gờ thô cứng, tích hợp mượt mà vào phông nền ứng dụng.

Bố cục được chia thành 3 nhóm khu vực chính:
1. **Khu vực Bên trái**: Logo biểu tượng đại diện thương hiệu.
2. **Khu vực Trung tâm**: Thanh điều hướng các danh mục chính dạng viên thuốc nổi (Center Nav Pill).
3. **Khu vực Bên phải**: Nhóm tiện ích cá nhân bao gồm nút đổi giao diện sáng/tối, bộ đếm phần trăm Credit ngày, nút Nâng cấp nhanh và Ảnh đại diện tài khoản.

---

## 🎨 2. Đặc tả Trực quan & Bố cục (Visual Specs)

### 2.1. Logo Thương hiệu (Brand Logo)
- **Vị trí**: Đặt tại góc trái trên cùng.
- **Hình dáng**: Icon biểu tượng khuôn mặt góc cạnh phong cách tối giản màu đen tuyền.

### 2.2. Thanh Điều hướng Trung tâm (Center Nav Pill)
- **Hình dáng**: Một khối khoang nhộng bo tròn nằm cân bằng ở chính giữa thanh Header.
- **Nền & Hiệu ứng**: Khối xám đen sẫm màu mờ có hiệu ứng làm mờ phông nền phía sau (Glassmorphism blur).
- **Các biểu tượng danh mục**: Chứa 5 biểu tượng trắng mờ đại diện cho Trang chủ, Trò chuyện, Thư viện ảnh, Video và Thư mục tệp.

### 2.3. Nhóm Tiện ích Hệ thống Bên phải
- **Chuyển đổi Chế độ Sáng/Tối (Theme Toggle)**: Nút tròn chứa biểu tượng Mặt trời đại diện cho chế độ Light Theme hiện tại.
- **Bộ đếm Credit Ngày (Daily Credit Meter)**:
  - Khung dạng khoang nhộng xám kem mờ chứa vòng tròn phần trăm màu xanh lá (`18% Daily Credits`).
- **Nút Nâng cấp Nhanh (`Upgrade now`)**:
  - Nút khoang nhộng màu đen tuyền kèm biểu tượng kim cương/trái tim tinh tế, thu hút sự chú ý nâng cấp dịch vụ.
- **Ảnh đại diện Người dùng (Profile Avatar)**:
  - Khung ảnh tròn hoàn hảo hiển thị avatar cá nhân ở góc ngoài cùng bên phải.

---

## 👆 3. Trạng thái Tương tác (UX States)

- **Hover vào các icon trên thanh Center Nav Pill**: Icon chuyển từ độ mờ trắng đục sang trắng sáng hoàn toàn. Icon đại diện cho trang hiện tại có nền tròn nhỏ nổi bật.
- **Hover vào Bộ đếm Credit**: Hiển thị hộp thông tin nhỏ cho biết chính xác số lượng lượt sinh ảnh còn lại trong ngày.
- **Click vào User Avatar**: Mở menu quản lý tài khoản cá nhân và cài đặt ứng dụng.
