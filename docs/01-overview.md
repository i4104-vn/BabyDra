# Chương 01: Tổng quan Dự án BabyDra

**Phiên bản:** 1.1.0  
**Phạm vi:** Giới thiệu dự án, các thành phần hệ thống, mục tiêu thiết kế và mô hình phân phối

---

## Mục lục

- [1. BabyDra là gì?](#1-babydra-là-gì)
- [2. Mục tiêu thiết kế](#2-mục-tiêu-thiết-kế)
- [3. Các thành phần của hệ thống](#3-các-thành-phần-của-hệ-thống)
- [4. Mô hình phân phối và phân nhánh](#4-mô-hình-phân-phối-và-phân-nhánh)
- [5. Sơ đồ kiến trúc tổng thể](#5-sơ-đồ-kiến-trúc-tổng-thể)
- [6. Bảng tra cứu cho lập trình viên](#6-bảng-tra-cứu-cho-lập-trình-viên)

---

## 1. BabyDra là gì?

BabyDra là môi trường desktop (Desktop Environment) xây dựng trên nền tảng giao thức Wayland dành cho hệ điều hành Arch Linux, được phát triển bằng ngôn ngữ lập trình Rust kết hợp với bộ công cụ đồ họa GTK4 và GTK4 Layer Shell.

Hệ thống được tối ưu hóa nhằm đáp ứng hai tiêu chí: tốc độ phản hồi tức thì và khả năng phân tách module rõ ràng, cho phép tùy biến và mở rộng linh hoạt.

---

## 2. Mục tiêu thiết kế

### 2.1. Phản hồi tức thì (Deterministic Low Latency)
Thời gian từ khi kích hoạt sự kiện đầu vào đến khi hiển thị giao diện được kiểm soát dưới 10 mili-giây. Hệ thống áp dụng mô hình Daemon-Client: các cửa sổ đồ họa được nạp sẵn vào bộ nhớ và chỉ thay đổi trạng thái hiển thị thay vì khởi tạo lại tiến trình.

### 2.2. Tính thẩm mỹ và ngôn ngữ thiết kế nhất quán
Giao diện áp dụng ngôn ngữ thiết kế Glassmorphism với nền bán trong suốt, bo tròn góc và hỗ trợ đồng bộ hai chế độ màu sáng (Light) và tối (Dark).

### 2.3. Cấu trúc module hóa và khả năng độc lập
Mã nguồn được phân rã thành các crate và thư viện độc lập. Các module giao tiếp với nhau qua D-Bus, Unix Domain Socket hoặc cơ chế gọi hàm nội bộ qua thư viện dùng chung.

---

## 3. Các thành phần của hệ thống

### 3.1. Thư viện dùng chung (`libs/`)

| Tên thư viện | Đường dẫn | Vai trò |
|---|---|---|
| `babydra-common` | `libs/babydra-common/` | Dịch vụ hệ điều hành, D-Bus, sysfs, mô hình dữ liệu và đa ngôn ngữ |
| `babydra-utils` | `libs/babydra-utils/` | CSS toàn cục, bộ widget dùng chung, bộ phân giải icon và quản lý theme |
| `babydra-island` | `libs/babydra-island/` | Widget Dynamic Island hiển thị thông báo và điều khiển phát nhạc |
| `babydra-launcher` | `libs/babydra-launcher/` | Thuật toán tìm kiếm, phân loại và thực thi ứng dụng desktop |

### 3.2. Ứng dụng thực thi (`crates/` và `install/`)

| Tên ứng dụng | Đường dẫn | Chức năng |
|---|---|---|
| `babydra-panel` | `crates/babydra-panel/` | Thanh taskbar chính, khay hệ thống, đồng hồ và trung tâm điều khiển |
| `babydra-switcher` | `crates/babydra-switcher/` | Bộ chuyển đổi cửa sổ ứng dụng (Alt+Tab Switcher) |
| `babydra-screenshot` | `crates/babydra-screenshot/` | Công cụ chụp màn hình đồ họa |
| `babydra-lock` | `crates/babydra-lock/` | Màn hình khóa và xác thực người dùng |
| `babydra-greeter` | `crates/babydra-greeter/` | Màn hình đăng nhập hệ thống tương thích Greetd |
| `babydra-settings` | `crates/babydra-settings/` | Trung tâm cấu hình hệ thống, giao diện và phần cứng |
| `babydra-preview` | `crates/babydra-preview/` | Ứng dụng xem nhanh hình ảnh |
| `babydra-explore` | `crates/babydra-explore/` | Trình duyệt và quản lý tập tin đồ họa |
| `babydra-installer` | `install/` | Bộ cài đặt TUI đa bước quản lý pull, build và phân phối binary |

---

## 4. Mô hình phân phối và phân nhánh

Kho mã nguồn BabyDra áp dụng mô hình phân tách 3 nhánh:

1. **`main`**: Nhánh phân phối tinh gọn, chỉ chứa công cụ cài đặt `babydra-installer`, script thực thi và tài liệu hướng dẫn.
2. **`release`**: Nhánh mã nguồn chính thức và ổn định do tác giả duy trì.
3. **`develop`**: Nhánh phát triển chung được checkout từ `release`, phục vụ việc đóng góp tính năng mới từ cộng đồng.

---

## 5. Sơ đồ kiến trúc tổng thể

```
[ Người dùng / Phím tắt ]
           |
           v
+------------------------+      Unix Socket / D-Bus      +------------------------+
| Client gửi tín hiệu    | ----------------------------> | Daemon (crates/)       |
+------------------------+                               | Cửa sổ nạp sẵn bộ nhớ  |
                                                         +------------------------+
                                                                     |
                                                                     v
                                                         +------------------------+
                                                         | babydra-common (libs/) |
                                                         +------------------------+
                                                                     |
                                                                     v
                                                         +------------------------+
                                                         | Hệ thống Arch Linux    |
                                                         | /sys, /proc, systemd   |
                                                         +------------------------+
```

---

## 6. Bảng tra cứu cho lập trình viên

| Yêu cầu | Vị trí mã nguồn | Tài liệu tham chiếu |
|---|---|---|
| CSS và giao diện | `libs/babydra-utils/src/styles/` | [03-project-structure.md](./03-project-structure.md) |
| Dịch vụ phần cứng và OS | `libs/babydra-common/src/services/` | [02-architecture.md](./02-architecture.md) |
| Cấu trúc widget giao diện | Tách biệt `mod.rs` và `render.rs` | [03-project-structure.md](./03-project-structure.md) |
| Hướng dẫn biên dịch | `cargo build --workspace` | [04-setup-and-build.md](./04-setup-and-build.md) |
| Quy trình phân nhánh và merge | Git workflow | [../WORKFLOW.md](../WORKFLOW.md) |
