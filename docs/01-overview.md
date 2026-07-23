# Chương 01: Tổng quan Dự án BabyDra

**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-07-23
**Phạm vi:** Giới thiệu dự án, các thành phần hệ thống, mục tiêu thiết kế

---

## Mục lục

- [1. BabyDra là gì?](#1-babydra-là-gì)
- [2. Mục tiêu thiết kế](#2-mục-tiêu-thiết-kế)
- [3. Các thành phần của hệ thống](#3-các-thành-phần-của-hệ-thống)
- [4. Sơ đồ tổng thể](#4-sơ-đồ-tổng-thể)
- [5. Cheat-sheet nhanh cho Developer](#5-cheat-sheet-nhanh-cho-developer)

---

## 1. BabyDra là gì?

BabyDra là một **môi trường desktop Linux nhẹ** (lightweight Linux Desktop Environment) được viết bằng ngôn ngữ lập trình **Rust**, sử dụng bộ công cụ giao diện **GTK4** để dựng giao diện đồ họa và chạy trên **Wayland** (giao thức hiển thị đồ họa hiện đại trên Linux).

Dự án được thiết kế để phù hợp với các máy tính có cấu hình trung bình, ưu tiên tốc độ phản hồi tức thì và giao diện thẩm mỹ cao mà không tiêu tốn nhiều tài nguyên hệ thống.

**Giải thích thuật ngữ:**

- **Desktop Environment (DE):** Bộ phần mềm cung cấp giao diện đồ họa cho hệ điều hành Linux. Bao gồm thanh taskbar, trình quản lý cửa sổ, trình khởi chạy ứng dụng, và các thành phần khác mà người dùng tương tác hàng ngày.
- **GTK4:** Thư viện giao diện đồ họa (GUI toolkit) phổ biến trên Linux, cung cấp sẵn các widget như nút bấm, ô nhập liệu, cửa sổ.
- **Wayland:** Giao thức hiển thị thế hệ mới trên Linux, thay thế cho X11. Có hiệu năng tốt hơn và bảo mật cao hơn.

---

## 2. Mục tiêu thiết kế

BabyDra được xây dựng xung quanh 3 mục tiêu cốt lõi:

### 2.1. Phản hồi tức thì (Instant Response)

Mọi thao tác của người dùng (nhấn phím tắt, click chuột, mở panel) phải cho kết quả hiển thị trong vòng dưới 10 mili-giây. Để đạt được điều này, dự án dùng mô hình **Daemon-Client**: giao diện được giữ sẵn trong bộ nhớ, chỉ bật/tắt hiển thị thay vì khởi động lại từ đầu mỗi lần gọi. Chi tiết xem [02-architecture.md](./02-architecture.md).

### 2.2. Thẩm mỹ cao (High-quality Aesthetics)

Giao diện sử dụng ngôn ngữ thiết kế **Glassmorphism** (kính mờ): nền bán trong suốt, hiệu ứng làm mờ phía sau, góc bo tròn mềm mại. Hỗ trợ cả chế độ sáng (Light) và tối (Dark). Chi tiết xem [design/01-design-philosophy.md](./design/01-design-philosophy.md).

### 2.3. Dễ bảo trì và mở rộng (Maintainable & Extensible)

Mã nguồn được tổ chức theo nguyên tắc phân tách rõ ràng giữa giao diện và logic nghiệp vụ. Mỗi thành phần là một module độc lập, dễ thêm mới hoặc sửa đổi mà không ảnh hưởng đến phần còn lại. Chi tiết xem [03-project-structure.md](./03-project-structure.md).

---

## 3. Các thành phần của hệ thống

Dự án BabyDra bao gồm hai loại crate Rust:

### 3.1. Thư viện dùng chung (Libraries) — thư mục `libs/`

Các thư viện này không thể chạy độc lập. Chúng cung cấp API, widget, và tài nguyên cho các ứng dụng trong `crates/`.

| Tên thư viện | Thư mục | Vai trò |
| :--- | :--- | :--- |
| `babydra-common` | `libs/babydra-common/` | Thư viện lõi: tương tác hệ điều hành, D-Bus, tệp `/sys/class`, models dữ liệu |
| `babydra-utils` | `libs/babydra-utils/` | Thư viện tiện ích: CSS toàn cục, widget dùng chung, khởi tạo theme |
| `babydra-island` | `libs/babydra-island/` | Widget Dynamic Island (hiệu ứng chỉ báo trên màn hình) |
| `babydra-launcher` | `libs/babydra-launcher/` | Logic tìm kiếm và khởi chạy ứng dụng |

### 3.2. Ứng dụng có thể thực thi (Executables) — thư mục `crates/`

Mỗi crate là một tiến trình độc lập, đảm nhận một chức năng cụ thể của desktop.

| Tên ứng dụng | Thư mục | Chức năng |
| :--- | :--- | :--- |
| `babydra-panel` | `crates/babydra-panel/` | Thanh taskbar chính: dock, system tray, clock, các toggle điều khiển nhanh |
| `babydra-switcher` | `crates/babydra-switcher/` | Bộ chuyển đổi ứng dụng (Alt+Tab Switcher) |
| `babydra-screenshot` | `crates/babydra-screenshot/` | Công cụ chụp màn hình |
| `babydra-lock` | `crates/babydra-lock/` | Màn hình khóa (Lock Screen) |
| `babydra-preview` | `crates/babydra-preview/` | Xem trước tệp (File Preview) |
| `babydra-settings` | `crates/babydra-settings/` | Cài đặt hệ thống |
| `babydra-explore` | `crates/babydra-explore/` | Trình quản lý tệp (File Explorer) |

---

## 4. Sơ đồ tổng thể

Sơ đồ dưới đây mô tả cách các thành phần giao tiếp với nhau:

```
Người dùng
     |
     | (nhấn phím, click chuột)
     v
+------------------+     Unix Socket / D-Bus     +-------------------+
| Client nhẹ       | --------------------------> | Daemon (crates/)  |
| (phím tắt)       |                             | Cửa sổ ẩn sẵn     |
+------------------+                             | trong bộ nhớ      |
                                                 +-------------------+
                                                          |
                                                          | gọi API
                                                          v
                                                 +-------------------+
                                                 | babydra-common    |
                                                 | (libs/)           |
                                                 | - Services        |
                                                 | - Models          |
                                                 +-------------------+
                                                          |
                                                          | đọc/ghi
                                                          v
                                                 +-------------------+
                                                 | Hệ điều hành Linux|
                                                 | - /sys/class/     |
                                                 | - D-Bus           |
                                                 | - sysfs           |
                                                 +-------------------+
```

```
babydra-utils (libs/)
      |
      +-- CSS toàn cục (dark/ và light/)
      +-- Widget dùng chung (components/)
      +-- Khởi tạo theme (ui/theme/)
      |
      v
   Được nạp vào GDK Display Context
   khi bất kỳ crate nào khởi động
```

---

## 5. Cheat-sheet nhanh cho Developer

Bảng tra cứu nhanh khi bắt đầu làm việc với dự án:

| Câu hỏi | Trả lời ngắn | Xem chi tiết tại |
| :--- | :--- | :--- |
| CSS nằm ở đâu? | `libs/babydra-utils/src/styles/` | [03-project-structure.md](./03-project-structure.md) |
| Thêm logic đọc hardware mới? | Thêm vào `libs/babydra-common/src/services/` | [02-architecture.md](./02-architecture.md) |
| Thêm widget mới? | Tách thành `mod.rs` + `render.rs` | [03-project-structure.md](./03-project-structure.md) |
| Màu accent là gì? | `#3b82f6` (Xanh dương Neon) | [design/02-design-tokens.md](./design/02-design-tokens.md) |
| Hover dùng transform không? | Không bao giờ. Chỉ đổi màu. | [design/01-design-philosophy.md](./design/01-design-philosophy.md) |
| Build dự án như thế nào? | `cargo build` tại thư mục gốc | [04-setup-and-build.md](./04-setup-and-build.md) |
| CSS inline trong Rust được không? | Không. CSS phải đặt trong `styles/` | [03-project-structure.md](./03-project-structure.md) |
