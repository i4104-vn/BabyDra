# Kiến trúc dự án BabyDra

## Tổng quan

BabyDra là một môi trường desktop được xây dựng bằng Rust với GTK4, sử dụng kiến trúc workspace đa crate. Dự án được tổ chức theo các lớp (layers) rõ ràng:

```
BabyDra/
├── libs/                    # Shared libraries (internal)
│   ├── babydra-core/        # Core logic, config, services, models
│   ├── babydra-island/      # Dynamic Island system
│   ├── babydra-ui-kit/      # Reusable GTK4 components
│   └── babydra-theme/       # Theme & styling
├── crates/                  # Application crates (binaries)
│   ├── babydra-desktop/     # Desktop icons, wallpaper, grid
│   ├── babydra-explore/     # File manager
│   ├── babydra-panel/       # Top panel / status bar
│   ├── babydra-launcher/    # Application launcher
│   ├── babydra-lock/        # Lock screen
│   ├── babydra-greeter/     # Login greeter
│   ├── babydra-settings/    # Settings app
│   ├── babydra-switcher/    # Window switcher
│   ├── babydra-screenshot/  # Screenshot tool
│   ├── babydra-keymap/      # Keybindings
│   └── babydra-preview/     # Preview panel
└── scripts/                 # Build & utility scripts
```

## Nguyên tắc thiết kế

### 1. Separation of Concerns (Tách biệt mối quan tâm)

