# BabyDra

BabyDra là môi trường desktop Linux viết bằng Rust, chạy trên Wayland và compositor labwc. Dự án cung cấp các thành phần giao diện, ứng dụng desktop, thư viện dùng chung và một bộ cài đặt TUI có khả năng cài mã nguồn từ các branch khác nhau.

Kho lưu trữ được tổ chức thành hai loại branch:

| Branch | Nội dung | Mục đích |
| :--- | :--- | :--- |
| `main` | `install/` và tài liệu | Phân phối bộ cài đặt. Branch này không chứa danh sách binary hoặc package của một phiên bản cụ thể. |
| `release` và các branch nguồn | Workspace Rust, tài nguyên assets (cấu hình, theme, desktop) và `workspace.toml` | Nguồn mã được bộ cài đặt checkout, build và triển khai. |

## Thành phần chính

### Binary

| Binary | Vai trò |
| :--- | :--- |
| `babydra-panel` | Panel, dock, status area, system tray, notification và Dynamic Island. |
| `babydra-desktop` | Nền desktop, wallpaper, biểu tượng tệp và context menu. |
| `babydra-switcher` | Bộ chuyển cửa sổ Alt-Tab. |
| `babydra-keymap` | Daemon xử lý phím tắt toàn cục từ Linux evdev. |
| `babydra-workspace` | Điều phối workspace ảo trên Wayland. |
| `babydra-explore` | Trình quản lý tệp GTK4. |
| `babydra-settings` | Giao diện cấu hình mạng, âm thanh, màn hình, theme và phím tắt. |
| `babydra-launcher` | Trình khởi chạy ứng dụng và tìm kiếm mờ. |
| `babydra-lock` | Màn hình khóa và xác thực PAM. |
| `babydra-greeter` | Giao diện đăng nhập cho greetd/cage. |
| `babydra-screenshot` | Chụp màn hình và xử lý OCR. |
| `babydra-preview` | Xem trước tệp ảnh và phương tiện. |

Danh sách thực tế không được bộ cài đặt duy trì bằng tên cố định. Nó được đọc từ `workspace.toml` của branch được chọn và đối chiếu với các binary Cargo được phát hiện trong workspace.

### Thư viện

| Thư viện | Vai trò |
| :--- | :--- |
| `babydra-core` | Service hệ thống, cấu hình, i18n và model không phụ thuộc GTK. |
| `babydra-ui-kit` | Widget GTK4, CSS dùng chung, icon và animation. |
| `babydra-island` | Runtime và cơ chế ưu tiên của Dynamic Island. |
| `babydra-theme` | Đọc, kế thừa và hợp nhất theme package. |

## Cài đặt

### Yêu cầu

- Arch Linux hoặc bản phân phối tương thích `pacman`.
- Wayland và labwc.
- Rust toolchain tương thích với `Cargo.toml` của branch nguồn.
- Quyền `sudo` cho các bước cài package, ghi `/usr/bin`, `/var/lib` và cấu hình greetd.
- Kết nối mạng khi cài package hoặc checkout branch.

### Chạy bộ cài đặt

Trên `main`:

```bash
./install/run.sh
```

Hoặc:

```bash
cd install
cargo run --release
```

Bộ cài đặt sẽ liệt kê branch có workspace Cargo, fetch thông tin branch từ remote, checkout branch đã chọn vào `branches/<branch>`, đọc `workspace.toml`, build workspace và triển khai các tài nguyên tương ứng.

### Build branch nguồn

Trên branch chứa workspace Rust:

```bash
cargo check --workspace
cargo test --workspace
cargo build --release --workspace
```

## `workspace.toml`

Mỗi branch nguồn có thể chứa một file `workspace.toml` ở thư mục gốc. Đây là metadata dành cho bộ cài đặt, không phải file cấu hình của bộ cài đặt trên `main`.

Ví dụ tối thiểu:

```toml
[[binaries]]
name = "babydra-panel"
scope = "user"
description = "Desktop panel and background services"

[[binaries]]
name = "babydra-greeter"
scope = "system"

[packages]
pacman = ["gtk4", "labwc"]
aur = ["fastfetch"]

[gsettings]
"org.gnome.desktop.interface.font-name" = "Inter 11"

[mime]
"text/plain" = "babydra-notepad.desktop"
```

`scope = "user"` cài binary vào `~/.local/bin`; `scope = "system"` cài vào `/usr/bin`. Có thể đặt `source = "tên-binary-build-ra"` khi tên file build khác tên binary cài đặt. Các file `.desktop` được đặt trực tiếp trong thư mục `desktops/`, installer sẽ tự động cài đặt và đăng ký các MIME associations được khai báo trong file desktop hoặc bảng `[mime]`.

Xem hướng dẫn đầy đủ trong [docs/03-setup.md](docs/03-setup.md).

## Tài liệu

| Tài liệu | Nội dung |
| :--- | :--- |
| [Tổng quan](docs/01-overview.md) | Phạm vi dự án, branch và thành phần. |
| [Kiến trúc](docs/02-architecture.md) | Ranh giới module, daemon-client và installer. |
| [Cài đặt và build](docs/03-setup.md) | Cài đặt, build, `workspace.toml` và troubleshooting. |
| [Cấu trúc dự án](docs/04-structure.md) | Cây thư mục và quy tắc đặt code. |
| [Theme](docs/05-themes.md) | Cấu trúc theme package và quy trình thêm theme. |
| [Luồng hệ thống](docs/06-system-flows.md) | Luồng khởi động và trao đổi giữa các thành phần. |
| [Dynamic Island](docs/07-dynamic-island.md) | Runtime, arbitration và cách thêm feature. |
| [API](docs/08-apis.md) | API của các thư viện lõi. |
| [Ngôn ngữ thiết kế](docs/09-design.md) | Token, typography, spacing và motion. |
| [Component library](docs/10-components.md) | Các widget GTK4 dùng chung. |

Quy trình đóng góp nằm trong [CONTRIBUTING.md](CONTRIBUTING.md).

## Giấy phép

BabyDra được phát hành theo Apache License 2.0. Xem [LICENSE](LICENSE).
