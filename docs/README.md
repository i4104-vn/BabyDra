# BabyDra Documentation

Hướng dẫn phát triển cho dự án BabyDra Desktop Environment.

## 📚 Tài liệu

| File | Mô tả |
|------|-------|
| [`architecture.md`](architecture.md) | Kiến trúc tổng thể, code organization, dependency layers, refactoring guidelines |
| [`island-guide.md`](island-guide.md) | Hướng dẫn chi tiết Dynamic Island: descriptor views, IslandFeature trait, extending |
| [`core-services-uikit.md`](core-services-uikit.md) | Cách sử dụng babydra-core (config, models, services), babydra-ui-kit (components, animations), và island |

## 🚀 Quick Start

### Cấu trúc dự án

```
BabyDra/
├── libs/                    # Shared libraries
│   ├── babydra-core/        # Domain logic, system services, config
│   ├── babydra-island/      # Dynamic Island system
│   ├── babydra-ui-kit/      # Reusable GTK4 components
│   └── babydra-theme/       # CSS, colors
├── crates/                  # Applications
│   ├── babydra-panel/       # Top panel
│   ├── babydra-desktop/     # Desktop icons, wallpaper
│   ├── babydra-explore/     # File manager
│   ├── babydra-settings/    # Settings app
│   └── ...                  # Launcher, Lock, Greeter, Switcher, Screenshot
└── docs/                    # Documentation (this folder)
```

### Dependency Direction

```
Apps → Island → UI Kit → Core → Theme
```

Chỉ import **xuôi chiều**. Không bao giờ import ngược.

### Common Imports

```rust
// Core: config, services, models
use babydra_core::{load_babydra_config, apply_saved_settings, CoreResult};
use babydra_core::services::{wifi, bluetooth, wallpaper, battery, ...};
use babydra_core::models::{FileEntry, BatteryInfo, Workspace, ...};

// UI Kit: components, animations, theme, icons
use babydra_ui_kit::prelude::*;

// Island: dynamic island manager & features
use babydra_island::{create_system_island, Island, IslandView, IslandFeature, IslandViewHandle};
```

## 🔑 Key Concepts

### 1. Core / Services / UI Kit Pattern

| Layer | Purpose | Example Usage |
|-------|---------|---------------|
| **Core** | System integration, config, domain models | `wifi::scan_wifi()`, `load_babydra_config()`, `FileEntry` |
| **UI Kit** | Reusable GTK4 widgets, animations, theming | `create_button()`, `slide_in()`, `init_theme()`, `get_icon()` |
| **Island** | Dynamic notch capsule with arbitration | `create_system_island()`, `IslandFeature` trait, `override_show_for()` |

### 2. Feature-Based Module Structure

Mỗi feature (media_player, notification, clipboard, wifi settings, ...) tuân theo:

```
feature_name/
├── mod.rs          # Main struct + trait impl (IslandFeature, etc.)
├── service/        # Background logic, DBus, polling, async tasks
│   ├── mod.rs
│   ├── poll.rs
│   └── ...
└── ui/             # Widget building & rendering
    ├── mod.rs
    ├── view.rs     # build() - tạo widget tree
    ├── render.rs   # update(data) - bind data → widget
    └── ...
```

### 3. Island Arbitration

Island hiển thị **một view tại một thời điểm**, quyết định bởi:

1. **Override** (priority cao nhất) - `override_show_for(duration)`: volume, brightness
2. **Priority** - Feature priority cao hơn thắng
3. **Request sequence** - Yêu cầu mới hơn thắng
4. **Hover keep** - Giữ view khi hover capsule

## 📝 Development Workflow

### Thêm Feature Mới

1. **System Service** → `libs/babydra-core/src/services/new_service/`
2. **UI Component** → `libs/babydra-ui-kit/src/components/new_component/`
3. **Island Feature** → `libs/babydra-island/src/features/new_feature/`
4. **Register** → Update `island/src/render.rs` → `create_system_island()`

### Code Quality

```bash
# Format
cargo fmt --all

# Lint
cargo clippy --workspace

# Test
cargo test --workspace

# Build
cargo build --workspace
```

## 🎯 Best Practices Summary

1. **Dependency Direction** - Chỉ import xuôi chiều (Apps → Island → UI Kit → Core)
2. **Service Layer** - Tách business logic (service/) khỏi UI (ui/)
3. **Async Properly** - Dùng `glib::spawn_future_local` + channels, không block main thread
4. **Error Handling** - Dùng `CoreResult<T>`, `?` operator, handle gracefully trong UI
5. **State Management** - `Rc<RefCell<_>>` cho shared state, `Cell` cho simple state
6. **Cleanup** - `on_hide()`, `dispose()` cho island features; drop receivers khi không cần
7. **Public API** - Re-export ở `lib.rs`, doc comment cho mọi `pub` item
8. **Testing** - Unit test cho service logic, integration test cho DBus/system calls

## 🔗 Useful Links

- [GTK4 Rust Documentation](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/)
- [glib-rs Main Context](https://docs.gtk-rs.org/glib/latest/glib/main_context/)
- [tokio Runtime](https://docs.rs/tokio/latest/tokio/)
- [BabyDra Theme CSS](themes/)

---

*Generated for BabyDra project. Update khi kiến trúc thay đổi.*