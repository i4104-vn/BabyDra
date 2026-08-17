# Cấu trúc dự án & quy chuẩn viết mã

**Phạm vi:** Quy chuẩn đặt tên thư mục, triết lý phân tách file, trách nhiệm từng module, quy tắc viết mã mới.
**Phiên bản:** 1.5.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Tổng quan cấu trúc thư mục](#1-tổng-quan-cấu-trúc-thư-mục)
- [2. Quy chuẩn đặt tên thư mục](#2-quy-chuẩn-đặt-tên-thư-mục)
- [3. Triết lý phân tách file mã nguồn](#3-triết-lý-phân-tách-file-mã-nguồn)
- [4. Bộ cài đặt TUI (`install/`)](#4-bộ-cài-đặt-tui-install)
- [5. Nhóm ứng dụng đồ họa (`crates/`)](#5-nhóm-ứng-dụng-đồ-họa-crates)
- [6. Nhóm thư viện dùng chung (`libs/`)](#6-nhóm-thư-viện-dùng-chung-libs)
- [7. Cấu hình hệ thống, Themes, Variants và Script](#7-cấu-hình-hệ-thống-themes-variants-và-script)
- [8. Tài liệu (`docs/`)](#8-tài-liệu-docs)
- [9. Quy tắc chung khi viết mã nguồn mới](#9-quy-tắc-chung-khi-viết-mã-nguồn-mới)

---

## 1. Tổng quan cấu trúc thư mục

> [!IMPORTANT]
> Kho mã nguồn phân tách theo mô hình 3 nhánh (xem `WORKFLOW.md` ở gốc repository):
> - **Nhánh `main`** — chỉ chứa `install/`, `install.sh`, tài liệu.
> - **Nhánh `release`/`develop`** — chứa thêm `crates/`, `libs/`, `configs/`, `themes/`, `variants/`, `scripts/`, `tests/`, `start.sh`, `update.sh`.

### 1.1. Nhánh `main` (Kênh phân phối)

```text
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

```text
BabyDra/                          <- Thư mục gốc repository (nhánh release)
├── Cargo.toml                    <- Workspace manifest — liệt kê toàn bộ crates/libs/install/tests
├── Cargo.lock
├── scripts/                      <- Scripts: install.sh, start.sh, update.sh, check.sh
├── wallpaper.png                 <- Hình nền mặc định của hệ thống
├── README.md
├── WORKFLOW.md                   <- Mô hình branch + ma trận sở hữu
├── CONTRIBUTING.md               <- Checklist PR
├── CHANGELOG.md                  <- Lịch sử phiên bản (SemVer)
├── docs/                         <- Tài liệu (đồng bộ với nhánh main)
├── configs/                      <- Cấu hình mẫu hệ thống (seed)
├── crates/                       <- Các ứng dụng đồ họa thực thi độc lập
├── libs/                         <- Các thư viện dùng chung (không thể chạy độc lập)
├── themes/                       <- Theme packages (tokens.json + fonts.json + css/)
├── variants/                     <- Variants (mỗi variant 1 thư mục riêng)
├── tests/                        <- Integration test suite (TDD safety net)
└── install/                      <- Bộ cài đặt TUI (dùng chung với nhánh main)
```

**Workspace `release` (`Cargo.toml`):**

```toml
[workspace]
resolver = "2"
members = [
    "libs/babydra-core",      "libs/babydra-island",
    "crates/babydra-launcher",  "libs/babydra-theme",
    "libs/babydra-ui-kit",
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

Tất cả crate ở tầng gốc (`libs/`, `crates/`) dùng **kebab-case**:

| Đúng | Sai | Lý do |
| :--- | :--- | :--- |
| `babydra-panel` | `babydraPanel` | camelCase không được phép |
| `babydra-core` | `babydra_core` | snake_case chỉ dùng bên trong `src/` |
| `babydra-screenshot` | `BabyDraScreenshot` | PascalCase không được phép |

### 2.2. Tầng bên trong `src/`: snake_case

Mọi thư mục con và file trong `src/` dùng **snake_case**:

| Đúng | Sai | Lý do |
| :--- | :--- | :--- |
| `src/widgets/` | `src/Widgets/` | PascalCase không được phép |
| `src/system_tray/` | `src/systemTray/` | camelCase không được phép |
| `src/control_center/` | `src/control-center/` | kebab-case chỉ dùng ở tầng crate |
| `src/main.rs` | `src/Main.rs` | Chỉ `main.rs`/`lib.rs` được đặc cách |

### 2.3. File CSS: snake_case

- CSS cấu trúc (shared) trong `libs/babydra-ui-kit/src/styles/shared/` — nhóm theo đối tượng: `panel/panel.css`, `explore/content_view.css`, `apps/settings.css`, `shared/button.css`…
- Lớp màu dark/light thuộc theme packages (`themes/<id>/css/dark.css`, `css/light.css`) — không nằm trong code.

---

## 3. Triết lý phân tách file mã nguồn

Mỗi module đồ họa trong `crates/` và `libs/` tuân thủ cấu trúc phân tách logic và giao diện:

| File | Trách nhiệm |
| :--- | :--- |
| `mod.rs` | Khai báo module, định nghĩa kiểu dữ liệu, State struct, cấu trúc điều khiển |
| `render.rs` | Xây dựng cây widget GTK4 (thuần khai báo UI) |
| `handlers.rs` | Xử lý sự kiện tương tác từ người dùng (click, phím, thay đổi trạng thái) |
| `actions.rs` | Các hành động nghiệp vụ cụ thể của widget (copy, delete, create…) |

> [!IMPORTANT]
> **Quy tắc chống trộn lẫn:** `render.rs` **không được** chứa `std::process::Command`, `std::fs::read` hay logic nghiệp vụ — mọi thao tác hệ thống phải gọi qua `babydra-core`.

> [!NOTE]
> Feature folder của **Dynamic Island** có quy ước riêng (`mod.rs` / `view.rs` / `render.rs` / `service.rs` + helper) — xem [guides/island-features](../guides/island-features.md).

---

## 4. Bộ cài đặt TUI (`install/`)

```text
install/
├── Cargo.toml                    <- Dependencies: ratatui 0.29, crossterm 0.28, chrono, dirs, anyhow, libc
├── README.md                     <- Hướng dẫn sử dụng nhanh
├── run.sh                        <- Script thực thi nhanh (chỉ trên nhánh release)
└── src/
    ├── lib.rs                    <- Library target: re-export models/system/tasks cho integration tests
    ├── main.rs                   <- Entry point: parse CLI args, raw mode, event loop 50ms tick
    ├── app/
    │   ├── mod.rs                <- Struct App: trạng thái wizard, channel, profiles, logs
    │   └── handlers.rs           <- Xử lý phím bấm theo từng step/modal
    ├── models/
    │   ├── mod.rs
    │   ├── step.rs               <- enum WizardStep (8 bước) + ALL/next/prev
    │   ├── options.rs            <- InstallChannel, PresetProfile, BranchMetadata, GenericOptionItem
    │   ├── binary.rs             <- BinaryItem, BinaryLocation (UserLocalBin/SystemBin)
    │   └── log.rs                <- LogLevel, LogMessage
    ├── system/
    │   ├── mod.rs                <- Helper: find_workspace_root, safe_copy_binary, stop_process...
    │   ├── initializers.rs       <- Danh sách binaries, package options, configs/themes options
    │   ├── git_ops.rs            <- fetch_branch_metadata (git log -1)
    │   ├── fs_ops.rs             <- copy, chmod, tar extraction
    │   └── process.rs            <- killall, spawn worker
    ├── tasks/
    │   ├── mod.rs                <- InstallPlan, InstallEvent, spawn_installation_worker
    │   ├── packages.rs           <- pacman deps / AUR (yay) / kernel permissions
    │   ├── binaries.rs           <- Copy binary vào ~/.local/bin hoặc /usr/bin
    │   ├── varlib.rs             <- Stage vào /var/lib/babydra
    │   ├── configs.rs            <- Sync labwc, .desktop entries, themes, gsettings
    │   └── display_manager.rs    <- Cấu hình greetd, mask gettys
    └── ui/
        ├── mod.rs
        ├── layout.rs             <- Header, sidebar, footer (key hints)
        ├── modals/               <- help, confirm, edit_path
        └── steps/                <- welcome, packages, binaries, varlib, configs,
                                     display_manager, progress, summary
```

### 4.1. 8 bước Wizard

| Bước | Tên | Mô tả |
| :--- | :--- | :--- |
| 1 | Welcome & Overview | Chọn kênh (Release/Develop/LocalSource), preset profile |
| 2 | System Packages & Deps | Pacman deps, AUR (yay), kernel/i2c permissions |
| 3 | BabyDra Binaries | Chọn binary cài vào `~/.local/bin` / `/usr/bin` |
| 4 | /var/lib Staging Bundle | Stage binaries, wallpapers, logos |
| 5 | Configs, Themes & Icons | labwc configs, .desktop entries, dotfiles, themes |
| 6 | Greetd Display Manager | Cấu hình greetd + mask VTs + enable service |
| 7 | Execute Installation | Chạy worker, theo dõi log realtime |
| 8 | Summary & Launch | Kết quả và thoát |

### 4.2. 3 kênh cài đặt

- **Release Channel** — git fetch nhánh `release` + build nếu binary chưa có.
- **Develop Channel** — tương tự với nhánh `develop`.
- **Local Source** — copy trực tiếp binary có sẵn (mặc định `target/release/`).

### 4.3. 3 Preset Profile

- **Full Desktop (Recommended)** — tất cả: binaries + varlib + configs/themes + display manager.
- **Binaries & /var/lib Staging Only** — chỉ binary + staging.
- **Custom Selection** — người dùng tự chọn từng mục.

### 4.4. Phím tắt chính

| Phím | Hành động |
| :--- | :--- |
| `1`–`8` | Nhảy thẳng tới bước tương ứng |
| `Tab` / `n`, `BackTab` / `p` | Bước kế tiếp / trước đó |
| `↑`/`↓`/`j`/`k`, `Space`, `a`/`A` | Di chuyển, toggle, chọn/bỏ chọn tất cả |
| `c` | Bước 1: đổi kênh — Bước 7: xóa log buffer |
| `s` | Đổi thư mục nguồn binary |
| `r` | Quét lại thư mục nguồn |
| `i` / `Enter` | Bắt đầu cài đặt (dialog xác nhận) |
| `?` | Mở help modal |
| `q` / `Ctrl+C` | Thoát |

---

## 5. Nhóm ứng dụng đồ họa (`crates/`)

> [!NOTE]
> Chỉ tồn tại trên nhánh `release`/`develop`.

### 5.1. `babydra-panel` — Thanh taskbar chính

```text
crates/babydra-panel/src/
├── main.rs                       <- Tray watcher, DDC detection, switcher tracker, GTK app
├── render.rs                     <- build_panel_ui: panel + control center + calendar + launcher windows
└── widgets/
    ├── mod.rs
    ├── panel/                    <- mod.rs, render.rs, modal.rs, toggle_grid.rs
    │   ├── items/                <- header, wifi, bluetooth, volume, vpn, backlight, storage, clean
    │   └── popover/              <- network, volume, vpn, battery
    ├── clock/                    <- Đồng hồ + calendar_window + notifications
    ├── sys_monitor/              <- CPU/RAM monitor
    ├── tray/                     <- Khay hệ thống (StatusNotifier)
    └── workspace/                <- Workspace + preview
```

### 5.2. `babydra-switcher` — Alt-Tab Switcher

```text
crates/babydra-switcher/src/
├── main.rs                       <- --daemon (giữ overlay) / one-shot client
├── daemon.rs                     <- Lắng nghe Unix socket /tmp/babydra-switcher.socket
├── render.rs
└── widgets/                      <- Overlay danh sách cửa sổ + preview
```

### 5.3. `babydra-screenshot` — Chụp màn hình

```text
crates/babydra-screenshot/src/
├── main.rs                       <- --full: toàn màn hình; mặc định: vùng (slurp/grim)
└── widgets/
    ├── editor.rs                 <- Editor sau khi chụp
    ├── canvas.rs                 <- Canvas vẽ/vùng chọn
    └── color_popover.rs          <- Chọn màu
```

### 5.4. `babydra-lock` — Màn hình khóa

```text
crates/babydra-lock/src/
├── main.rs                       <- Parse --image, GTK app, map window tới mọi màn hình
├── render.rs                     <- build_lock_ui
└── widgets/                      <- Locker UI (xác thực PAM qua babydra_core::verify_password)
```

### 5.5. `babydra-greeter` — Màn hình đăng nhập (greetd)

```text
crates/babydra-greeter/src/
├── main.rs                       <- init_logger, đọc GREETD_SOCK/WAYLAND_DISPLAY, GTK app
├── auth.rs                       <- Xác thực qua greetd protocol
├── handlers.rs                   <- setup_handlers
├── render.rs                     <- build_greeter_ui
├── theme.rs
└── widgets/                      <- login, splash, top_bar
```

### 5.6. `babydra-settings` — Trung tâm cấu hình

```text
crates/babydra-settings/src/
├── main.rs                       <- CLI helpers: --apply-battery-saver, --set-power-profile...
├── layout.rs
└── widgets/
    ├── appearance/               <- theme, wallpaper, avatar
    ├── apps/                     <- launch, update, uninstall
    ├── bluetooth/
    ├── certificates/
    ├── displays/
    ├── env/
    ├── hosts/                    <- File /etc/hosts
    ├── keybinds/
    ├── power/                    <- Power profile + battery_card
    ├── startup/
    ├── system_info/
    └── system_update/            <- pacman update với log realtime
```

### 5.7. `babydra-preview` — Xem nhanh hình ảnh

```text
crates/babydra-preview/src/
├── main.rs                       <- Đường dẫn ảnh từ argv, fallback FileDialog
├── exif_reader.rs                <- Đọc metadata EXIF
└── widgets/
    └── viewer.rs                 <- Viewer zoom/pan
```

### 5.8. `babydra-explore` — Trình quản lý tập tin

```text
crates/babydra-explore/src/
├── main.rs                       <- tokio runtime + SessionState + create_explore_window
└── widgets/
    ├── window/                   <- layout (split/preview), handlers, widgets (tabs)
    ├── content_view/             <- rendering (grid/list), gestures (background, clipboard, flowbox,
    │                                listbox), items (grid_item)
    ├── header_bar/               <- Thanh địa chỉ + điều hướng
    ├── sidebar/                  <- Cây thư mục + bookmarks
    ├── preview_panel/            <- Xem trước nhanh
    ├── info_panel/               <- Thông tin file/folder
    ├── status_bar/               <- Thanh trạng thái đáy
    ├── tab_bar/                  <- Tab phiên làm việc
    └── settings_dialog/          <- context_menu, general, keybinds
```

---

## 6. Nhóm thư viện dùng chung (`libs/`)

### 6.1. `babydra-core` — Logic lõi

> Crate **duy nhất** chứa logic thuần + services; mọi app gọi API qua đây.

```text
libs/babydra-core/src/
├── lib.rs                        <- Re-export phẳng toàn bộ API tiện dụng
├── config/
│   ├── mod.rs
│   └── settings.rs               <- BabyDraConfig, ThemeConfig, ShellConfig, PowerConfig,
│                                    WallpaperConfig, NotificationConfig, ExploreSettings
│                                    (file cấu hình ~/.babydra/babydra.conf, gồm [theme] selection)
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
└── services/                     <- Xem chi tiết [architecture](../architecture/index.md) mục 7
```

### 6.2. `babydra-ui-kit` — UI kit (giao diện dùng chung)

```text
libs/babydra-ui-kit/src/
├── lib.rs                        <- pub mod components; pub mod explore; pub mod ui;
├── components/                   <- badge, buttons (icon/standard/tile), card, list_group, modal
│                                    (password/vpn/wifi...), placeholder, popovers, slider, switch,
│                                    tooltips, wifi, navbar, progress
├── components/explore/           <- Feature components cho Explore:
│   ├── context_menu/             <-   clipboard, custom_items, dimming, file_actions, widgets
│   ├── dialogs/                  <-   alert, archive, confirm, conflict, decompress, new_file,
│   │                                new_folder, properties, rename
│   ├── drag/                     <-   source, target
│   ├── helpers/                  <-   archive, format, path, trash
│   ├── items/                    <-   grid_card, list_row
│   ├── selection/                <-   grid, listbox
│   ├── widgets/                  <-   button
│   └── prelude.rs                <-   re-export API feature (tránh trùng tên với prelude gốc)
├── ui/
│   ├── theme/mod.rs              <- init_theme(): đọc ThemeSelection, resolve theme package
│   │                                qua babydra-theme, nạp CSS shared + lớp màu dark/light
│   ├── theme/colors.rs           <- Color tokens dùng chung cho Cairo + CSS
│   ├── icon/                     <- resolver, assets
│   ├── animation/                <- easing, genie, island, slide
│   ├── battery.rs, window.rs
└── styles/shared/                <- CSS cấu trúc (shared) — lớp màu thuộc themes/
```

### 6.3. `babydra-theme` — Theme engine

Crate thuần logic (không GTK): đọc theme package từ `themes/<id>/` và resolve lớp CSS (dark/light) + tokens + fonts:

```text
libs/babydra-theme/src/
├── lib.rs                        <- load_package, resolve_theme (kế thừa base), themes_root()
│                                    (BABYDRA_THEMES_DIR → ~/.babydra/themes →
│                                    /usr/share/babydra/themes → workspace themes/)
└── tokens.rs                     <- ThemeTokens, DarkLightTokens, RadiusTokens (serde, merge)
```

`babydra-ui-kit` gọi `resolve_theme(id)` trong `init_theme()` — đổi theme = đổi 1 dòng trong `babydra.conf`: `[theme] selection = { id = "babydra-blue" }`.

### 6.4. `babydra-island` — Dynamic Island (mở rộng được)

Island **mở rộng được**: đăng ký view/feature mới qua trait `IslandFeature` (feature phức tạp) hoặc descriptor `IslandView` + handle `IslandViewHandle` (overlay đơn giản). Controller loop arbitrate theo priority, hỗ trợ ghi đè tạm thời (`override_show_for`) để chiếm chỗ media player rồi tự trả lại view trước đó.

```text
libs/babydra-island/src/
├── lib.rs                        <- Re-export API public + default_island()
├── render.rs                     <- create_system_island, build_default_island
├── island/                       <- Island manager + controller loop:
│   ├── mod.rs                    <-   arbitration, override, transitions, register API
│   └── view.rs                   <-   IslandView, IslandViewHandle, IslandFeature trait, IslandCtx
├── features/                     <- Mỗi feature = 1 thư mục riêng (cấu trúc chuẩn):
│   ├── mod.rs                    <-   khai báo các feature
│   ├── default/                  <-   idle logo (mod.rs duy nhất)
│   ├── media_player/             <-   mod.rs, view.rs, render.rs, poll.rs, art.rs,
│   │                                popover.rs, visualizer.rs, format.rs
│   └── notification/             <-   mod.rs, view.rs, render.rs, service.rs
├── models/                       <- ActiveNotification, NotificationMsg (re-export)
└── widgets/                      <- Re-export notification API (compat)
```

> [!IMPORTANT]
> Quy ước cấu trúc feature: mỗi feature là **1 thư mục** trong `features/` với
> `mod.rs` (vòng đời + IslandFeature) + `view.rs` (dựng widget) + `render.rs`
> (đẩy dữ liệu) + `service.rs` (service nền, tùy chọn) — xem
> [guides/island-features](../guides/island-features.md).

### 6.5. `babydra-launcher` — Launcher ứng dụng

```text
crates/babydra-launcher/src/
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

## 7. Cấu hình hệ thống, Themes, Variants và Script

### 7.1. `configs/`

```text
configs/
├── labwc/
│   ├── autostart                 <- Switcher daemon, fcitx5, panel, settings, scripts
│   ├── rc.xml                    <- Window rules, phím tắt, theme
│   ├── fonts.conf                <- Fontconfig (Segoe UI, Cascadia Code)
│   ├── settings.ini              <- GTK settings (nhân bản sang gtk-3.0/4.0)
│   ├── themerc-override          <- Override titlebar theme
│   ├── themes/{dark,light}       <- Theme titlebar dark/light
│   └── scripts/
│       ├── bat_saver.sh          <- Tự bật battery saver khi pin thấp
│       └── switcher.sh           <- Hỗ trợ Alt-Tab
├── kitty/                        <- Cấu hình terminal
├── nvim/                         <- init.lua + lazy + custom
├── fastfetch/                    <- config.jsonc + logo
└── themes/
    ├── BabyDra/                  <- GTK theme (window buttons SVG)
    ├── cursor/                   <- Twilight cursors (.tar)
    └── icons/                    <- We10X icons (.tar)
```

### 7.2. `themes/` — Theme packages

Điểm mở rộng chính: mỗi giao diện = 1 theme package, người tạo theme mới **không cần sửa 1 dòng code**. Chi tiết: [themes](../themes/index.md).

```text
themes/
└── <theme-id>/
    ├── tokens.json                <- Design tokens (surface, border, accent, font, radius)
    ├── fonts.json                 <- Font families + fallbacks
    └── css/                       <- CSS tách riêng, không nằm chung với JSON
        ├── dark.css               <- Lớp màu dark-mode
        ├── light.css              <- Lớp màu light-mode
        └── theme.css              <- (tùy chọn) override nạp cuối
```

Các theme đi kèm: `babydra-default` (base), `babydra-blue`, `babydra-purple`, `babydra-green`, `babydra-rose` — cùng cấu trúc, chỉ khác accent.

### 7.3. `variants/` — Variants

```text
variants/<name>/
└── variant.toml                   <- theme ref, app list, keybinds, config overrides
```

Merge thứ tự (phải sang trái thắng):
`system defaults < configs/ seed < theme package < variant < ~/.babydra/ (user)`

- API: `babydra_core::config::variant::{list_variants, load_variant, get_keybind}`.
- Variants đi kèm: `default`, `blue`, `purple`, `green`, `rose`.

### 7.4. `tests/` — Test suite (TDD safety net)

Toàn bộ test tách về đây — không còn `#[cfg(test)]` trong workspace crates. Integration test chia theo vùng (`common/`, `models/`, `services/`, `theme/`, `installer/`, `kits/`), mỗi file là một test binary khai báo trong `tests/Cargo.toml`. Chạy: `cargo test --workspace` / `cargo test -p babydra-tests`.

### 7.5. `scripts/`

| Script | Vai trò |
| :--- | :--- |
| `scripts/install.sh` | Cài đặt toàn bộ DE từ source (dependencies + build + deploy) |
| `scripts/start.sh` | Khởi động: config labwc + .desktop entries + chạy labwc |
| `scripts/update.sh` | Hot-update: rebuild + copy binary + sync config + restart panel |
| `scripts/check.sh` | Safety net: cargo check + fmt + clippy + test |

### 7.6. `start.sh` — Khởi động DE

1. Dừng tiến trình cũ (`killall babydra-panel babydra-launcher ...`).
2. Sao chép `wallpaper.png` vào `~/.babydra/`.
3. Đồng bộ `configs/labwc/*` vào `~/.config/labwc/`.
4. Đăng ký `.desktop` entries cho Preview & Settings, bind MIME.
5. Kiểm tra `ddcutil` và ddcutil-service.
6. `exec labwc`.

### 7.7. `update.sh` — Hot-update & Reload

1. `cargo build --release`.
2. Dừng mọi tiến trình BabyDra.
3. Copy binary mới vào `~/.local/bin/` (+ `sudo cp babydra-greeter /usr/bin/`).
4. Stage logos/wallpapers vào `/var/lib/babydra/` và `/usr/share/babydra/`.
5. Đồng bộ toàn bộ `configs/`.
6. Áp dụng GSettings + `fc-cache -f`.
7. `labwc --reconfigure` nếu đang chạy, restart `babydra-panel`.

### 7.8. `install.sh` (nhánh `release`) — Cài đặt toàn bộ từ source

1. Cài dependencies qua `sudo pacman -Syu` (gtk4, gtk4-layer-shell, labwc, pipewire, ddcutil, greetd, cage...).
2. Cấu hình `i2c-dev` và CPU performance permissions.
3. Cài `yay` + AUR packages (kitty, neovim, fastfetch...).
4. Build workspace và deploy binary + configs + themes.

> [!NOTE]
> Trên nhánh `main`, `install.sh` khác: chỉ build binary `babydra-installer` (nếu chưa có) rồi `exec` bộ cài đặt TUI.

---

## 8. Tài liệu (`docs/`)

```text
docs/
├── README.md                     <- Mục lục tài liệu (trang đầu duy nhất có header trình bày)
├── overview/                     <- Tổng quan dự án
│   └── index.md
├── architecture/                 <- Kiến trúc mã nguồn
│   └── index.md
├── structure/                    <- Cấu trúc dự án (chính là file này)
│   └── index.md
├── setup/                        <- Cài đặt & build
│   └── index.md
├── themes/                       <- Tạo theme/variant mới
│   └── index.md
├── apis/                         <- API reference từng thư viện
│   ├── index.md                  <-   Tổng hợp UI kit & prelude
│   ├── core.md                   <-   API babydra-core
│   ├── ui-kit.md                 <-   API babydra-ui-kit
│   └── explore-kit.md            <-   API components::explore
├── flows/                        <- Luồng hoạt động hiện tại từng crate/lib
│   ├── index.md                  <-   Tổng quan hệ thống + bản đồ luồng
│   ├── core.md, ui-kit.md, theme.md, island.md
│   ├── panel.md, switcher.md, screenshot.md, lock.md, greeter.md
│   └── settings.md, preview.md, explore.md, launcher.md, installer.md
├── guides/                       <- Hướng dẫn sử dụng code tái sử dụng
│   ├── island.md                 <-   Sử dụng & mở rộng Dynamic Island
│   ├── island-features.md        <-   Cấu trúc chuẩn của island feature
│   └── island-internals.md       <-   Kiến trúc runtime & luồng chi tiết island
└── design/                       <- Tài liệu thiết kế giao diện
    ├── README.md
    ├── visual-language.md, surfaces.md, color.md, typography.md,
    ├── states.md, motion.md, spacing.md, theming.md, tokens.md
    └── components/               <- Tài liệu từng component
```

---

## 9. Quy tắc chung khi viết mã nguồn mới

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Thư mục crate dùng kebab-case; file/thư mục trong `src/` dùng snake_case |
| DO | Tách `mod.rs` (logic) / `render.rs` (UI) / `handlers.rs` (sự kiện) cho widget phức tạp |
| DO | Mọi logic hệ thống nằm trong `babydra-core/src/services/` và gọi qua API |
| DO | CSS cấu trúc trong `styles/shared/`; màu dark/light thuộc `themes/` |
| DO | Chuỗi hiển thị đi qua `i18n::t()` với file JSON en/vi tương ứng |
| DO | State dùng chung qua `Rc<RefCell<T>>` — không lưu business data trong widget |
| DO | Feature mới của island nằm trong `features/<feature>/` theo cấu trúc chuẩn |
| DO NOT | Không import `gtk4` trong `babydra-core` |
| DO NOT | Không hardcode chuỗi UI tiếng Việt/Anh trong widget |
| DO NOT | Không tự tạo `GtkCssProvider` riêng trong ứng dụng |
| DO NOT | Không viết CSS inline trong mã Rust |
