# Tổng quan dự án BabyDra

**Phạm vi:** BabyDra là gì, mục tiêu thiết kế, các thành phần hệ thống, mô hình phân phối.
**Phiên bản:** 1.3.0
**Cập nhật lần cuối:** 2026-08-17

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

BabyDra là một **môi trường desktop Linux nhẹ** (lightweight Desktop Environment) viết bằng **Rust**, dùng **GTK4** kết hợp **GTK4 Layer Shell** để dựng giao diện, chạy trên compositor **labwc** (Wayland).

Dự án dành cho máy cấu hình trung bình, ưu tiên:

- **Tốc độ phản hồi tức thì** — mọi thao tác (phím tắt, click, mở panel) cho kết quả dưới 10ms nhờ mô hình Daemon-Client (xem [architecture](../architecture/index.md)).
- **Tính thẩm mỹ cao** — ngôn ngữ thiết kế **Glassmorphism**: nền kính mờ, bo tròn góc, đồng bộ hai chế độ Dark/Light (xem [design](../design/README.md)).
- **Cấu trúc module hóa rõ ràng** — mã nguồn tách thành các crate ứng dụng và thư viện dùng chung, giao tiếp qua D-Bus, Unix Domain Socket hoặc gọi hàm nội bộ.

**Giải thích thuật ngữ:**

| Thuật ngữ | Ý nghĩa |
| :--- | :--- |
| Desktop Environment (DE) | Bộ phần mềm cung cấp giao diện đồ họa cho Linux — taskbar, window manager, launcher, lock screen… |
| GTK4 | Thư viện GUI phổ biến trên Linux, cung cấp widget như nút bấm, ô nhập liệu, cửa sổ |
| Wayland | Giao thức hiển thị thế hệ mới thay thế X11 — hiệu năng tốt hơn, bảo mật cao hơn |
| labwc | Wayland compositor tối giản (stacking compositor), đóng vai trò window manager |
| GTK4 Layer Shell | Phần mở rộng định vị cửa sổ vào tầng (layer) riêng của Wayland — dùng cho panel, lock screen, switcher |

---

## 2. Mục tiêu thiết kế

### 2.1. Phản hồi tức thì (Instant Response)

Mọi thao tác phải cho kết quả dưới **10ms**. Để đạt được điều này, dự án dùng mô hình **Daemon-Client**: giao diện được giữ sẵn trong bộ nhớ, chỉ bật/tắt hiển thị thay vì khởi động lại từ đầu. Chi tiết xem [architecture](../architecture/index.md).

### 2.2. Thẩm mỹ cao (High-quality Aesthetics)

Giao diện dùng ngôn ngữ thiết kế **Glassmorphism**: nền bán trong suốt, hiệu ứng làm mờ phía sau, góc bo tròn mềm mại. Hỗ trợ cả chế độ sáng (Light) và tối (Dark). Chi tiết xem [design](../design/README.md).

### 2.3. Cấu trúc module hóa và khả năng độc lập

Mã nguồn phân rã thành các crate và thư viện độc lập. Các module giao tiếp qua D-Bus, Unix Domain Socket hoặc cơ chế gọi hàm nội bộ qua thư viện dùng chung (`babydra-core`).

---

## 3. Các thành phần của hệ thống

> [!NOTE]
> Các thư mục `crates/`, `libs/`, `configs/` chỉ nằm trên nhánh **`release`**. Bộ cài đặt `install/` có trên **cả hai nhánh** — trên `main` nó là sản phẩm chính, khởi chạy qua `install.sh` ở thư mục gốc.

### 3.1. Thư viện dùng chung (`libs/`)

| Tên thư viện | Đường dẫn | Vai trò |
| :--- | :--- | :--- |
| `babydra-core` | `libs/babydra-core/` | Dịch vụ hệ điều hành (battery, wifi, bluetooth, volume, backlight, vpn, power, storage…), D-Bus, sysfs, models, i18n (en/vi), logger |
| `babydra-ui-kit` | `libs/babydra-ui-kit/` | CSS toàn cục, bộ widget dùng chung (button, card, modal, switch…), theme, icon resolver, animation, feature components cho Explore |
| `babydra-island` | `libs/babydra-island/` | Dynamic Island mở rộng được: notification, media player, overlay — xem [guides/island](../guides/island.md) |
| `babydra-theme` | `libs/babydra-theme/` | Theme engine: load theme package, resolve CSS dark/light + tokens + fonts |
| `babydra-launcher` | `crates/babydra-launcher/` | Launcher ứng dụng: fuzzy search, lưới ứng dụng, tìm kiếm file |

> [!NOTE]
> `babydra-launcher` nằm trong `crates/` nhưng là **library + binary** (có `lib.rs`). Xem [structure](../structure/index.md).

### 3.2. Ứng dụng thực thi (`crates/`)

