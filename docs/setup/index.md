# Cài đặt & build dự án

**Phạm vi:** Yêu cầu hệ thống, cài đặt dependencies, build, chạy từng crate, cài đặt DE và xử lý lỗi.
**Phiên bản:** 1.3.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Yêu cầu hệ thống](#1-yêu-cầu-hệ-thống)
- [2. Cài đặt Dependencies](#2-cài-đặt-dependencies)
- [3. Clone và chuẩn bị dự án](#3-clone-và-chuẩn-bị-dự-án)
- [4. Cài đặt qua Bộ cài đặt TUI (khuyến nghị)](#4-cài-đặt-qua-bộ-cài-đặt-tui-khuyến-nghị)
- [5. Build từ mã nguồn (nhánh release)](#5-build-từ-mã-nguồn-nhánh-release)
- [6. Chạy từng crate ứng dụng](#6-chạy-từng-crate-ứng-dụng)
- [7. Script hệ thống](#7-script-hệ-thống)
- [8. Xử lý lỗi thường gặp](#8-xử-lý-lỗi-thường-gặp)

---

## 1. Yêu cầu hệ thống

### Hệ điều hành

- **Arch Linux** hoặc bản phân phối tương thích (dùng `pacman` và `yay`).
- **Wayland compositor** đang chạy — mặc định **labwc**. Không hỗ trợ X11.
- Kiểm tra: `echo $XDG_SESSION_TYPE` phải trả về `wayland`.

### Phiên bản Rust

- **Rust 1.80.0 trở lên** (stable channel). Kiểm tra: `rustc --version`.

### Thư viện hệ thống cần thiết

| Thư viện | Gói cần cài (Arch Linux) | Dùng cho |
| :--- | :--- | :--- |
| GTK4 | `gtk4` | Bộ công cụ giao diện đồ họa |
| GTK4 Layer Shell | `gtk4-layer-shell` | Định vị cửa sổ trên Wayland |
| GLib | `glib2` | Nền tảng cho GTK |
| D-Bus | `dbus` | Giao tiếp liên tiến trình |
| PipeWire | `pipewire` | Điều khiển âm thanh |
| pkg-config | `pkgconf` | Tìm thư viện khi build |
| labwc | `labwc` | Wayland compositor / window manager |
| greetd + cage | `greetd cage` | Màn hình đăng nhập |
| playerctl | `playerctl` | Điều khiển media player (island) |
| grim + slurp | `grim slurp` | Chụp màn hình |
| wl-clipboard | `wl-clipboard` | Clipboard (screenshot copy) |
| ddcutil | `ddcutil` | Điều khiển độ sáng màn hình ngoài |

> [!NOTE]
> Nếu dùng **Bộ cài đặt TUI** (mục 4), toàn bộ dependencies được cài tự động qua bước "System Packages & Deps" — không cần cài tay.

---

## 2. Cài đặt Dependencies

### Arch Linux (cài tay)

```bash
sudo pacman -S --needed base-devel git pkgconf gtk4 gtk4-layer-shell rust \
    labwc meson ninja playerctl grim slurp wl-clipboard libnotify \
    pipewire pipewire-pulse pipewire-alsa wireplumber ddcutil \
    greetd cage bluez bluez-utils networkmanager polkit
```

### AUR helper (nếu chưa có)

```bash
git clone https://aur.archlinux.org/yay-bin.git /tmp/yay-bin
cd /tmp/yay-bin && makepkg -si
```

### Fonts và công cụ bổ sung (qua yay)

```bash
yay -S --noconfirm ttf-segoe-ui-variable ttf-cascadia-code-nerd kitty neovim fastfetch wlrctl awww
```

---

## 3. Clone và chuẩn bị dự án

```bash
git clone <địa_chỉ_repository> BabyDra
cd BabyDra

rustc --version
cargo --version
```

Hai cách sử dụng:

| Nhánh | Mục đích |
| :--- | :--- |
| `main` | Người dùng cuối: chạy bộ cài đặt TUI (mục 4) |
| `release` | Developer: build toàn bộ mã nguồn DE (mục 5) |

```bash
git fetch origin release
git checkout release
```

---

## 4. Cài đặt qua Bộ cài đặt TUI (khuyến nghị)

Bộ cài đặt `babydra-installer` nằm trên nhánh `main` — cách cài đặt khuyến nghị cho người dùng cuối.

### 4.1. Khởi chạy

```bash
# Cách 1: Script install.sh (build binary nếu chưa có rồi launch TUI)
chmod +x ./install.sh
./install.sh

# Cách 2: Trực tiếp qua cargo
cargo run --release -p babydra-installer

# Cách 3: Với thư mục binary có sẵn (không cần build)
./install.sh /path/to/custom/binaries
```

CLI arguments:

| Argument | Mô tả |
| :--- | :--- |
| `-h`, `--help` | Hiển thị trợ giúp |
| `-v`, `--version` | Hiển thị phiên bản |
| `[SOURCE_BIN_DIR]` | Đường dẫn thư mục chứa binary có sẵn (vd: `target/release`) |

### 4.2. Quy trình trong TUI

1. **Bước 1 (Welcome):** nhấn `c` chọn kênh — **Release** (ổn định) / **Develop** (thử nghiệm) / **Local Source**. Trình chọn hiển thị commit hash, tác giả, ngày cập nhật của từng nhánh. Dùng `j`/`k` chọn preset profile.
2. **Bước 2 (Packages):** chọn pacman deps, AUR packages, kernel/i2c permissions.
3. **Bước 3 (Binaries):** chọn 9 binary component (panel, switcher, screenshot, lock, launcher, preview, settings, explore, greeter).
4. **Bước 4 (/var/lib):** stage binaries, wallpapers, logos vào `/var/lib/babydra`.
5. **Bước 5 (Configs & Themes):** sync labwc configs, .desktop entries, dotfiles, themes/icons/cursors, gsettings.
6. **Bước 6 (Display Manager):** cấu hình greetd (`cage` + `babydra-greeter`), mask gettys, enable greetd.service.
7. **Bước 7 (Execute):** theo dõi log realtime và progress gauge.
8. **Bước 8 (Summary):** xem kết quả và thoát.

> [!IMPORTANT]
> Nếu chọn kênh **Release/Develop** mà binary chưa có, bộ cài đặt tự động `git fetch origin <branch>` + `cargo build --release --workspace`. Kênh **Local Source** copy trực tiếp, không build.

### 4.3. Kết quả sau khi cài

- Binary đồ họa → `~/.local/bin/`
- `babydra-greeter` → `/usr/bin/` (cần quyền root)
- Assets staging → `/var/lib/babydra/` và `/usr/share/babydra/`
- Configs → `~/.config/labwc/`, `~/.config/kitty/`, `~/.config/nvim/`, `~/.config/fastfetch/`, `~/.config/gtk-3.0/`, `~/.config/gtk-4.0/`, `~/.config/fontconfig/`
- Themes/Icons/Cursors → `~/.local/share/themes/`, `~/.local/share/icons/`

---

## 5. Build từ mã nguồn (nhánh release)

> [!NOTE]
> Cần workspace đầy đủ — chuyển sang nhánh `release` trước.

```bash
# Release mode (khuyến nghị)
cargo build --release --workspace

# Debug mode
cargo build --workspace
```

Artifact nằm tại `target/release/` (hoặc `target/debug/`).

```bash
# Build một crate cụ thể
cargo build --release -p babydra-panel
cargo build --release -p babydra-explore

# Kiểm tra compile không sinh binary
cargo check --workspace
cargo check -p babydra-panel

# Format chuẩn
cargo fmt --check

# Clippy (lint)
cargo clippy --workspace -- -D warnings
```

---

## 6. Chạy từng crate ứng dụng

> [!IMPORTANT]
> Các ứng dụng chạy trên Wayland — phải đang trong phiên Wayland (labwc, Sway, Hyprland...) mới hoạt động đúng.

```bash
cargo run -p babydra-panel        # Thanh taskbar chính
cargo run -p babydra-explore      # Trình quản lý tập tin
cargo run -p babydra-switcher     # Alt-Tab switcher
cargo run -p babydra-settings     # Trung tâm cấu hình
cargo run -p babydra-preview -- /path/to/image.png
```

### Chạy binary đã build

```bash
./target/release/babydra-panel
./target/release/babydra-switcher --daemon   # Chế độ daemon giữ overlay trong bộ nhớ
./target/release/babydra-screenshot --full   # Chụp toàn màn hình ngay lập tức
./target/release/babydra-lock --image ~/wallpaper.png
```

### Chạy toàn bộ hệ thống

```bash
./start.sh
```

`start.sh` đồng bộ config labwc, đăng ký .desktop entries rồi `exec labwc` — toàn bộ DE khởi động cùng autostart.

---

## 7. Script hệ thống

| Script | Nhánh | Mục đích |
| :--- | :--- | :--- |
| `install.sh` (main) | `main` | Build & launch bộ cài đặt TUI (mục 4) |
| `install.sh` (release) | `release` | Cài đặt toàn bộ DE từ source: pacman deps, yay, perms, build, deploy |
| `start.sh` | `release` | Khởi động DE: sync labwc config, .desktop entries, chạy labwc |
| `update.sh` | `release` | Hot-update: rebuild, dừng tiến trình cũ, copy binary, sync config, restart panel |

**Cập nhật DE sau khi có code mới (developer):**

```bash
git pull origin release
./update.sh               # rebuild + deploy + restart
```

---

## 8. Xử lý lỗi thường gặp

### Lỗi: `error: failed to run custom build command for gtk4-sys`

**Nguyên nhân:** thiếu GTK4 development headers.

```bash
sudo pacman -S gtk4 gtk4-layer-shell
```

### Lỗi: `cannot find -lgtk4-layer-shell`

**Nguyên nhân:** thiếu gtk4-layer-shell.

```bash
sudo pacman -S gtk4-layer-shell
# Nếu không có package: build từ source
git clone https://github.com/wmww/gtk4-layer-shell
cd gtk4-layer-shell && mkdir build && cd build
meson setup --prefix=/usr
ninja && sudo ninja install
```

### Lỗi: `WAYLAND_DISPLAY is not set`

**Nguyên nhân:** đang chạy trong phiên X11 hoặc terminal thiếu biến môi trường Wayland.

```bash
echo $WAYLAND_DISPLAY   # Phải trả về tên socket như wayland-1
echo $XDG_SESSION_TYPE  # Phải trả về "wayland"
```

### Lỗi: `error[E0433]: failed to resolve: use of undeclared crate`

**Nguyên nhân:** quên thêm dependency vào `Cargo.toml` của crate.

```toml
[dependencies]
babydra-core = { path = "libs/babydra-core" }
babydra-ui-kit = { path = "libs/babydra-ui-kit" }
```

(hoặc `{ workspace = true }` nếu khai báo trong `[workspace.dependencies]` của Cargo.toml gốc)

### Lỗi: `Text file busy` (ETXTBSY) khi cập nhật binary

**Nguyên nhân:** tiến trình cũ vẫn đang chạy.

```bash
killall babydra-panel babydra-switcher babydra-screenshot babydra-lock \
    babydra-launcher babydra-preview babydra-settings babydra-explore || true
```

(Bộ cài đặt TUI và `update.sh` đã tự xử lý bước này.)

### Lỗi: Screenshot không hoạt động

Cài `grim slurp wl-clipboard` và kiểm tra đang chạy trên phiên Wayland.

### Build chậm lần đầu

Build lần đầu mất 5–15 phút vì compile toàn bộ dependencies. Từ lần thứ hai Cargo cache lại, chỉ rebuild phần thay đổi (5–30 giây).
