# BabyDra Desktop Shell

## 1. Tổng quan Dự án

BabyDra là môi trường giao diện người dùng (Desktop Shell) xây dựng trên nền tảng giao thức Wayland dành cho hệ điều hành Arch Linux. Dự án sử dụng trình quản lý cửa sổ `labwc` kết hợp cùng bộ công cụ đồ họa `GTK4 Layer Shell` nhằm tối ưu hóa hiệu năng, giảm thiểu độ trễ phản hồi và duy trì tính thẩm mỹ cao.

Nhánh `main` đóng vai trò là kênh phân phối tập trung, lưu trữ công cụ cài đặt độc lập (`babydra-installer`), script khởi tạo hệ thống và hệ thống tài liệu hướng dẫn.

---

## 2. Cơ chế phân nhánh hệ thống

Kho mã nguồn BabyDra được tổ chức theo cấu trúc phân tầng:

| Nhánh | Mục đích | Phạm vi truy cập |
|---|---|---|
| `main` | Phân phối bộ cài đặt TUI và tài liệu hướng dẫn | Kênh mặc định cho người dùng cuối |
| `release` | Mã nguồn đầy đủ của phiên bản chính thức ổn định | Do tác giả trực tiếp duy trì và phát hành |
| `develop` | Nhánh nền tảng phục vụ phát triển và tích hợp cộng đồng | Checkout từ `release`, dành cho lập trình viên |

---

## 3. Hướng dẫn cài đặt

### 3.1. Yêu cầu hệ thống
- Hệ điều hành: Arch Linux hoặc các bản phân phối tương thích.
- Trình quản lý gói: `pacman` và `yay` (hoặc trình trợ giúp AUR tương đương).
- Môi trường Wayland: Trình quản lý cửa sổ `labwc`.
- Rust toolchain: Phiên bản 1.80.0 trở lên.

### 3.2. Cài đặt thông qua Bộ cài đặt TUI (babydra-installer)

Người dùng có thể khởi chạy bộ cài đặt trực tiếp từ thư mục gốc của nhánh `main`:

```bash
# Cấp quyền thực thi và khởi chạy script cài đặt
chmod +x ./install.sh
./install.sh
```

Hoặc khởi chạy trực tiếp thông qua Cargo:

```bash
cargo run --release -p babydra-installer
```

### 3.3. Cơ chế hoạt động của bộ cài đặt

Bộ cài đặt `babydra-installer` hỗ trợ hai kênh nguồn chính:
1. **Release Channel**: Tự động đồng bộ mã nguồn từ nhánh `release`, thực hiện biên dịch tối ưu và triển khai các tệp nhị phân chính thức.
2. **Develop Channel**: Đồng bộ mã nguồn từ nhánh `develop` để cài đặt các tính năng mới nhất từ cộng đồng.

Sau khi hoàn tất biên dịch, các tệp nhị phân được tự động cài đặt vào `~/.local/bin` và đưa vào thư mục lưu trữ hệ thống `/var/lib/babydra`.

---

## 4. Hướng dẫn dành cho nhà phát triển

Nhà phát triển mong muốn đóng góp mã nguồn hoặc tạo biến thể riêng cần chuyển sang nhánh `develop`:

```bash
# 1. Chuyển sang nhánh phát triển
git checkout develop

# 2. Đồng bộ dữ liệu mới nhất từ upstream
git pull origin develop

# 3. Tạo nhánh làm việc theo định danh cá nhân
git checkout -b feature/<tên-user>

# 4. Kiểm tra biên dịch toàn bộ workspace
cargo check --workspace
```

Quy trình phân nhánh và quy tắc đóng góp chi tiết được quy định tại tập tin [WORKFLOW.md](WORKFLOW.md).

---

## 5. Tài liệu kỹ thuật chi tiết

- [Chương 01: Tổng quan dự án](docs/01-overview.md)
- [Chương 02: Kiến trúc hệ thống](docs/02-architecture.md)
- [Chương 03: Cấu trúc thư mục và module](docs/03-project-structure.md)
- [Chương 04: Hướng dẫn thiết lập và biên dịch](docs/04-setup-and-build.md)
- [Quy chuẩn phân nhánh và phát triển](WORKFLOW.md)

---

## 6. Giấy phép sử dụng

Dự án được phát hành dưới giấy phép MIT License. Chi tiết xem tại tập tin LICENSE.
