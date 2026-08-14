# Chương 01: Tổng quan Dự án BabyDra

**Phiên bản:** 1.2.0
**Cập nhật lần cuối:** 2026-08-14
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

BabyDra là một **môi trường desktop Linux nhẹ** (lightweight Linux Desktop Environment) được viết bằng ngôn ngữ lập trình **Rust**, sử dụng bộ công cụ giao diện **GTK4** kết hợp **GTK4 Layer Shell** để dựng giao diện đồ họa, chạy trên trình quản lý cửa sổ **labwc** (Wayland compositor).

Dự án được thiết kế cho các máy tính có cấu hình trung bình, ưu tiên:

- **Tốc độ phản hồi tức thì**: mọi thao tác (phím tắt, click, mở panel) hiển thị kết quả dưới 10ms nhờ mô hình Daemon-Client (xem [02-architecture.md](./02-architecture.md)).
- **Tính thẩm mỹ cao**: ngôn ngữ thiết kế **Glassmorphism** — nền kính mờ bán trong suốt, bo tròn góc, đồng bộ hai chế độ màu Dark/Light (xem [design/](./design/README.md)).
- **Cấu trúc module hóa rõ ràng**: mã nguồn được tách thành các crate ứng dụng và thư viện dùng chung, giao tiếp qua D-Bus, Unix Domain Socket hoặc gọi hàm nội bộ.

**Giải thích thuật ngữ:**

- **Desktop Environment (DE):** Bộ phần mềm cung cấp giao diện đồ họa cho hệ điều hành Linux — thanh taskbar, trình quản lý cửa sổ, launcher, màn hình khóa, v.v.
- **GTK4:** Thư viện giao diện đồ họa (GUI toolkit) phổ biến trên Linux, cung cấp sẵn các widget như nút bấm, ô nhập liệu, cửa sổ.
- **Wayland:** Giao thức hiển thị thế hệ mới trên Linux, thay thế cho X11 — hiệu năng tốt hơn và bảo mật cao hơn.
- **labwc:** Wayland compositor tối giản (stacking compositor), đóng vai trò window manager cho BabyDra.
- **GTK4 Layer Shell:** Phần mở rộng cho phép định vị cửa sổ vào các tầng (layer) riêng của Wayland — dùng cho panel, lock screen, switcher.

---

## 2. Mục tiêu thiết kế

BabyDra được xây dựng xung quanh 3 mục tiêu cốt lõi:

### 2.1. Phản hồi tức thì (Instant Response)

Mọi thao tác của người dùng (nhấn phím tắt, click chuột, mở panel) phải cho kết quả hiển thị trong vòng **dưới 10 mili-giây**. Để đạt được điều này, dự án dùng mô hình **Daemon-Client**: giao diện được giữ sẵn trong bộ nhớ, chỉ bật/tắt hiển thị thay vì khởi động lại từ đầu mỗi lần gọi. Chi tiết xem [02-architecture.md](./02-architecture.md).

### 2.2. Thẩm mỹ cao (High-quality Aesthetics)

Giao diện sử dụng ngôn ngữ thiết kế **Glassmorphism** (kính mờ): nền bán trong suốt, hiệu ứng làm mờ phía sau, góc bo tròn mềm mại. Hỗ trợ cả chế độ sáng (Light) và tối (Dark). Chi tiết xem [design/](./design/README.md).

### 2.3. Cấu trúc module hóa và khả năng độc lập

Mã nguồn được phân rã thành các crate và thư viện độc lập. Các module giao tiếp với nhau qua D-Bus, Unix Domain Socket hoặc cơ chế gọi hàm nội bộ qua thư viện dùng chung (`babydra-common`).

---

## 3. Các thành phần của hệ thống

> [!NOTE]
> Các thư mục `crates/`, `libs/`, `configs/` chỉ nằm trên nhánh **`release`**. Bộ cài đặt `install/` có trên **cả hai nhánh** — trên `main` nó là sản phẩm chính, được khởi chạy qua `install.sh` ở thư mục gốc.

