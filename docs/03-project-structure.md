# Chương 03: Cấu trúc Dự án và Quy chuẩn Viết mã

**Phiên bản:** 1.3.0
**Cập nhật lần cuối:** 2026-08-17
**Phạm vi:** Quy chuẩn đặt tên thư mục, triết lý phân tách file, trách nhiệm từng module, quy tắc viết mã nguồn mới

---

## Mục lục

- [1. Tổng quan cấu trúc thư mục](#1-tổng-quan-cấu-trúc-thư-mục)
- [2. Quy chuẩn đặt tên thư mục](#2-quy-chuẩn-đặt-tên-thư-mục)
- [3. Triết lý phân tách file mã nguồn](#3-triết-lý-phân-tách-file-mã-nguồn)
- [4. Bộ cài đặt TUI (`install/`)](#4-bộ-cài-đặt-tui-install)
- [5. Nhóm ứng dụng đồ họa (`crates/`)](#5-nhóm-ứng-dụng-đồ-họa-crates)
- [6. Nhóm thư viện dùng chung (`libs/`)](#6-nhóm-thư-viện-dùng-chung-libs)
- [7. Cấu hình hệ thống và Script (`configs/`, `start.sh`, `update.sh`)](#7-cấu-hình-hệ-thống-và-script-configs-startsh-updatesh)
- [8. Tài liệu (`docs/`)](#8-tài-liệu-docs)
- [9. Quy tắc chung khi viết mã nguồn mới](#9-quy-tắc-chung-khi-viết-mã-nguồn-mới)

---

## 1. Tổng quan cấu trúc thư mục

> [!IMPORTANT]
> Kho mã nguồn được phân tách theo mô hình 3 nhánh (xem [WORKFLOW.md](../WORKFLOW.md)):
> - **Nhánh `main`** — chỉ chứa `install/`, `install.sh`, tài liệu.
> - **Nhánh `release`/`develop`** — chứa thêm `crates/`, `libs/`, `configs/`, `start.sh`, `update.sh`, `wallpaper.png`.

### 1.1. Nhánh `main` (Kênh phân phối)

```
BabyDra/                          <- Thư mục gốc repository (nhánh main)
├── Cargo.toml                    <- Workspace manifest — members: ["install"]
├── Cargo.lock                    <- Khóa phiên bản dependency (không chỉnh tay)
├── install.sh                    <- Script khởi chạy bộ cài đặt TUI
├── README.md                     <- Hướng dẫn tổng quan người dùng cuối
├── WORKFLOW.md                   <- Quy chuẩn phân nhánh và phát triển
├── docs/                         <- Toàn bộ tài liệu dự án (xem mục 8)
└── install/                      <- Bộ cài đặt TUI (babydra-installer)
```

### 1.2. Nhánh `release` (Mã nguồn chính thức)

```
BabyDra/                          <- Thư mục gốc repository (nhánh release)
├── Cargo.toml                    <- Workspace manifest — liệt kê toàn bộ crates/libs/install/tests
├── Cargo.lock
├── scripts/                      <- Scripts: install.sh, start.sh, update.sh, check.sh (đã gom lại)
├── wallpaper.png                 <- Hình nền mặc định của hệ thống
├── README.md
├── WORKFLOW.md                   <- Mô hình branch + ma trận sở hữu (chống conflict)
├── CONTRIBUTING.md               <- Checklist PR
├── CHANGELOG.md                  <- Lịch sử phiên bản (SemVer)
├── docs/                         <- Tài liệu (đồng bộ với nhánh main)
├── configs/                      <- Cấu hình mẫu hệ thống (seed)
├── crates/                       <- Các ứng dụng đồ họa thực thi độc lập
├── libs/                         <- Các thư viện dùng chung (không thể chạy độc lập)
├── themes/                       <- Theme packages (tokens.json + theme.css + fonts.json)
├── variants/                     <- Variants (mỗi variant 1 thư mục riêng)
├── tests/                        <- Integration test suite (TDD safety net)
└── install/                      <- Bộ cài đặt TUI (dùng chung với nhánh main)
```

**Workspace `release` (`Cargo.toml`):**

```toml
[workspace]
resolver = "2"
members = [
    "libs/babydra-common",    "libs/babydra-island",
    "libs/babydra-launcher",  "libs/babydra-theme",
    "libs/babydra-utils",     "libs/babydra-explore-kit",
    "crates/babydra-panel",   "crates/babydra-switcher",
    "crates/babydra-screenshot", "crates/babydra-lock",
    "crates/babydra-preview", "crates/babydra-settings",
    "crates/babydra-explore", "crates/babydra-greeter",
    "install", "tests",
]
```

---

## 2. Quy chuẩn đặt tên thư mục

### 2.1. Tầng gốc: kebab-case

**kebab-case** là định dạng viết tên bằng chữ thường, các từ phân cách nhau bằng dấu gạch nối (`-`).

Tất cả crate ứng dụng và thư viện ở tầng gốc (`libs/` và `crates/`) phải dùng kebab-case:

| Đúng | Sai | Lý do |
| :--- | :--- | :--- |
| `babydra-panel` | `babydraPanel` | camelCase không được phép |
| `babydra-common` | `babydra_common` | snake_case chỉ dùng bên trong `src/` |
| `babydra-screenshot` | `BabyDraScreenshot` | PascalCase không được phép |

Ví dụ đúng:
- `libs/babydra-common/`
- `libs/babydra-utils/`
- `crates/babydra-panel/`
- `crates/babydra-explore/`

### 2.2. Tầng bên trong `src/`: snake_case

**snake_case** là định dạng viết tên bằng chữ thường, các từ phân cách nhau bằng dấu gạch dưới (`_`).

Tất cả thư mục con và file mã nguồn bên trong thư mục `src/` của bất kỳ crate nào đều phải dùng snake_case:

| Đúng | Sai | Lý do |
| :--- | :--- | :--- |
| `src/widgets/` | `src/Widgets/` | PascalCase không được phép |
| `src/system_tray/` | `src/systemTray/` | camelCase không được phép |
| `src/control_center/` | `src/control-center/` | kebab-case chỉ dùng ở tầng crate |
| `src/main.rs` | `src/Main.rs` | Chỉ `main.rs`/`lib.rs` được đặc cách |

Ví dụ đúng:
- `crates/babydra-panel/src/widgets/panel/items/volume/`
- `libs/babydra-common/src/services/system/backlight/`

### 2.3. File CSS: snake_case

Các file CSS trong `libs/babydra-utils/src/styles/` dùng snake_case, nhóm theo đối tượng:

- `panel/panel.css`, `panel/taskbar.css`, `panel/clock.css`
- `explore/content_view.css`, `explore/context_menu.css`
- `apps/settings.css`, `apps/switcher.css`
- `shared/button.css`, `shared/switch.css`

---

## 3. Triết lý phân tách file mã nguồn

Mỗi module đồ họa trong `crates/` và `libs/` phải tuân thủ cấu trúc phân tách logic và giao diện:

| File | Trách nhiệm |
| :--- | :--- |
| `mod.rs` | Khai báo module, định nghĩa kiểu dữ liệu, State struct và cấu trúc điều khiển |
| `render.rs` | Xây dựng cây phân cấp widget giao diện bằng GTK4 (thuần khai báo UI) |
| `handlers.rs` | Xử lý các sự kiện tương tác từ người dùng (click, phím bấm, thay đổi trạng thái) |
| `actions.rs` | Các hành động nghiệp vụ cụ thể của widget (copy, delete, create...) |

> [!IMPORTANT]
> **Quy tắc chống trộn lẫn:** `render.rs` **không được** chứa `std::process::Command`, `std::fs::read` hay logic nghiệp vụ — mọi thao tác hệ thống phải gọi qua `babydra-common`.

---

## 4. Bộ cài đặt TUI (`install/`)

```
install/
├── Cargo.toml                    <- Dependencies: ratatui 0.29, crossterm 0.28, chrono, dirs, anyhow, libc
├── README.md                     <- Hướng dẫn sử dụng nhanh
├── run.sh                        <- Script thực thi nhanh (chỉ có trên nhánh release)
└── src/
    ├── main.rs                   <- Entry point: parse CLI args, raw mode, event loop 50ms tick
    │                                (trên main, entry point là install.sh ở thư mục gốc)
    ├── app/
    │   ├── mod.rs                <- Struct App: toàn bộ trạng thái wizard, channel, profiles, logs
    │   └── handlers.rs           <- Xử lý toàn bộ phím bấm theo từng step/modal
    ├── models/
    │   ├── mod.rs
    │   ├── step.rs               <- enum WizardStep (8 bước) + ALL/next/prev
    │   ├── options.rs            <- InstallChannel (Release/Develop/LocalSource), PresetProfile,
    │   │                             BranchMetadata, GenericOptionItem
    │   ├── binary.rs             <- BinaryItem, BinaryLocation (UserLocalBin/SystemBin)
    │   └── log.rs                <- LogLevel, LogMessage
    ├── system/
    │   ├── mod.rs                <- Helper: find_workspace_root, default_binary_source_dir, is_root,
    │   │                             safe_copy_binary, stop_process, format_size...
    │   ├── initializers.rs       <- Danh sách 9 binaries, 3 package options, 4 varlib options,
    │   │                             7 configs/themes options, 3 display manager options
    │   ├── git_ops.rs            <- fetch_branch_metadata: git log -1 để lấy commit hash/author/date
    │   ├── fs_ops.rs             <- Thao tác file: copy, chmod, tar extraction
    │   └── process.rs            <- Dừng tiến trình cũ (killall), spawn worker
    ├── tasks/
    │   ├── mod.rs                <- InstallPlan, InstallEvent, spawn_installation_worker
    │   ├── packages.rs           <- Cài pacman deps / AUR (yay) / kernel permissions
    │   ├── binaries.rs           <- Copy binary vào ~/.local/bin hoặc /usr/bin (sudo fallback)
    │   ├── varlib.rs             <- Stage binaries/wallpapers/logos vào /var/lib/babydra, chmod 777
    │   ├── configs.rs            <- Sync labwc, .desktop entries, dotfiles, themes, gsettings
    │   └── display_manager.rs    <- Cấu hình greetd (cage + babydra-greeter), mask gettys
    └── ui/
        ├── mod.rs
        ├── layout.rs             <- Header, sidebar (8 steps + plan summary), footer (key hints)
        ├── modals/
        │   ├── help.rs           <- Bảng phím tắt
        │   ├── confirm.rs        <- Dialog xác nhận bắt đầu cài đặt
        │   └── edit_path.rs      <- Nhập đường dẫn thư mục binary tùy chỉnh
        └── steps/
            ├── welcome.rs        <- Step 1: chọn kênh (c) + preset profile (j/k)
            ├── packages.rs       <- Step 2: hệ thống packages & dependencies
            ├── binaries.rs       <- Step 3: chọn 9 binary component
            ├── varlib.rs         <- Step 4: /var/lib staging bundle
            ├── configs.rs        <- Step 5: configs, themes & icons
            ├── display_manager.rs<- Step 6: greetd display manager
            ├── progress.rs       <- Step 7: execute + real-time log stream + progress gauge
            └── summary.rs        <- Step 8: kết quả cài đặt
```

### 4.1. 8 bước Wizard

| Bước | Tên | Mô tả |
| :--- | :--- | :--- |
| 1 | Welcome & Overview | Chọn kênh (Release/Develop/LocalSource), chọn preset profile |
| 2 | System Packages & Deps | Pacman deps, AUR (yay), kernel/i2c permissions |
| 3 | BabyDra Binaries | Chọn binary để cài vào `~/.local/bin` / `/usr/bin` |
| 4 | /var/lib Staging Bundle | Stage binaries, wallpapers, logos vào `/var/lib/babydra` |
| 5 | Configs, Themes & Icons | labwc configs, .desktop entries, dotfiles, themes/icons/cursors |
| 6 | Greetd Display Manager | Cấu hình greetd + mask VTs + enable service |
| 7 | Execute Installation | Chạy worker, theo dõi log realtime |
| 8 | Summary & Launch | Kết quả và thoát |

### 4.2. 3 kênh cài đặt (Install Channel)

- **Release Channel** — kéo nhánh `release` (git fetch) + build `cargo build --release --workspace` nếu binary chưa có, rồi copy.
- **Develop Channel** — tương tự với nhánh `develop`.
- **Local Source** — copy trực tiếp binary có sẵn trong thư mục nguồn (mặc định `target/release/`, có thể chỉnh bằng `s`).

### 4.3. 3 Preset Profile

- **Full Desktop (Recommended)** — cài tất cả: binaries + varlib + configs/themes + display manager.
- **Binaries & /var/lib Staging Only** — chỉ binary + staging, bỏ qua dotfiles và display manager.
- **Custom Selection** — người dùng tự chọn từng mục.

### 4.4. Phím tắt chính

| Phím | Hành động |
| :--- | :--- |
| `1`–`8` | Nhảy thẳng tới bước tương ứng |
| `Tab` / `n`, `BackTab` / `p` | Bước kế tiếp / trước đó |
| `↑`/`↓`/`j`/`k`, `Space`, `a`/`A` | Di chuyển, toggle, chọn/bỏ chọn tất cả |
| `c` | Bước 1: đổi kênh cài đặt — Bước 7: xóa log buffer |
| `s` | Đổi thư mục nguồn binary |
| `r` | Quét lại thư mục nguồn |
| `i` / `Enter` | Bắt đầu cài đặt (mở dialog xác nhận) |
| `?` | Mở help modal |
| `q` / `Ctrl+C` | Thoát |

---

## 5. Nhóm ứng dụng đồ họa (`crates/`)

> [!NOTE]
> Thư mục này chỉ tồn tại trên nhánh `release`/`develop`.

### 5.1. `babydra-panel` — Thanh taskbar chính

```
crates/babydra-panel/src/
├── main.rs                       <- Khởi động tray watcher, DDC detection, switcher tracker, GTK app
├── render.rs                     <- build_panel_ui: panel + control center + calendar + launcher windows
└── widgets/
    ├── mod.rs
    ├── panel/
    │   ├── mod.rs, render.rs, modal.rs, toggle_grid.rs
    │   ├── items/                <- header, wifi, bluetooth, volume, vpn, backlight, storage, clean
    │   └── popover/              <- network, volume, vpn, battery
    ├── clock/                    <- Đồng hồ + calendar_window + notifications (notification_group)
    ├── sys_monitor/              <- CPU/RAM monitor
    ├── tray/                     <- Khay hệ thống (StatusNotifier)
    └── workspace/                <- Workspace + preview
```

### 5.2. `babydra-switcher` — Alt-Tab Switcher

```
crates/babydra-switcher/src/
├── main.rs                       <- Hai chế độ: --daemon (giữ overlay trong bộ nhớ) / one-shot client
├── daemon.rs                     <- Lắng nghe Unix socket /tmp/babydra-switcher.socket
├── render.rs
└── widgets/                      <- Overlay danh sách cửa sổ + preview
```

### 5.3. `babydra-screenshot` — Chụp màn hình

```
crates/babydra-screenshot/src/
├── main.rs                       <- --full: chụp toàn màn hình; mặc định: chụp vùng (slurp/grim)
└── widgets/
    ├── editor.rs                 <- Editor sau khi chụp
    ├── canvas.rs                 <- Canvas vẽ/vùng chọn
    └── color_popover.rs          <- Chọn màu
```

### 5.4. `babydra-lock` — Màn hình khóa

```
crates/babydra-lock/src/
├── main.rs                       <- Parse --image, tạo GTK app, map window tới mọi màn hình
├── render.rs                     <- build_lock_ui
└── widgets/                      <- Locker UI (xác thực PAM qua babydra_common::verify_password)
```

### 5.5. `babydra-greeter` — Màn hình đăng nhập (greetd)

```
crates/babydra-greeter/src/
├── main.rs                       <- init_logger, đọc GREETD_SOCK/WAYLAND_DISPLAY, GTK app
├── auth.rs                       <- Xác thực qua greetd protocol
├── handlers.rs                   <- setup_handlers
├── render.rs                     <- build_greeter_ui
├── theme.rs
└── widgets/                      <- login, splash, top_bar
```

### 5.6. `babydra-settings` — Trung tâm cấu hình

```
crates/babydra-settings/src/
├── main.rs                       <- CLI helpers: --apply-battery-saver, --check-battery-saver,
│                                    --set-power-profile, --sync-greeter-wallpaper
├── layout.rs
└── widgets/
    ├── appearance/               <- Giao diện: theme, wallpaper, avatar
    ├── apps/                     <- Quản lý ứng dụng (launch, update, uninstall)
    ├── bluetooth/
    ├── certificates/             <- CA certificates
    ├── displays/                 <- Màn hình (save/apply qua babydra_common::system::display)
    ├── env/                      <- Biến môi trường
    ├── hosts/                    <- File /etc/hosts
    ├── keybinds/                 <- Phím tắt
    ├── power/                    <- Power profile + battery_card (battery saver tự động)
    ├── startup/                  <- Ứng dụng khởi động cùng hệ thống
    ├── system_info/              <- Thông tin hệ thống
    └── system_update/            <- Cập nhật hệ thống (pacman) với log realtime
```

### 5.7. `babydra-preview` — Xem nhanh hình ảnh

```
crates/babydra-preview/src/
├── main.rs                       <- Nhận đường dẫn ảnh từ argv, fallback FileDialog
├── exif_reader.rs                <- Đọc metadata EXIF
└── widgets/
    └── viewer.rs                 <- Viewer zoom/pan
```

### 5.8. `babydra-explore` — Trình quản lý tập tin

```
crates/babydra-explore/src/
├── main.rs                       <- tokio runtime + SessionState + create_explore_window
└── widgets/
    ├── window/                   <- layout (split/preview), handlers (events/navigation), widgets (tabs)
    ├── content_view/             <- rendering (grid_renderer/list_renderer), gestures (background,
    │                                clipboard, flowbox, listbox), items (grid_item)
    ├── header_bar/               <- Thanh địa chỉ + điều hướng
    ├── sidebar/                  <- Cây thư mục + bookmarks
    ├── preview_panel/            <- Xem trước nhanh (actions, create)
    ├── info_panel/               <- Thông tin file/folder
    ├── status_bar/               <- Thanh trạng thái đáy
    ├── tab_bar/                  <- Tab phiên làm việc
    └── settings_dialog/          <- context_menu, general, keybinds
```

---

## 6. Nhóm thư viện dùng chung (`libs/`)

### 6.1. `babydra-common` — Logic lõi

```
libs/babydra-common/src/
├── lib.rs                        <- Re-export phẳng toàn bộ API tiện dụng
├── config/
│   ├── mod.rs
│   └── settings.rs               <- BabyDraConfig, ThemeConfig, ShellConfig, PowerConfig,
│                                    WallpaperConfig, NotificationConfig, ExploreSettings
│                                    (file cấu hình ~/.babydra/babydra.conf)
├── i18n/
│   ├── mod.rs                    <- Hàm t("namespace.key")
│   └── locales/<app>/{en,vi}.json
├── models/
│   ├── mod.rs
│   ├── explore/                  <- directory, file_entry, grouping, session, tab, widgets
│   ├── settings/                 <- app_info, certificates, display, env_var, hosts, keybind,
│   │                                startup_command, system_info, system_update, vpn, wifi
│   ├── shell/                    <- battery, dbusmenu, island_state, network, notification,
│   │                                power, shell_config, storage, theme_config, tray_item, volume
│   ├── network.rs
│   └── screenshot/
└── services/                     <- Xem chi tiết Chương 02, mục 7
```

### 6.2. `babydra-utils` — UI kit (giao diện dùng chung)

```
libs/babydra-utils/src/
├── lib.rs                        <- pub mod components; pub mod ui;
├── components/                   <- badge, buttons (icon/standard/tile), card (standard/
│                                    scrollable/switch_card), list_group, modal
│                                    (password/vpn_config/vpn_log/wifi_config/wifi_info/wifi_password
│                                    dialog), placeholder, popovers, slider, switch, tooltips, wifi
│                                    (close_button, navbar, progress, spinners: deprecated, feature
│                                    `deprecated-components`)
├── ui/
│   ├── theme/mod.rs              <- init_theme(): gộp CSS shared + dark/light, nạp provider toàn cục
│   ├── theme/colors.rs           <- Color tokens dùng chung cho Cairo + CSS (T1.4)
│   ├── icon/                     <- resolver, assets
│   ├── animation/                <- easing, genie, island, slide
│   ├── battery.rs, window.rs
└── styles/                       <- CSS: shared/ + dark/ + light/
```

### 6.3. `babydra-explore-kit` — Explore feature kit

Tách từ `babydra-utils` (T3.1) — dialogs, context menus, drag & drop, file item builders:

```
libs/babydra-explore-kit/src/
├── lib.rs                        <- pub mod explore; pub use explore::*;
└── explore/
    ├── context_menu/             <- clipboard, custom_items, dimming, file_actions, widgets
    ├── dialogs/                  <- alert, archive, confirm, conflict, decompress, new_file,
    │                                new_folder, properties, rename
    ├── drag/                     <- source, target
    ├── helpers/                  <- archive, format, path, trash
    ├── items/                    <- grid_card, list_row
    ├── selection/                <- grid, listbox
    └── widgets/                  <- button
```

### 6.4. `babydra-theme` — Theme engine

Crate thuần logic (không GTK): đọc theme package từ `themes/<id>/`:

```
libs/babydra-theme/src/
├── lib.rs                        <- load_package, resolve_theme (hỗ trợ kế thừa base)
└── tokens.rs                     <- ThemeTokens, DarkLightTokens, RadiusTokens (serde, merge)
```

### 6.5. `babydra-island` — Dynamic Island


```
libs/babydra-island/src/
├── lib.rs                        <- create_system_island
├── models/                       <- Trạng thái island (widgets.rs)
├── player/                       <- playerctl, player_loop (media player overlay)
├── widgets/                      <- popover, visualizer
└── render.rs
```

### 6.6. `babydra-launcher` — Launcher ứng dụng

```
libs/babydra-launcher/src/
├── lib.rs                        <- build_launcher_ui
├── main.rs                       <- GTK app org.babydra.launcher
├── render.rs
└── widgets/
    ├── app_row/                  <- Dòng ứng dụng trong kết quả
    ├── file_search/              <- Tìm kiếm file
    ├── search/                   <- Fuzzy search ứng dụng
    └── footer/                   <- Gợi ý phím tắt
```

---

## 7. Cấu hình hệ thống, Themes, Variants và Script (`configs/`, `themes/`, `variants/`, `scripts/`)

### 7.1. `configs/`

```
configs/
├── labwc/
│   ├── autostart                 <- Khởi chạy switcher daemon, fcitx5, panel, settings, scripts
│   ├── rc.xml                    <- Window rules, phím tắt (A-Tab, W-q, W-l, W-F12, Print...), theme
│   ├── fonts.conf                <- Fontconfig (Segoe UI, Cascadia Code)
│   ├── settings.ini              <- GTK settings (nhân bản sang gtk-3.0/4.0)
│   ├── themerc-override          <- Override titlebar theme (corner radius, màu viền)
│   ├── themes/{dark,light}       <- Theme titlebar dark/light
│   └── scripts/
│       ├── bat_saver.sh          <- Tự động bật battery saver khi pin thấp
│       └── switcher.sh           <- Hỗ trợ Alt-Tab
├── kitty/                        <- Cấu hình terminal
├── nvim/                         <- init.lua + lazy + custom (keymap_viewer, sidebar)
├── fastfetch/                    <- config.jsonc + logo
└── themes/
    ├── BabyDra/                  <- GTK theme (labwc window buttons SVG)
    ├── cursor/                   <- Twilight cursors (.tar)
    └── icons/                    <- We10X icons (.tar)
```

### 7.2. `themes/` — Theme packages

Điểm mở rộng chính (T3.3): mỗi giao diện = 1 theme package, người tạo theme mới
**không cần sửa 1 dòng code**:

```
themes/
└── <theme-id>/
    ├── tokens.json                <- Design tokens (surface, border, accent, font, radius)
    ├── theme.css                  <- Lớp màu theme (nạp lên core CSS)
    └── fonts.json                 <- Font families + fallbacks
```

- `babydra-default/` — theme chính thức.
- `babydra-blue/` — theme mẫu thứ hai (kế thừa default, override accent + radius) — test sống.
- Engine: `babydra-theme` (`load_package`, `resolve_theme`, hỗ trợ `base` kế thừa).

### 7.3. `variants/` — Variants

Mỗi phiên bản hoàn chỉnh = 1 thư mục riêng (T3.4):

```
variants/<name>/
└── variant.toml                   <- theme ref, app list, keybinds, config overrides
```

Merge thứ tự (phải sang trái thắng):
`system defaults < configs/ seed < theme package < variant < ~/.babydra/ (user)`

- API: `babydra_common::config::variant::{list_variants, load_variant, get_keybind}`.
- `variants/default/` — variant chính thức.

### 7.4. `tests/` — Test suite (TDD safety net)

Integration test chia theo vùng (xem `tests/README.md`); unit test `#[cfg(test)]`
trong từng crate. Chạy: `cargo test --workspace` / `cargo test -p babydra-tests`.

### 7.5. `scripts/`

Các script đã gom về một nơi (T3.6):

| Script | Vai trò |
| :--- | :--- |
| `scripts/install.sh` | Cài đặt toàn bộ DE từ source (dependencies + build + deploy) |
| `scripts/start.sh` | Khởi động: config labwc + .desktop entries + chạy labwc |
| `scripts/update.sh` | Hot-update: rebuild + copy binary + sync config + restart panel |
| `scripts/check.sh` | Safety net: cargo check + fmt + clippy + test |

### 7.6. `start.sh` — Khởi động DE

Công việc chính:
1. Dừng các tiến trình cũ (`killall babydra-panel babydra-launcher babydra-preview ...`).
2. Sao chép `wallpaper.png` vào `~/.babydra/`.
3. Đồng bộ `configs/labwc/*` vào `~/.config/labwc/` (autostart, rc.xml, themerc-override, themes, scripts).
4. Đăng ký `.desktop` entries cho Preview & Settings, bind MIME cho hình ảnh.
5. Kiểm tra `ddcutil` và ddcutil-service.
6. `exec labwc` — khởi chạy compositor.

### 7.7. `update.sh` — Hot-update & Reload

1. `cargo build --release` — build lại toàn bộ workspace.
2. Dừng mọi tiến trình BabyDra.
3. Copy binary mới vào `~/.local/bin/` (+ `sudo cp babydra-greeter /usr/bin/`).
4. Stage logos/wallpapers vào `/var/lib/babydra/` và `/usr/share/babydra/`.
5. Đồng bộ toàn bộ `configs/` (labwc, gtk, fontconfig, kitty, nvim, fastfetch, themes/icons/cursors).
6. Áp dụng GSettings (font Segoe UI, icon We10X, cursor Twilight) + `fc-cache -f`.
7. `labwc --reconfigure` nếu đang chạy, khởi động lại `babydra-panel`.

### 7.8. `install.sh` (nhánh `release`) — Cài đặt toàn bộ từ source

1. Cài dependencies qua `sudo pacman -Syu` (gtk4, gtk4-layer-shell, labwc, pipewire, ddcutil, greetd, cage, bluetooth, networkmanager...).
2. Cấu hình `i2c-dev` và CPU performance permissions.
3. Cài `yay` (AUR helper) nếu chưa có, cài AUR packages (kitty, neovim, fastfetch...).
4. Build workspace và deploy binary + configs + themes.

> [!NOTE]
> Trên nhánh `main`, `install.sh` khác: chỉ build binary `babydra-installer` (nếu chưa có) rồi `exec` bộ cài đặt TUI với các tham số truyền vào.

---

## 8. Tài liệu (`docs/`)

```
docs/
├── README.md                     <- Mục lục tài liệu
├── 01-overview.md                <- Tổng quan dự án
├── 02-architecture.md            <- Kiến trúc mã nguồn
├── 03-project-structure.md       <- Cấu trúc dự án (chính là file này)
├── 04-setup-and-build.md         <- Cài đặt & build
└── design/                       <- Tài liệu thiết kế giao diện
    ├── README.md
    ├── visual-language.md        <- Ngôn ngữ thị giác Glassmorphism
    ├── surfaces.md               <- Bề mặt UI
    ├── color.md                  <- Màu sắc
    ├── typography.md             <- Typography
    ├── states.md                 <- Trạng thái tương tác
    ├── motion.md                 <- Chuyển động
    ├── spacing.md                <- Khoảng cách
    ├── theming.md                <- Dark/Light theming
    ├── tokens.md                 <- Bảng design tokens
    └── components/               <- 16 tài liệu component (buttons, badge, card, switch,
                                     slider, modal, popovers, navbar, list_group, placeholder,
                                     progress, spinners, tooltips, close_button, alerts, wifi)
```

---

## 9. Quy tắc chung khi viết mã nguồn mới

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Thư mục crate dùng kebab-case; file/thư mục trong `src/` dùng snake_case |
| DO | Tách `mod.rs` (logic) / `render.rs` (UI) / `handlers.rs` (sự kiện) cho mọi widget phức tạp |
| DO | Mọi logic hệ thống phải nằm trong `babydra-common/src/services/` và gọi qua API |
| DO | Mọi CSS phải nằm trong `libs/babydra-utils/src/styles/` và nạp qua `init_theme()` |
| DO | Chuỗi hiển thị phải đi qua `i18n::t()` với file JSON en/vi tương ứng |
| DO | State dùng chung qua `Rc<RefCell<T>>` — không lưu business data trong widget |
| DO NOT | Không import `gtk4` trong `babydra-common` |
| DO NOT | Không hardcode chuỗi UI tiếng Việt/Anh trong widget |
| DO NOT | Không tự tạo `GtkCssProvider` riêng trong ứng dụng |
| DO NOT | Không viết CSS inline trong mã Rust |