| Layer | Trách nhiệm | Depends on |
|-------|-------------|------------|
| **apps (crates/*)** | UI logic, wiring, app-specific behavior | core, ui-kit, island |
| **babydra-island** | Dynamic island manager + features | ui-kit, core |
| **babydra-ui-kit** | Pure UI components, animations, theming | core (minimal), gtk4 |
| **babydra-core** | Config, services, models, system integration | std, glib, gio |
| **babydra-theme** | CSS, color schemes, icons | - |

**Quy tắc:** Dependency chỉ đi **xuôi chiều** (top-down). Không được import ngược (ví dụ: core không được import ui-kit).

### 2. Core / Services / UI Kit Pattern

#### babydra-core (Domain Logic)
```
libs/babydra-core/src/
├── config/        # Configuration loading/saving
├── error.rs       # CoreError, CoreResult
├── i18n/          # Internationalization
├── models/        # Domain models (FileEntry, BatteryInfo, Workspace, etc.)
└── services/      # System services (clipboard, wifi, bluetooth, power, wallpaper, ...)
```

**Khi nào dùng core:**
- Cần truy cập system services (wifi, bluetooth, power, wallpaper)
- Cần domain models (FileEntry, BatteryInfo, Workspace)
- Cần config (load/save settings)
- Cần shared utilities (logger, search, tray, notification)

#### babydra-ui-kit (UI Components)
```
libs/babydra-ui-kit/src/
├── components/           # Reusable widget builders
│   ├── buttons/          # create_button, create_fab, create_icon_btn, ...
│   ├── cards/            # create_card, create_switch_card, create_scroll_list
│   ├── context_menu/     # ContextMenuBuilder, tray menus
│   ├── modals/           # PasswordDialog, WifiConfigDialog, VpnConfigDialog
│   ├── sliders/          # PillSlider, CustomSlider, DebouncedSlider
│   ├── explore/          # File manager specific components
│   │   ├── dialogs/      # Archive, Conflict, Properties, Rename dialogs
│   │   ├── context_menu/ # File context menus
│   │   ├── drag/         # Drag & drop source/target
│   │   └── items/        # GridCard, ListRow
│   └── ...
├── ui/                   # Low-level UI utilities
│   ├── animation/        # easing, genie, island, slide, topbar
│   ├── icon/             # Icon resolver, assets
│   ├── theme/            # Colors, apply_theme_class, init_theme
│   ├── battery/          # Battery drawing
│   ├── image/            # Masking, cropping helpers
│   └── window/           # Layer window setup
└── prelude               # One-stop import
```

**Khi nào dùng ui-kit:**
- Xây dựng UI components (buttons, cards, dialogs, menus)
- Cần animations (slide, genie, island animations)
- Cần icon handling (resolver, fallbacks, colored icons)
- Cần theme utilities (dark mode, CSS classes)
- Cần window/layer setup

#### babydra-island (Dynamic Island)
```
libs/babydra-island/src/
├── island/           # Island manager, builder, view handling
│   ├── mod.rs        # Island, IslandBuilder, IslandConfig
│   └── view.rs       # IslandView, IslandViewHandle, IslandFeature trait
├── features/         # Built-in features
│   ├── default/      # Idle logo pill
│   ├── media_player/ # MPRIS player + visualizer + popover
│   ├── notification/ # Desktop notifications
│   └── clipboard/    # Clipboard history
├── models.rs         # Shared models
├── render.rs         # System island creation
└── widgets.rs        # Shared widgets
```

**Khi nào dùng island:**
- Cần hiển thị system status trong notch capsule
- Muốn đăng ký feature mới (volume, brightness, timer, ...)
- Cần override tạm thời (volume overlay, clipboard popup)

---

## Code Organization Guidelines

### Module Structure (Mỗi crate/lib nên tuân theo)

```
src/
├── lib.rs / main.rs        # Entry point, public API re-exports
├── mod.rs                  # Module declarations (nếu có submodules)
├── [feature]/              # Feature-based modules
│   ├── mod.rs              # Public API của feature
│   ├── service/            # Business logic, background tasks
│   ├── ui/                 # Widget building, rendering
│   │   ├── view.rs         # Widget construction
│   │   ├── render.rs       # Data → Widget updates
│   │   └── mod.rs
│   └── controller/         # Event handling, keyboard, gestures
```

**Ví dụ:** `libs/babydra-island/src/features/media_player/`

```
media_player/
├── mod.rs          # MediaPlayerFeature + IslandFeature impl
├── service/
│   ├── mod.rs
│   ├── poll.rs     # Background playerctl polling
│   ├── art.rs      # Artwork loading
│   └── format.rs   # Time formatting, app icons
└── ui/
    ├── mod.rs
    ├── view.rs     # PlayerWidgets::build()
    ├── visualizer.rs
    ├── popover.rs
    └── render.rs   # update_player_view()
```

### Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Files/modules | snake_case | `context_menu.rs`, `grid_card.rs` |
| Structs/Enums | PascalCase | `PlayerWidgets`, `IslandViewHandle` |
| Functions/Methods | snake_case | `build_view`, `update_player_view` |
| Constants | SCREAMING_SNAKE | `PRIORITY`, `TARGET_WIDTH` |
| Traits | PascalCase | `IslandFeature` |
| Type aliases | PascalCase | `CoreResult<T>` |

### Public API Guidelines

1. **Re-export ở `lib.rs`** - Chỉ public những gì thực sự cần thiết
2. **Sử dụng `pub mod`** cho module, `pub use` cho items
3. **Prelude module** - Tạo `prelude` cho imports phổ biến (xem `ui-kit/src/lib.rs`)
4. **Document public API** - Mọi `pub` item phải có doc comment (`///`)

---

## Code Splitting & Refactoring Guidelines

### Khi nào tách file/module?

| Trigger | Action |
|---------|--------|
| File > 500 lines | Tách thành submodules |
| Function > 100 lines | Extract helper functions |
| Struct có > 10 fields | Xem xét split thành nested structs |
| Logic lặp lại ≥ 3 lần | Extract thành shared function/module |
| Module có > 5 submodules | Xem xét gom nhóm hoặc tách crate |

### Refactoring Patterns

#### 1. Extract Service Layer
```rust
// Trước: Logic混在 UI
fn build_ui(&self) {
    let data = fetch_from_dbus();  // Business logic
    render(&data);                  // UI
}

// Sau: Tách service
// service/poll.rs
pub fn spawn_polling() -> Receiver<Data> { ... }

// ui/view.rs
pub fn build() -> Widget { ... }

// ui/render.rs
pub fn update_view(widget: &Widget, data: &Data) { ... }

// mod.rs
struct Feature {
    service: Receiver<Data>,
    widgets: Widgets,
}
```

#### 2. Feature-Based Modules
Mỗi feature là một module độc lập với cấu trúc chuẩn:
- `mod.rs` - Struct chính + trait impl
- `service/` - Background logic, DBus, polling
- `ui/` - Widget building, rendering
- `controller/` - Input handling (nếu cần)

#### 3. Dependency Injection cho Testability
```rust
// Thay vì hardcode dependency
struct Feature {
    service: ConcreteService,
}

// Dùng trait + constructor injection
trait Service: Send + Sync {
    fn fetch(&self) -> Data;
}

struct Feature<S: Service> {
    service: S,
}

impl<S: Service> Feature<S> {
    fn new(service: S) -> Self { Self { service } }
}
```

#### 4. State Management
- **Local state**: Giữ trong struct feature (`Cell`, `RefCell`, `Rc<RefCell<_>>`)
- **Shared state**: Dùng `Rc<RefCell<_>>` hoặc channel (`tokio::sync::mpsc`)
- **Island state**: Dùng `IslandViewHandle` + `ViewState` (đã có sẵn)

---

## Error Handling

```rust
// core/error.rs
pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Config error: {0}")]
    Config(#[from] ConfigError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    // ...
}

// Sử dụng
fn load_config() -> CoreResult<Config> { ... }

// Trong UI: handle gracefully
match load_config() {
    Ok(cfg) => apply(cfg),
    Err(e) => {
        log::error!("Failed to load config: {e}");
        show_error_dialog(&e);
    }
}
```

---

## Async & Concurrency

| Scenario | Approach |
|----------|----------|
| Background polling (DBus, playerctl) | `tokio::spawn` + `mpsc::channel` → `glib::MainContext` |
| File I/O | `gio::File::read_async` hoặc `tokio::fs` |
| UI updates from background | `glib::MainContext::default().spawn_local` hoặc `sender.send().then(update_ui)` |
| One-shot async | `glib::spawn_future_local` |

**Lưu ý:** GTK4 không thread-safe. Luôn marshal về main thread trước khi touch widgets.

---

## Testing

```bash
# Unit tests (core logic)
cargo test -p babydra-core

# Integration tests
cargo test --test integration

# UI tests (headless với gtk4-test)
cargo test -p babydra-ui-kit --features test
```

---

## Build & Development

```bash
# Build all
cargo build --workspace

# Build specific crate
cargo build -p babydra-panel

# Run app
cargo run -p babydra-panel

# Check + clippy
cargo check --workspace && cargo clippy --workspace

# Format
cargo fmt --all
```

---

## Adding New Features

### 1. New System Service (core)
```
libs/babydra-core/src/services/new_service/
├── mod.rs        # Public API
├── service.rs    # Implementation
└── model.rs      # Domain models (nếu cần)
```
→ Re-export ở `core/src/services/mod.rs` và `core/src/lib.rs`

### 2. New UI Component (ui-kit)
```
libs/babydra-ui-kit/src/components/new_component/
├── mod.rs        # Public builders
├── builder.rs    # Widget construction
└── render.rs     # Data binding (nếu cần)
```
→ Add to `components/mod.rs` và `prelude`

### 3. New Island Feature (island)
```
libs/babydra-island/src/features/new_feature/
├── mod.rs        # Feature struct + IslandFeature impl
├── service/      # Background logic (nếu cần)
│   ├── mod.rs
│   └── ...
└── ui/           # Widgets
    ├── mod.rs
    ├── view.rs
    └── render.rs
```
→ Register trong `island/src/render.rs` → `create_system_island()`

---

## Code Review Checklist

- [ ] Dependency direction đúng (không import ngược layer)
- [ ] Public API có doc comment
- [ ] Error handling thích hợp (CoreResult, ? operator)
- [ ] Không block main thread (async đúng chỗ)
- [ ] Widget lifecycle đúng (no memory leaks)
- [ ] CSS classes tuân theo naming convention
- [ ] Tests cho logic quan trọng
- [ ] `cargo fmt`, `cargo clippy` pass