| Tên ứng dụng | Đường dẫn | Chức năng |
| :--- | :--- | :--- |
| `babydra-panel` | `crates/babydra-panel/` | Thanh taskbar chính: dock, tray, workspace, đồng hồ + lịch + thông báo, sys monitor, control center |
| `babydra-switcher` | `crates/babydra-switcher/` | Bộ chuyển đổi cửa sổ Alt+Tab — daemon giữ overlay trong bộ nhớ, nhận tín hiệu qua Unix socket |
| `babydra-screenshot` | `crates/babydra-screenshot/` | Chụp màn hình vùng/`--full`, editor với canvas & color picker |
| `babydra-lock` | `crates/babydra-lock/` | Màn hình khóa với xác thực PAM, hỗ trợ ảnh nền tùy chỉnh |
| `babydra-greeter` | `crates/babydra-greeter/` | Màn hình đăng nhập tương thích **greetd** (chạy trong cage), xác thực PAM |
| `babydra-settings` | `crates/babydra-settings/` | Trung tâm cấu hình: appearance, apps, bluetooth, certificates, displays, env, hosts, keybinds, power, startup, system info, system update |
| `babydra-preview` | `crates/babydra-preview/` | Trình xem nhanh hình ảnh (hardware-accelerated), đọc EXIF |
| `babydra-explore` | `crates/babydra-explore/` | Trình quản lý tập tin: content view (grid/list), preview, info panel, sidebar, tabs, gestures, context menu, dialogs |

### 3.3. Bộ cài đặt TUI (`install/`)

| Tên | Đường dẫn | Chức năng |
| :--- | :--- | :--- |
| `babydra-installer` | `install/` (nhánh `main`) | Wizard TUI 8 bước (Ratatui + Crossterm) triển khai BabyDra: 3 kênh nguồn, 3 preset, log realtime |

**Kênh cài đặt (Install Channel):**

| Kênh | Nguồn binary | Mô tả |
| :--- | :--- | :--- |
| **Release Channel** | Tự đồng bộ nhánh `release` (git fetch + `cargo build --release`) | Bản chính thức ổn định |
| **Develop Channel** | Tự đồng bộ nhánh `develop` | Bản thử nghiệm cộng đồng |
| **Local Source** | Thư mục binary có sẵn trên máy (mặc định `target/release/`) | Copy trực tiếp, không build |

Trình chọn kênh hiển thị **metadata của nhánh** (tên nhánh, commit hash, tác giả, ngày cập nhật). Chi tiết 8 bước wizard xem [structure](../structure/index.md) và [setup](../setup/index.md).

### 3.4. Cấu hình và Script hệ thống (`configs/` + script gốc)

| Thành phần | Vai trò |
| :--- | :--- |
| `configs/labwc/` | `rc.xml` (phím tắt, window rules), `autostart`, `themerc-override`, `themes/` (dark/light), `scripts/` (bat_saver.sh, switcher.sh), `fonts.conf`, `settings.ini` |
| `configs/kitty/` | Cấu hình terminal kitty |
| `configs/nvim/` | Cấu hình Neovim (keymap viewer, sidebar) |
| `configs/fastfetch/` | Cấu hình fastfetch + logo |
| `configs/themes/` | GTK theme BabyDra, bộ icon We10X, bộ cursor Twilight |
| `start.sh` | Khởi động: ghi config labwc, đăng ký .desktop entries, MIME, chạy labwc |
| `update.sh` | Hot-update: rebuild, dừng tiến trình cũ, copy binary mới, sync config, restart panel |

---

## 4. Mô hình phân phối và phân nhánh

Kho mã nguồn áp dụng mô hình 3 nhánh chính (tất cả do tác giả quản lý):

1. **`main`** — kênh phân phối tinh gọn: **chỉ chứa bộ cài đặt** (`install/`) và tài liệu.
2. **`release`** — **nhánh mặc định**: mã nguồn chính thức ổn định do tác giả push lên (toàn bộ `crates/`, `libs/`, `configs/`, `themes/`, `variants/`, `scripts/`, `tests/`).
3. **`develop`** — nhánh phát triển, tách ra từ `release`.

Không ai ngoài tác giả có thể push vào `main`/`release`/`develop`. Người đóng góp
tạo nhánh riêng từ `develop` và chỉ làm việc trong nhánh của mình. Bộ cài đặt
(`babydra-installer`) liệt kê toàn bộ nhánh có thể cài đặt (trừ `main`) để build
và triển khai từ bất kỳ nhánh nào trong số đó.

---

## 5. Sơ đồ kiến trúc tổng thể

```text
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
                                                         | babydra-core (libs/)  |
                                                         | services / models /   |
                                                         | i18n / config         |
                                                         +------------------------+
                                                                     |
                                                                     v
                                                         +------------------------+
                                                         | Hệ thống Arch Linux    |
                                                         | /sys, /proc, D-Bus,    |
                                                         | systemd, PipeWire      |
                                                         +------------------------+
```

Toàn bộ giao diện được tô màu bởi CSS dùng chung từ `babydra-ui-kit` (nạp qua `init_theme()`), đồng bộ giữa mọi ứng dụng.

---

## 6. Bảng tra cứu cho lập trình viên

| Yêu cầu | Vị trí mã nguồn | Tài liệu tham chiếu |
| :--- | :--- | :--- |
| CSS và giao diện | `libs/babydra-ui-kit/src/styles/` | [design](../design/README.md), [structure](../structure/index.md) |
| Dịch vụ phần cứng và OS | `libs/babydra-core/src/services/` | [architecture](../architecture/index.md) |
| Cấu trúc widget giao diện | Tách biệt `mod.rs` và `render.rs` | [structure](../structure/index.md) |
| Đa ngôn ngữ (i18n) | `libs/babydra-core/src/i18n/` + `locales/*/en.json`, `vi.json` | [apis/core](../apis/core.md) |
| Hướng dẫn biên dịch | `cargo build --release --workspace` | [setup](../setup/index.md) |
| Mở rộng Dynamic Island | `libs/babydra-island/` | [guides/island](../guides/island.md) |
| Quy trình phân nhánh | Git workflow | [CONTRIBUTING](../../CONTRIBUTING.md), [structure](../structure/index.md) |
