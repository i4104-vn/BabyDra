# Chương 04: Hướng dẫn Cài đặt và Build Dự án

**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-07-23
**Phạm vi:** Yêu cầu hệ thống, cài đặt dependencies, build, chạy từng crate

---

## Mục lục

- [1. Yêu cầu hệ thống](#1-yêu-cầu-hệ-thống)
- [2. Cài đặt Dependencies](#2-cài-đặt-dependencies)
- [3. Clone và chuẩn bị dự án](#3-clone-và-chuẩn-bị-dự-án)
- [4. Build dự án](#4-build-dự-án)
- [5. Chạy từng crate ứng dụng](#5-chạy-từng-crate-ứng-dụng)
- [6. Cài đặt lên hệ thống](#6-cài-đặt-lên-hệ-thống)
- [7. Xử lý lỗi thường gặp](#7-xử-lý-lỗi-thường-gặp)

---

## 1. Yêu cầu hệ thống

### Hệ điều hành

- **Linux** với Wayland compositor đang chạy (Sway, Hyprland, GNOME Wayland, KDE Plasma Wayland, v.v.)
- Không hỗ trợ X11.

### Phiên bản Rust

- **Rust 1.78.0 trở lên** (stable channel)
- Kiểm tra phiên bản hiện tại: `rustc --version`

### Thư viện hệ thống cần thiết

| Thư viện | Gói cần cài (Arch Linux) | Gói cần cài (Ubuntu/Debian) | Dùng cho |
| :--- | :--- | :--- | :--- |
| GTK4 | `gtk4` | `libgtk-4-dev` | Bộ công cụ giao diện đồ họa |
| GTK4 Layer Shell | `gtk4-layer-shell` | `libgtk4-layer-shell-dev` | Định vị cửa sổ trên Wayland |
| GLib | `glib2` | `libglib2.0-dev` | Nền tảng cho GTK |
| D-Bus | `dbus` | `libdbus-1-dev` | Giao tiếp liên tiến trình |
| PipeWire | `pipewire` | `libpipewire-0.3-dev` | Điều khiển âm thanh |
| pkg-config | `pkgconf` | `pkg-config` | Công cụ tìm thư viện khi build |

---

## 2. Cài đặt Dependencies

### Arch Linux

```bash
sudo pacman -S rust gtk4 gtk4-layer-shell glib2 dbus pipewire pkgconf
```

### Ubuntu / Debian

```bash
sudo apt update
sudo apt install rustup libgtk-4-dev libgtk4-layer-shell-dev \
    libglib2.0-dev libdbus-1-dev libpipewire-0.3-dev pkg-config
rustup default stable
```

### Fedora

```bash
sudo dnf install rust cargo gtk4-devel gtk4-layer-shell-devel \
    glib2-devel dbus-devel pipewire-devel pkgconf
```

---

## 3. Clone và chuẩn bị dự án

```bash
# Clone repository
git clone <địa_chỉ_repository> BabyDra
cd BabyDra

# Kiểm tra Rust toolchain
rustc --version
cargo --version
```

---

## 4. Build dự án

### Build toàn bộ workspace (debug mode)

```bash
# Đứng tại thư mục gốc BabyDra/
cargo build
```

Artifact build nằm tại `target/debug/`.

### Build toàn bộ workspace (release mode)

```bash
cargo build --release
```

Artifact build nằm tại `target/release/`. File nhị phân nhỏ hơn và chạy nhanh hơn đáng kể so với debug.

### Build một crate cụ thể

```bash
# Build chỉ babydra-panel
cargo build -p babydra-panel

# Build chỉ babydra-explore ở release mode
cargo build --release -p babydra-explore
```

### Kiểm tra lỗi compile mà không sinh file binary

```bash
cargo check
# hoặc kiểm tra một crate cụ thể:
cargo check -p babydra-panel
```

---

## 5. Chạy từng crate ứng dụng

**Lưu ý quan trọng:** Các ứng dụng chạy trên Wayland. Phải đang chạy trong phiên Wayland (không phải X11) mới hoạt động đúng.

### Chạy trực tiếp qua cargo (debug)

```bash
# Chạy Panel
cargo run -p babydra-panel

# Chạy File Explorer
cargo run -p babydra-explore

# Chạy Alt-Tab Switcher
cargo run -p babydra-switcher
```

### Chạy binary đã build

```bash
# Sau khi cargo build --release
./target/release/babydra-panel
./target/release/babydra-explore
```

### Chạy tất cả daemon cùng lúc

Script `start.sh` ở thư mục gốc khởi động tất cả daemon theo đúng thứ tự:

```bash
./start.sh
```

---

## 6. Cài đặt lên hệ thống

Script `install.sh` thực hiện:
1. Build tất cả crate ở release mode.
2. Copy binary vào `~/.local/bin/` (hoặc `/usr/local/bin/` nếu chạy với sudo).
3. Cài đặt file cấu hình mặc định vào `~/.config/babydra/`.
4. Cài đặt file `.desktop` và autostart service.

```bash
# Cài đặt cho user hiện tại
./install.sh

# Cài đặt toàn hệ thống (cần quyền root)
sudo ./install.sh --system
```

---

## 7. Xử lý lỗi thường gặp

### Lỗi: `error: failed to run custom build command for gtk4-sys`

**Nguyên nhân:** Thiếu thư viện GTK4 development headers.

**Giải pháp:**
```bash
# Arch Linux
sudo pacman -S gtk4

# Ubuntu
sudo apt install libgtk-4-dev
```

### Lỗi: `cannot find -lgtk4-layer-shell`

**Nguyên nhân:** Thiếu thư viện gtk4-layer-shell.

**Giải pháp:**
```bash
# Arch Linux (có trong AUR)
yay -S gtk4-layer-shell

# Build từ source nếu không có package
git clone https://github.com/wmww/gtk4-layer-shell
cd gtk4-layer-shell && mkdir build && cd build
meson setup --prefix=/usr
ninja && sudo ninja install
```

### Lỗi: `WAYLAND_DISPLAY is not set`

**Nguyên nhân:** Đang chạy trong phiên X11 hoặc terminal không có biến môi trường Wayland.

**Giải pháp:** Chắc chắn đang đăng nhập vào phiên Wayland. Kiểm tra:
```bash
echo $WAYLAND_DISPLAY   # Phải trả về tên socket như wayland-1
echo $XDG_SESSION_TYPE  # Phải trả về "wayland"
```

### Lỗi: `error[E0433]: failed to resolve: use of undeclared crate`

**Nguyên nhân:** Quên thêm dependency vào `Cargo.toml` của crate.

**Giải pháp:** Thêm dependency vào `Cargo.toml` của crate đang phát triển:
```toml
[dependencies]
babydra-common = { workspace = true }
babydra-utils = { workspace = true }
```

### Build chậm lần đầu

Build lần đầu tốn nhiều thời gian (5–15 phút tùy máy) vì phải compile tất cả dependencies. Từ lần thứ hai trở đi, Cargo cache lại, chỉ rebuild những gì thay đổi, mất khoảng 5–30 giây.
