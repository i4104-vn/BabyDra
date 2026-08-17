# Chương 04: Hướng dẫn Cài đặt và Build Dự án

**Phiên bản:** 1.2.0
**Cập nhật lần cuối:** 2026-08-14
**Phạm vi:** Yêu cầu hệ thống, cài đặt dependencies, build, chạy từng crate, cài đặt DE và xử lý lỗi

---

## Mục lục

- [1. Yêu cầu hệ thống](#1-yêu-cầu-hệ-thống)
- [2. Cài đặt Dependencies](#2-cài-đặt-dependencies)
- [3. Clone và chuẩn bị dự án](#3-clone-và-chuẩn-bị-dự-án)
- [4. Cài đặt qua Bộ cài đặt TUI (khuyến nghị)](#4-cài-đặt-qua-bộ-cài-đặt-tui-khuyến-nghị)
- [5. Build từ mã nguồn (nhánh release)](#5-build-từ-mã-nguồn-nhánh-release)
- [6. Chạy từng crate ứng dụng](#6-chạy-từng-crate-ứng-dụng)
- [7. Script hệ thống: start.sh, update.sh, install.sh](#7-script-hệ-thống-startsh-updatesh-installsh)
- [8. Xử lý lỗi thường gặp](#8-xử-lý-lỗi-thường-gặp)

---

## 1. Yêu cầu hệ thống

### Hệ điều hành

- **Arch Linux** hoặc các bản phân phối tương thích (baby-dra được thiết kế cho Arch, dùng `pacman` và `yay`).
- **Wayland compositor** đang chạy — mặc định là **labwc**. Không hỗ trợ X11.
- Kiểm tra: `echo $XDG_SESSION_TYPE` phải trả về `wayland`.

### Phiên bản Rust

- **Rust 1.80.0 trở lên** (stable channel) — theo yêu cầu chính thức trong README.
- Kiểm tra phiên bản hiện tại: `rustc --version`

### Thư viện hệ thống cần thiết

| Thư viện | Gói cần cài (Arch Linux) | Dùng cho |
| :--- | :--- | :--- |
| GTK4 | `gtk4` | Bộ công cụ giao diện đồ họa |
| GTK4 Layer Shell | `gtk4-layer-shell` | Định vị cửa sổ trên Wayland |
| GLib | `glib2` | Nền tảng cho GTK |
| D-Bus | `dbus` | Giao tiếp liên tiến trình |
| PipeWire | `pipewire` | Điều khiển âm thanh |
| pkg-config | `pkgconf` | Công cụ tìm thư viện khi build |
| labwc | `labwc` | Wayland compositor / window manager |
| greetd + cage | `greetd cage` | Màn hình đăng nhập |
| playerctl | `playerctl` | Điều khiển media player (island) |
| grim + slurp | `grim slurp` | Chụp màn hình |
| wl-clipboard | `wl-clipboard` | Clipboard (screenshot copy) |
| ddcutil | `ddcutil` | Điều khiển độ sáng màn hình ngoài |

> [!NOTE]
> Nếu dùng **Bộ cài đặt TUI** (xem mục 4), toàn bộ dependencies trên được cài tự động qua bước "System Packages & Deps" — không cần cài tay.

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
# Cài yay từ AUR
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
# Clone repository (mặc định nhánh main — kênh phân phối)
git clone <địa_chỉ_repository> BabyDra
cd BabyDra

# Kiểm tra Rust toolchain
rustc --version
cargo --version
```

Hai cách sử dụng:

| Nhánh | Mục đích |
| :--- | :--- |
| `main` | Người dùng cuối: chạy bộ cài đặt TUI (mục 4) |
| `release` | Developer: build toàn bộ mã nguồn DE (mục 5) |

```bash
# Chuyển sang nhánh mã nguồn đầy đủ (developer)
git fetch origin release
git checkout release
```

---

## 4. Cài đặt qua Bộ cài đặt TUI (khuyến nghị)

Bộ cài đặt `babydra-installer` nằm trên nhánh `main`. Đây là cách cài đặt khuyến nghị cho người dùng cuối.

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

1. **Bước 1 (Welcome):** Nhấn `c` để chọn kênh — **Release** (ổn định) / **Develop** (thử nghiệm) / **Local Source** (binary có sẵn). Trình chọn kênh hiển thị commit hash, tác giả và ngày cập nhật của từng nhánh. Dùng `j`/`k` chọn preset profile: *Full Desktop*, *Binaries & /var/lib Only*, *Custom*.
2. **Bước 2 (Packages):** Chọn cài pacman deps, AUR packages, kernel/i2c permissions.
3. **Bước 3 (Binaries):** Chọn 9 binary component (panel, switcher, screenshot, lock, launcher, preview, settings, explore, greeter) — binary nào có trong thư mục nguồn sẽ được đánh dấu sẵn.
4. **Bước 4 (/var/lib):** Stage binaries, wallpapers, logos vào `/var/lib/babydra` (chmod 777).
5. **Bước 5 (Configs & Themes):** Sync labwc configs, .desktop entries, dotfiles (GTK/fontconfig/kitty/nvim/fastfetch), themes/icons/cursors, gsettings.
6. **Bước 6 (Display Manager):** Cấu hình greetd (`cage` + `babydra-greeter`), mask gettys tty2-6, enable greetd.service.
7. **Bước 7 (Execute):** Theo dõi log realtime và progress gauge.
8. **Bước 8 (Summary):** Xem kết quả và thoát.

> [!IMPORTANT]
> Nếu chọn kênh **Release/Develop** mà binary chưa có trong thư mục nguồn, bộ cài đặt sẽ tự động `git fetch origin <branch>` và chạy `cargo build --release --workspace` để tạo binary trước khi cài. Kênh **Local Source** copy trực tiếp, không build.

### 4.3. Kết quả sau khi cài

- Binary đồ họa → `~/.local/bin/`
- `babydra-greeter` → `/usr/bin/` (cần quyền root)
- Assets staging → `/var/lib/babydra/` và `/usr/share/babydra/`
- Configs → `~/.config/labwc/`, `~/.config/kitty/`, `~/.config/nvim/`, `~/.config/fastfetch/`, `~/.config/gtk-3.0/`, `~/.config/gtk-4.0/`, `~/.config/fontconfig/`
- Themes/Icons/Cursors → `~/.local/share/themes/`, `~/.local/share/icons/`

---

## 5. Build từ mã nguồn (nhánh release)

> [!NOTE]
> Bước này cần workspace đầy đủ — chuyển sang nhánh `release` trước (`git checkout release`).

### Build toàn bộ workspace

```bash
# Release mode (khuyến nghị — binary nhỏ hơn, chạy nhanh hơn)
cargo build --release --workspace

# Debug mode
cargo build --workspace
```

Artifact nằm tại `target/release/` (hoặc `target/debug/`).

### Build một crate cụ thể

```bash
cargo build --release -p babydra-panel
cargo build --release -p babydra-explore
```

### Kiểm tra lỗi compile mà không sinh binary

```bash
cargo check --workspace
cargo check -p babydra-panel
```

### Format chuẩn Rust

```bash
cargo fmt --check
```

---

## 6. Chạy từng crate ứng dụng

**Lưu ý quan trọng:** Các ứng dụng chạy trên Wayland. Phải đang trong phiên Wayland (`labwc`, Sway, Hyprland...) mới hoạt động đúng.

### Chạy qua cargo (debug)

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

Script `start.sh` đồng bộ config labwc, đăng ký .desktop entries rồi `exec labwc` — toàn bộ DE khởi động cùng autostart (panel, switcher daemon, fcitx5...).

---

## 7. Script hệ thống: start.sh, update.sh, install.sh

| Script | Nhánh | Mục đích |
| :--- | :--- | :--- |
| `install.sh` (main) | `main` | Build & launch bộ cài đặt TUI (mục 4) |
| `install.sh` (release) | `release` | Cài đặt toàn bộ DE từ source: pacman deps, yay, i2c/CPU perms, build, deploy |
| `start.sh` | `release` | Khởi động DE: sync labwc config, .desktop entries, chạy labwc |
| `update.sh` | `release` | Hot-update: `cargo build --release`, dừng tiến trình cũ, copy binary mới, sync toàn bộ config, `labwc --reconfigure`, restart panel |

**Cập nhật DE sau khi có code mới (developer):**

```bash
git pull origin release   # hoặc rebase từ develop
./update.sh               # rebuild + deploy + restart
```

---

## 8. Xử lý lỗi thường gặp

### Lỗi: `error: failed to run custom build command for gtk4-sys`

**Nguyên nhân:** Thiếu thư viện GTK4 development headers.

**Giải pháp:**
```bash
sudo pacman -S gtk4 gtk4-layer-shell
```

### Lỗi: `cannot find -lgtk4-layer-shell`

**Nguyên nhân:** Thiếu thư viện gtk4-layer-shell.

**Giải pháp:**
```bash
sudo pacman -S gtk4-layer-shell
# Nếu không có package: build từ source
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
babydra-core = { path = "libs/babydra-core" }
babydra-ui-kit = { path = "kits/babydra-ui-kit" }
```
(hoặc `{ workspace = true }` nếu khai báo trong `[workspace.dependencies]` của Cargo.toml gốc)

### Lỗi: `Text file busy` (ETXTBSY) khi cập nhật binary

**Nguyên nhân:** Tiến trình cũ vẫn đang chạy, ghi đè executable bị chặn.

**Giải pháp:** Dừng tiến trình trước khi cập nhật:
```bash
killall babydra-panel babydra-switcher babydra-screenshot babydra-lock \
    babydra-launcher babydra-preview babydra-settings babydra-explore || true
```
(Bộ cài đặt TUI và `update.sh` đã tự xử lý bước này.)

### Lỗi: Screenshot không hoạt động

**Giải pháp:** Cài `grim slurp wl-clipboard` và kiểm tra đang chạy trên phiên Wayland.

### Build chậm lần đầu

Build lần đầu tốn nhiều thời gian (5–15 phút tùy máy) vì phải compile tất cả dependencies. Từ lần thứ hai trở đi, Cargo cache lại, chỉ rebuild những gì thay đổi, mất khoảng 5–30 giây.