### 3.1. Thư viện dùng chung (`libs/`)

| Tên thư viện | Đường dẫn | Vai trò |
|---|---|---|
| `babydra-common` | `libs/babydra-common/` | Dịch vụ hệ điều hành (battery, wifi, bluetooth, volume, backlight, vpn, power, storage...), D-Bus, sysfs, mô hình dữ liệu, i18n (en/vi), logger |
| `babydra-utils` | `libs/babydra-utils/` | CSS toàn cục (dark/light/shared), bộ widget dùng chung (button, card, modal, switch...), theme, icon resolver, animation, context menu & dialogs cho Explore |
| `babydra-island` | `libs/babydra-island/` | Widget Dynamic Island: thông báo, overlay âm lượng/độ sáng, media player (playerctl + visualizer) |
| `babydra-launcher` | `libs/babydra-launcher/` | Launcher ứng dụng: tìm kiếm mờ (fuzzy search), lưới ứng dụng, tìm kiếm file |

### 3.2. Ứng dụng thực thi (`crates/`)

| Tên ứng dụng | Đường dẫn | Chức năng |
|---|---|---|
| `babydra-panel` | `crates/babydra-panel/` | Thanh taskbar chính: dock, khay hệ thống (tray), workspace, đồng hồ + lịch + thông báo, sys monitor, control center; các item backlight/bluetooth/clean/storage/volume/vpn/wifi |
| `babydra-switcher` | `crates/babydra-switcher/` | Bộ chuyển đổi cửa sổ Alt+Tab — chạy daemon giữ overlay trong bộ nhớ, nhận tín hiệu qua Unix socket `/tmp/babydra-switcher.socket` |
| `babydra-screenshot` | `crates/babydra-screenshot/` | Công cụ chụp màn hình: chụp vùng (regional), chụp toàn màn hình (`--full`), editor chỉnh sửa với canvas & color picker |
| `babydra-lock` | `crates/babydra-lock/` | Màn hình khóa với xác thực PAM, hỗ trợ hình nền tùy chỉnh (`--image`), map tới mọi màn hình |
| `babydra-greeter` | `crates/babydra-greeter/` | Màn hình đăng nhập hệ thống tương thích **greetd** (chạy trong cage compositor), xác thực PAM, splash screen |
| `babydra-settings` | `crates/babydra-settings/` | Trung tâm cấu hình hệ thống: appearance, apps, bluetooth, certificates, displays, env, hosts, keybinds, power (battery saver), startup, system info, system update; CLI helper (`--apply-battery-saver`, `--set-power-profile`...) |
| `babydra-preview` | `crates/babydra-preview/` | Trình xem nhanh hình ảnh (hardware-accelerated), đọc EXIF, tích hợp file dialog |
| `babydra-explore` | `crates/babydra-explore/` | Trình quản lý tập tin đồ họa: content view (grid/list), preview panel, info panel, sidebar, tab bar, settings dialog, gestures, context menu, dialogs (confirm, conflict, archive, alert) |

### 3.3. Bộ cài đặt TUI (`install/`)

| Tên | Đường dẫn | Chức năng |
|---|---|---|
| `babydra-installer` | `install/` (nhánh `main`) | Wizard TUI 8 bước (Ratatui + Crossterm) triển khai BabyDra: 3 kênh nguồn, 3 preset, log realtime |

**Kênh cài đặt (Install Channel):**

| Kênh | Nguồn binary | Mô tả |
|---|---|---|
| **Release Channel** | Tự đồng bộ nhánh `release` (git fetch + `cargo build --release`) | Bản chính thức ổn định |
| **Develop Channel** | Tự đồng bộ nhánh `develop` | Bản thử nghiệm cộng đồng |
| **Local Source** | Thư mục binary có sẵn trên máy (mặc định `target/release/`) | Copy trực tiếp, không build |

