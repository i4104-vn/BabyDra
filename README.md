<p align="center">
  <img src="logo.png" width="120" height="120" alt="BabyDra logo">
</p>

<h3 align="center">BabyDra — Desktop Shell</h3>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.80+-a5844f?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/GTK4-0.9-blue?style=for-the-badge&logo=gtk&logoColor=white" alt="GTK4">
  <img src="https://img.shields.io/badge/Wayland-labwc-4bc0c0?style=for-the-badge" alt="Wayland">
  <img src="https://img.shields.io/badge/license-Apache_2.0-blue?style=for-the-badge" alt="License">
</p>

<p align="center">Môi trường desktop Wayland nhẹ, hiệu năng cao cho Arch Linux — viết bằng Rust + GTK4 Layer Shell, chạy trên compositor labwc.</p>

---

## Mục lục

- [Giới thiệu](#giới-thiệu)
- [Cài đặt](#cài-đặt)
- [Mô hình phân nhánh](#mô-hình-phân-nhánh)
- [Phát triển & đóng góp](#phát-triển--đóng-góp)
- [Tài liệu](#tài-liệu)
- [Giấy phép](#giấy-phép)

---

## Giới thiệu

BabyDra là một **môi trường desktop (Desktop Shell) Linux nhẹ** xây dựng trên nền tảng giao thức Wayland dành cho Arch Linux. Dự án kết hợp trình quản lý cửa sổ `labwc` với bộ công cụ đồ họa `GTK4 Layer Shell` nhằm tối ưu hiệu năng, giảm độ trễ phản hồi, tận dụng GPU cho FPS cao và duy trì tính thẩm mỹ (glassmorphism, design tokens đồng nhất).

Các thành phần chính:

| Thành phần | Crate | Vai trò |
| :--- | :--- | :--- |
| Dynamic Island & Panel | `babydra-panel` | Island, dock, status bar, notification |
| Window Switcher | `babydra-switcher` | Alt-Tab với icon & preview cửa sổ |
| File Explorer | `babydra-explore` | Trình duyệt file GTK4 hiện đại |
| Settings | `babydra-settings` | Trung tâm cấu hình hệ thống |
| Launcher | `babydra-launcher` | App grid + tìm kiếm nhanh |
| Lock & Greeter | `babydra-lock`, `babydra-greeter` | Màn hình khóa & đăng nhập (greetd/cage) |
| Screenshot & Preview | `babydra-screenshot`, `babydra-preview` | Chụp màn hình, xem ảnh nhanh |

> [!NOTE]
> Kho mã nguồn phân tách theo mô hình 3 nhánh. Nhánh `main` chỉ chứa bộ cài đặt và tài liệu — chi tiết xem mục [Mô hình phân nhánh](#mô-hình-phân-nhánh).

---

## Cài đặt

### Yêu cầu hệ thống

- Hệ điều hành: Arch Linux hoặc bản phân phối tương thích.
- Trình quản lý gói: `pacman` và `yay` (hoặc trình trợ giúp AUR tương đương).
- Môi trường Wayland: trình quản lý cửa sổ `labwc`.
- Rust toolchain: phiên bản 1.80.0 trở lên.

### Cách 1 — Bộ cài đặt TUI (babydra-installer)

Bộ cài đặt tương tác trong terminal, wizard 10 bước:

```bash
cargo run --release -p babydra-installer
```

### Cách 2 — Script tự động

```bash
chmod +x ./scripts/install.sh
./scripts/install.sh
```

Script thực hiện đúng quy trình của `babydra-installer`: cài dependencies, build release, deploy binaries, đồng bộ config labwc/GTK/theme/greetd.

### Cơ chế hoạt động của bộ cài đặt

`babydra-installer` liệt kê **toàn bộ nhánh có thể cài đặt** (trừ `main` — nhánh chỉ chứa bộ cài đặt, không có mã nguồn để build):

1. **Release Channel** — đồng bộ mã nguồn từ nhánh `release` (mặc định), biên dịch tối ưu và triển khai tệp nhị phân chính thức.
2. **Develop Channel** — đồng bộ mã nguồn từ nhánh `develop` để cài đặt các tính năng mới nhất.
3. **Nhánh đóng góp viên** — bất kỳ nhánh nào khác được tạo từ `develop` đều có thể chọn để cài đặt thử nghiệm.

Khi chạy, bộ cài đặt:

1. Yêu cầu **mật khẩu sudo trước** (modal che ký tự, xác thực 1 lần trước khi thay đổi bất cứ điều gì — tránh khóa tài khoản).
2. Checkout nhánh đã chọn → `git pull` → `cargo build --release`.
3. Sao chép binaries vào `~/.local/bin` (riêng `babydra-greeter` vào `/usr/bin`).
4. Đưa binaries + wallpapers + logo vào `/var/lib/babydra` và `/usr/share/babydra`.
5. Đồng bộ config `labwc`, GTK, terminal (kitty/neovim), theme packages (`~/.babydra/themes`), icon, cursor, greetd.

---

## Mô hình phân nhánh

Kho mã nguồn BabyDra được tổ chức theo mô hình 3 nhánh chính, tất cả do **tác giả** trực tiếp quản lý:

| Nhánh | Vai trò | Quyền hạn |
| :--- | :--- | :--- |
| `main` | Kênh phân phối tinh gọn — **chỉ chứa bộ cài đặt** (`install/`) và tài liệu | Chỉ tác giả |
| `release` | **Nhánh mặc định** — mã nguồn đầy đủ chính thức do tác giả push lên | Chỉ tác giả |
| `develop` | Nền tảng phát triển, tách ra từ `release` | Chỉ tác giả |

Không ai ngoài tác giả có thể push trực tiếp vào `main`, `release` hoặc `develop`. Người đóng góp **tạo nhánh riêng từ `develop`** và chỉ làm việc bên trong nhánh của mình; bộ cài đặt liệt kê các nhánh đó để cài đặt thử nghiệm.

---

## Phát triển & đóng góp

Nhà phát triển làm việc trên nhánh `develop`:

```bash
git checkout develop
git pull origin develop
git checkout -b <tên-bạn>/<tên-công-việc>   # nhánh riêng của bạn
cargo check --workspace
```

Trước khi gửi thay đổi, chạy bộ kiểm tra an toàn:

```bash
./scripts/check.sh          # cargo check + fmt + clippy -D warnings + test
```

Quy trình và quy tắc đóng góp chi tiết: [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Tài liệu

Tài liệu chính thức nằm trong thư mục `docs/` (bắt đầu từ [docs/README.md](docs/README.md)):

| Tài liệu | Nội dung |
| :--- | :--- |
| [01 — Tổng quan](docs/01-overview.md) | Giới thiệu, thành phần, mô hình phân nhánh |
| [02 — Kiến trúc](docs/02-architecture.md) | Pattern thiết kế, daemon-client, sơ đồ hệ thống |
| [03 — Cài đặt & build](docs/03-setup.md) | Cài đặt qua installer/script, build từ nguồn |
| [04 — Cấu trúc & quy chuẩn](docs/04-structure.md) | Cây thư mục, trách nhiệm module, quy chuẩn code |
| [05 — Themes & Variants](docs/05-themes-variants.md) | Tạo theme & variant mới — không cần sửa code |
| [06 — Luồng hoạt động](docs/06-system-flows.md) | Luồng hoạt động hiện tại của từng crate/lib |
| [07 — Dynamic Island](docs/07-dynamic-island.md) | Dùng & mở rộng Dynamic Island |
| [08 — API](docs/08-apis.md) | API reference (core, ui-kit, explore) |
| [09 — Design](docs/09-design.md) · [10 — Components](docs/10-components.md) | Ngôn ngữ thiết kế & component library |

---

## Giấy phép

Dự án được phát hành dưới giấy phép **Apache License 2.0**. Chi tiết xem tại tập tin [LICENSE](LICENSE).