Trình chọn kênh hiển thị **metadata của nhánh** (tên nhánh, commit hash, tác giả, ngày cập nhật) để người dùng biết chính xác bản đang cài. Chi tiết về 8 bước wizard tại [03-project-structure.md](./03-project-structure.md) và [04-setup-and-build.md](./04-setup-and-build.md).

### 3.4. Cấu hình và Script hệ thống (`configs/` + script gốc)

| Thành phần | Vai trò |
|---|---|
| `configs/labwc/` | `rc.xml` (phím tắt, window rules), `autostart`, `themerc-override`, `themes/` (dark/light), `scripts/` (bat_saver.sh, switcher.sh), `fonts.conf`, `settings.ini` (GTK) |
| `configs/kitty/` | Cấu hình terminal kitty |
| `configs/nvim/` | Cấu hình Neovim (keymap viewer, sidebar với các tab agents/changes/review...) |
| `configs/fastfetch/` | Cấu hình fastfetch + logo |
| `configs/themes/` | GTK theme BabyDra, bộ icon We10X, bộ cursor Twilight |
| `start.sh` | Script khởi động: ghi config labwc, đăng ký .desktop entries, MIME, khởi chạy labwc |
| `update.sh` | Script hot-update: rebuild, dừng tiến trình cũ, copy binary mới, đồng bộ toàn bộ config, gsettings, khởi động lại panel |

---

## 4. Mô hình phân phối và phân nhánh

Kho mã nguồn BabyDra áp dụng mô hình phân tách 3 nhánh:

1. **`main`**: Nhánh phân phối tinh gọn — chỉ chứa công cụ cài đặt `babydra-installer`, script thực thi (`install.sh`) và tài liệu hướng dẫn (`docs/`, `README.md`, `WORKFLOW.md`).
2. **`release`**: Nhánh mã nguồn chính thức và ổn định do tác giả duy trì — chứa toàn bộ `crates/`, `libs/`, `configs/`, `start.sh`, `update.sh`.
3. **`develop`**: Nhánh phát triển chung được checkout từ `release`, phục vụ việc đóng góp tính năng mới từ cộng đồng. Các developer tạo nhánh `feature/<tên-user>` từ đây.

Quy trình phân nhánh chi tiết được quy định tại [WORKFLOW.md](../WORKFLOW.md).

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
                                                         | services / models / i18n |
                                                         +------------------------+
                                                                     |
                                                                     v
                                                         +------------------------+
                                                         | Hệ thống Arch Linux    |
                                                         | /sys, /proc, D-Bus,    |
                                                         | systemd, PipeWire      |
                                                         +------------------------+
```

Toàn bộ giao diện được tô màu bởi CSS dùng chung từ `libs/babydra-utils/src/styles/` (nạp qua `babydra_utils::ui::theme::init_theme()`), đồng bộ giữa mọi ứng dụng.

---

## 6. Bảng tra cứu cho lập trình viên

| Yêu cầu | Vị trí mã nguồn | Tài liệu tham chiếu |
|---|---|---|
| CSS và giao diện | `libs/babydra-utils/src/styles/` (nhánh `release`) | [design/](./design/README.md), [03-project-structure.md](./03-project-structure.md) |
| Dịch vụ phần cứng và OS | `libs/babydra-common/src/services/` (nhánh `release`) | [02-architecture.md](./02-architecture.md) |
| Cấu trúc widget giao diện | Tách biệt `mod.rs` và `render.rs` | [03-project-structure.md](./03-project-structure.md) |
| Đa ngôn ngữ (i18n) | `libs/babydra-common/src/i18n/` + `locales/*/en.json`, `vi.json` | [03-project-structure.md](./03-project-structure.md) |
| Hướng dẫn biên dịch | `cargo build --release --workspace` | [04-setup-and-build.md](./04-setup-and-build.md) |
| Quy trình phân nhánh và merge | Git workflow | [../WORKFLOW.md](../WORKFLOW.md) |
