# Hướng dẫn sử dụng Core, Services, UI Kit

## Tổng quan dependency

```
┌─────────────────────────────────────┐
│        Applications (crates/*)      │  ← Panel, Desktop, Explore, Settings, ...
├─────────────────────────────────────┤
│         babydra-island              │  ← Dynamic Island manager + features
├─────────────────────────────────────┤
│         babydra-ui-kit              │  ← Reusable GTK4 components, animations
├─────────────────────────────────────┤
│         babydra-core                │  ← Config, Services, Models, System APIs
├─────────────────────────────────────┤
│         babydra-theme               │  ← CSS, colors, icons
└─────────────────────────────────────┘
```

**Quy tắc:** Chỉ import **xuôi chiều** (từ trên xuống dưới). Không bao giờ import ngược.

---

## babydra-core

### 1. Config (Cấu hình)

```rust
use babydra_core::{
    load_babydra_config, save_babydra_config, BabyDraConfig,
    load_desktop_config, save_desktop_config, DesktopConfig,
    load_explore_cfg, save_explore_cfg, ExploreSettings,
    get_config_dir, get_conf_path,
};

// Load config (tự động tạo default nếu chưa có)
let config: BabyDraConfig = load_babydra_config()?;
let desktop: DesktopConfig = load_desktop_config()?;
let explore: ExploreSettings = load_explore_cfg()?;

// Modify và save
config.theme.selection.dark = Some(true);
save_babydra_config(&config)?;

// Config dir path
let dir = get_config_dir();  // ~/.config/babydra
let path = get_conf_path();  // ~/.config/babydra/babydra.conf
```

**Config structs chính:**
- `BabyDraConfig` - Theme, clipboard, notification, power, shell, wallpaper
- `DesktopConfig` - Grid, icons, wallpaper settings
- `ExploreSettings` - File manager preferences
- `ClipboardConfig`, `NotificationConfig`, `PowerConfig`, `ShellConfig`, `ThemeConfig`, `WallpaperConfig`

### 2. Models (Domain Models)

```rust
use babydra_core::models::{
    explore::{FileEntry, FileType, SessionState, TabState, ActivePane, get_group_name},
    shell::{BatteryInfo, PerformanceProfile, Workspace, IslandState, Monitor, Notification},
    settings::{AppInfo, Keybind, Shortcut, WifiState, BluetoothState, ...},
    screenshot::ScreenshotData,
    network::NetworkInfo,
};

// FileEntry - Core model cho file manager
let entry = FileEntry {
    name: "document.pdf".into(),
    path: "/home/user/document.pdf".into(),
    file_type: FileType::File,
    size: Some(1024),
    modified: Some(chrono::Utc::now()),
    mime_type: Some("application/pdf".into()),
    is_hidden: false,
    is_symlink: false,
};

// BatteryInfo
if let Some(battery) = get_battery_info() {
    println!("Level: {}%, Charging: {}", battery.percentage, battery.charging);
}

// Workspace
let workspaces = get_workspaces();
let current = get_current_workspace();
switch_workspace(2);  // Switch to workspace 2
```

### 3. Services (System Services)

#### Clipboard
```rust
use babydra_core::services::clipboard::{
    push_entry, get_entries, get_shortcut, copy_to_system_clipboard,
    spawn_clipboard_watcher, trigger_clipboard, is_clipboard_enabled, ClipboardEntry,
};

// Push entry
push_entry(ClipboardEntry {
    text: "Hello world".into(),
    timestamp: chrono::Utc::now(),
    is_image: false,
});

// Get history
let entries = get_entries();  // Vec<ClipboardEntry>

// Copy to system clipboard
copy_to_system_clipboard("text to copy");

// Spawn background watcher (call once at startup)
spawn_clipboard_watcher();
```

#### Wallpaper
```rust
use babydra_core::services::wallpaper::{
    apply_wallpaper, set_wallpaper, set_wallpaper_with_mode,
    get_wallpaper, get_wallpaper_dir, get_wallpaper_mode,
    get_local_wallpapers, get_live_wallpapers, get_static_wallpapers,
    get_or_create_thumbnail, get_or_create_first_frame,
    is_video_file, is_gif_file, is_static_wallpaper_file,
    apply_greeter_wp, sync_shared_assets,
};

// Apply saved wallpaper at startup
apply_wallpaper();

// Set new wallpaper
set_wallpaper_with_mode("/path/to/image.jpg", WallpaperMode::Fill)?;

// Get thumbnails (async, returns path)
let thumb = get_or_create_thumbnail("/path/to/video.mp4").await?;
```

#### System Services (Hardware & OS Integration)

```rust
// Battery
use babydra_core::services::system::battery::{get_battery_info, apply_battery_saver};
let info = get_battery_info();  // Option<BatteryInfo>

// Bluetooth
use babydra_core::services::system::bluetooth::{
    get_bt_devices, is_bluetooth_enabled, set_bt_enabled, BtDevice,
};
let devices = get_bt_devices();
set_bt_enabled(true);

// Display
use babydra_core::services::system::display::{
    get_displays, apply_display_configs, apply_saved_displays, save_displays,
};

// Network
use babydra_core::services::system::network::{
    get_active_network_info, get_local_ip, get_network_speed,
};

// Power
use babydra_core::services::system::power::{
    get_current_profile, apply_saved_profile, set_perf_profile,
    poweroff, reboot, suspend, PerformanceProfile,
};
set_perf_profile(PerformanceProfile::Performance)?;

// Volume/Audio
use babydra_core::services::system::volume::{
    get_audio_backend, AudioBackendType, AudioDevice,
};
let devices = volume::get_output_devices();

// VPN
use babydra_core::services::system::vpn::{get_vpn_connections, VpnConn};

// WiFi
use babydra_core::services::system::wifi::{
    scan_wifi, connect_wifi, disconnect_wifi, forget_wifi, WifiNetwork,
};

// Storage
use babydra_core::services::system::storage::{get_disks, DiskInfo};

// Monitor/Resources
use babydra_core::services::system::monitor::{
    get_app_resource_usage, get_formatted_uptime, AppResourceUsage,
};

// Tray
use babydra_core::services::tray::{spawn_tray_service, TrayItem};

// Window Management
use babydra_core::services::window::{
    get_active_window, focus_window, close_window, close_all_windows,
};
use babydra_core::services::window::mru::{
    get_running_apps, activate_app, save_history, get_history,
};

// Apps (Desktop entries)
use babydra_core::services::apps::{find_desktop_apps, refresh_desktop_apps, DesktopApp};

// MPRIS (Media Player)
use babydra_core::services::mpris::{run_playerctl, decode_uri};

// Notification
use babydra_core::services::notification::service::{
    send_notification, send_app_notif, send_notif_icon, ActiveNotification,
};
send_notification("Title", "Body", Some("icon-name"));

// Screenshot
use babydra_core::services::screenshot::{
    capture_screen, capture_fullscreen, trigger_save, get_screenshot_path,
};

// Search
use babydra_core::services::search::{search_files, SearchResult};

// Clock
use babydra_core::services::clock::format_clock_date;

// EXIF
use babydra_core::services::exif::{read_exif, ExifData};

// Workspace
use babydra_core::services::workspace::{
    get_workspaces, get_current_workspace, switch_workspace,
    next_workspace, prev_workspace, sync_workspace_apps,
    filter_apps_for_workspace, DEFAULT_WORKSPACE_COUNT,
};
```

#### Explore Services (File Operations)
```rust
use babydra_core::services::explore::{
    load_directory, copy_path, move_path, delete_path, rename_path,
    send_to_trash, calc_dir_size, filter_entries, sort_entries,
    get_icon_name, get_owner_group, read_image_metadata,
    load_cropped_square, start_dbus_service, FileWatcher,
    parse_shortcut, matches_shortcut, clean_modifiers, shortcuts,
};
```

### 4. Apply Saved Settings (Startup)

```rust
use babydra_core::apply_saved_settings;

// Gọi một lần khi khởi động ứng dụng chính (panel, desktop, greeter)
apply_saved_settings();

// Làm:
// 1. CPU Performance Profile
// 2. Display Monitor configs
// 3. System Wallpaper
// 4. Greeter Wallpaper
// 5. Auto Battery Saver
// 6. Labwc Titlebar Theme
// 7. Kitty Terminal Theme
```

### 5. Error Handling

```rust
use babydra_core::{CoreError, CoreResult};

// Tất cả services trả về CoreResult
fn do_something() -> CoreResult<()> {
    let config = load_babydra_config()?;  // ? operator
    let battery = get_battery_info().ok_or(CoreError::NotFound("battery"))?;
    Ok(())
}
```

---

## babydra-ui-kit

### 1. Prelude (Import nhanh)

```rust
use babydra_ui_kit::prelude::*;

// Có sẵn:
// - Component builders: create_button, create_fab, create_card, create_switch_card, ...
// - ContextMenuBuilder, PasswordDialog, WifiConfigDialog, ...
// - Theme: init_theme, apply_theme_class, is_dark_mode, set_dark_mode
// - Icons: get_icon, get_icon_colored, get_fallback_icon, get_resolved_icon
// - Animations: slide_in, slide_out, genie_in, genie_out, island_animate_size, ...
// - Battery: create_battery_area, draw_cairo_battery, get_battery_hex
// - Image: apply_rounded_mask, create_circle_avatar, crop_square
// - Window: init_layer_window, setup_click_outside_dismiss
```

### 2. Buttons

```rust
use babydra_ui_kit::components::buttons::{create_button, create_icon_btn, create_fab, create_icon_button, create_danger_btn, create_accent_button, create_toggle_tile, create_square_tile, create_color_btn, create_colored_icon, create_rssi_icon, create_battery_icon, create_sys_wifi_icon, create_wifi_icon, create_wifi_net_icon, create_vpn_icon, create_wp_thumb, render_wifi_svg};

// Standard button
let btn = create_button("Click me");
btn.connect_clicked(|_| println!("Clicked"));

// Icon button
let btn = create_icon_btn("audio-volume-high-symbolic");
btn.set_tooltip_text(Some("Volume"));

// FAB (Floating Action Button)
let fab = create_fab("plus");
fab.connect_clicked(|_| show_dialog());

// Toggle tile (cho settings)
let tile = create_toggle_tile("Dark Mode", "preferences-system-symbolic", true);
tile.connect_toggled(|_, active| set_dark_mode(active));

// Square tile
let tile = create_square_tile("WiFi", "network-wireless-symbolic", true);
```

### 3. Cards & Lists

```rust
use babydra_ui_kit::components::cards::{create_card, create_switch_card, create_scroll_list, create_css_card, create_footer_box, create_footer_btn, create_group_header, create_subtitle, create_title, create_placeholder, PlaceholderState};

// Standard card
let card = create_card();
card.set_child(Some(&content_widget));

// Switch card (setting row với toggle)
let row = create_switch_card("Bluetooth", true);
row.connect_toggled(|_, active| set_bluetooth_enabled(active));

// Scrollable list
let list = create_scroll_list();
list.append(&item1);
list.append(&item2);

// CSS Card (custom styling)
let card = create_css_card("my-custom-class");
```

### 4. Context Menus

```rust
use babydra_ui_kit::components::context_menu::{ContextMenuBuilder, create_menu_for, create_menu_full, create_menu_item, create_menu_sep, create_menu_text, create_menu_shortcut, create_menu_sens, create_menu_popover, show_tray_menu, close_tray_menu, clear_box, clear_list_box, build_tray_gio_menu, create_submenu_item};

// Builder pattern
let menu = ContextMenuBuilder::new()
    .item("Open", "document-open-symbolic", || open_file())
    .separator()
    .item("Copy", "edit-copy-symbolic", || copy())
    .submenu("More", |sub| {
        sub.item("Rename", "edit-rename-symbolic", || rename())
        sub.item("Delete", "user-trash-symbolic", || delete())
    })
    .build();

// Show at pointer
menu.popup_at_pointer(None::<&gdk::Event>);

// Tray menu (cho system tray)
let tray_menu = build_tray_gio_menu(&tray_items);
show_tray_menu(&tray_menu, &button);
```

### 5. Modals & Dialogs

```rust
use babydra_ui_kit::components::modals::{
    PasswordDialog, WifiConfigDialog, WifiInfoDialog, WifiPasswordDialog,
    VpnConfigDialog, VpnLogDialog, ChangeHostnameDialog, ChangeNameDialog,
    ChangePasswordDialog, dialog_builder, create_popover,
};

// Password dialog
let dialog = PasswordDialog::new("Authentication Required", "Enter your password:");
dialog.connect_response(|dialog, response| {
    if response == gtk4::ResponseType::Ok {
        let password = dialog.password();
        verify(password);
    }
    dialog.close();
});
dialog.present();

// WiFi config dialog
let dialog = WifiConfigDialog::new(&wifi_network, saved_password);
dialog.connect_response(|dialog, response| {
    if response == gtk4::ResponseType::Ok {
        let (ssid, password) = dialog.get_credentials();
        connect_wifi(ssid, password);
    }
});

// Generic dialog builder
let dialog = dialog_builder()
    .title("Confirm")
    .message("Are you sure?")
    .buttons(&[("Cancel", gtk4::ResponseType::Cancel), ("Yes", gtk4::ResponseType::Ok)])
    .build();
```

### 6. Sliders

```rust
use babydra_ui_kit::components::slider::{PillSlider, CustomSlider, DebouncedSlider, CustomSwitch};

// Pill slider (volume, brightness)
let slider = PillSlider::new(0.0, 100.0, 50.0);
slider.connect_value_changed(|_, value| set_volume(value as i32));

// Debounced slider (chỉ emit sau khi user dừng kéo)
let slider = DebouncedSlider::new(0.0, 1.0, 0.5, Duration::from_millis(150));
slider.connect_value_changed(|_, value| set_opacity(value));

// Custom switch
let switch = CustomSwitch::new(true);
switch.connect_state_set(|_, state| set_enabled(state));
```

### 7. Explore Components (File Manager Specific)

```rust
use babydra_ui_kit::components::explore::{
    // Dialogs
    AlertDialog, ArchiveDialog, ConfirmDialog, ConflictDialog, DecompressDialog,
    JobLogDialog, NewFileDialog, NewFolderDialog, OpenWithDialog, PropertiesDialog,
    RenameDialog, ShellDialog,
    // Context Menu
    ContextMenuBuilder, FileContextMenu, ClipboardContextMenu, DimmingContextMenu,
    EmptyActionsContextMenu, MoreContextMenu,
    // Drag & Drop
    DragSource, DragTarget,
    // Items
    GridCard, ListRow,
    // Selection
    SelectionManager,
    // Helpers
    format_size, format_date, get_mime_icon, archive::extract, trash::send_to_trash,
};

// Grid card (icon view)
let card = GridCard::new(&file_entry);
card.connect_activate(|_| open_file());

// List row (detail view)
let row = ListRow::new(&file_entry);
row.connect_activate(|_| open_file());

// Properties dialog
let dialog = PropertiesDialog::new(&file_entries);
dialog.present();
```

### 8. Theme & Icons

```rust
use babydra_ui_kit::ui::{init_theme, apply_theme_class, is_dark_mode, set_dark_mode};
use babydra_ui_kit::ui::icon::{get_icon, get_icon_colored, get_fallback_icon, get_resolved_icon, get_icon_from_svg, get_logo_png, set_image_from_icon};
use babydra_ui_kit::ui::theme::{Colors, COLOR_*};

// Khởi tạo theme (gọi sớm nhất có thể)
init_theme();

// Check/set dark mode
if is_dark_mode() { ... }
set_dark_mode(true);

// Apply CSS class to widget
apply_theme_class(&widget, "card");  // Adds .card, .card:dark etc.

// Icons
let icon = get_icon("audio-volume-high-symbolic");
let colored = get_icon_colored("network-wireless-symbolic", &Colors::accent());
let image = get_resolved_icon("custom-icon", 24);  // Tự tìm trong theme/hicolor

// Set image từ icon name
set_image_from_icon(&image_widget, "folder-symbolic", 24);
```

### 9. Animations

```rust
use babydra_ui_kit::ui::animation::{
    easing::{ease_in_cubic, ease_out_cubic, ease_in_out_cubic, ease_out_back, ease_out_quart, linear},
    slide::{slide_in, slide_out, slide_out_cb, SlideDirection},
    genie::{genie_in, genie_out},
    island::{island_animate_size, island_animate_width, island_zoom_in, island_zoom_out},
    topbar::topbar_startup_cascade,
};

// Slide animation
slide_in(widget, SlideDirection::FromTop, 300, || {});
slide_out(widget, SlideDirection::ToBottom, 300, || widget.hide());

// Genie effect (minimize/restore)
genie_in(widget, target_rect, 400, || {});
genie_out(widget, start_rect, 400, || {});

// Island animations (dùng bởi island internal)
island_zoom_in(capsule.upcast_ref(), 200, 30, 350);
island_animate_size(capsule.upcast_ref(), cur_w, target_w, cur_h, target_h, 350, || {});
```

### 10. Battery, Image, Window Helpers

```rust
// Battery drawing
use babydra_ui_kit::ui::battery::{create_battery_area, draw_cairo_battery, get_battery_hex, get_battery_rgb};
let area = create_battery_area();
area.set_draw_func(|_, cr, _, _| draw_cairo_battery(cr, 80, true, false));

// Image masking
use babydra_ui_kit::ui::image::{apply_circular_mask, apply_rounded_mask, create_circle_avatar, create_rounded_picture, crop_circle, crop_rounded, crop_square, crop_square_pixbuf};
let avatar = create_circle_avatar(&pixbuf, 48);

// Window setup
use babydra_ui_kit::ui::window::{init_layer_window, setup_click_outside_dismiss};
let window = init_layer_window(gtk4::LayerShellEdge::Top, "my-panel", true);
setup_click_outside_dismiss(&window, || window.hide());
```

---

## Pattern: Sử dụng Core + UI Kit + Island trong App

### Ví dụ: Panel App

```rust
// crates/babydra-panel/src/main.rs
use babydra_core::{
    apply_saved_settings,
    services::{
        system::power::apply_saved_profile,
        wallpaper::apply_wallpaper,
        tray::spawn_tray_service,
    },
};
use babydra_ui_kit::prelude::*;
use babydra_island::{create_system_island, default_island};

fn main() -> CoreResult<()> {
    // 1. Init GTK
    let app = gtk4::Application::builder()
        .application_id("com.babydra.Panel")
        .build();
    
    app.connect_startup(|_| {
        // 2. Apply saved settings
        apply_saved_settings();
        
        // 3. Init UI Kit theme
        init_theme();
        
        // 4. Start system services
        spawn_tray_service();
    });
    
    app.connect_activate(|app| {
        // 5. Create island
        let island = create_system_island();
        
        // 6. Build panel UI using ui-kit components
        let window = build_panel_window(&island);
        
        window.present();
    });
    
    app.run();
    Ok(())
}

fn build_panel_window(island: &Island) -> gtk4::Window {
    let window = init_layer_window(
        gtk4::LayerShellEdge::Top,
        "babydra-panel",
        true,
    );
    
    let main_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    main_box.add_css_class("panel");
    
    // Left: Workspaces, App menu
    let left = create_workspace_widget();
    main_box.append(&left);
    
    // Center: Island capsule
    main_box.append(&island.capsule());
    
    // Right: System tray, Clock, Battery, Volume
    let right = create_system_status_widgets();
    main_box.append(&right);
    
    window.set_child(Some(&main_box));
    window
}
```

### Ví dụ: Settings App

```rust
// crates/babydra-settings/src/main.rs
use babydra_core::{
    load_babydra_config, save_babydra_config,
    services::system::{bluetooth, wifi, display, power, volume},
};
use babydra_ui_kit::prelude::*;

fn build_wifi_page() -> gtk4::Widget {
    let list = create_scroll_list();
    
    // Scan networks
    let networks = wifi::scan_wifi();
    for net in networks {
        let row = create_wifi_row(&net);
        row.connect_activate(move |_| {
            let dialog = WifiConfigDialog::new(&net, None);
            dialog.connect_response(move |d, resp| {
                if resp == gtk4::ResponseType::Ok {
                    let (ssid, pass) = d.get_credentials();
                    wifi::connect_wifi(ssid, pass).unwrap();
                }
            });
            dialog.present();
        });
        list.append(&row);
    }
    
    list.upcast()
}

fn build_display_page() -> gtk4::Widget {
    let displays = display::get_displays();
    // Build UI for each display: resolution, refresh rate, scale, position
    // On change: display::apply_display_configs(&configs)
}
```

---

## Best Practices

### 1. Import Style
```rust
// ✅ Good: Prelude cho common items
use babydra_ui_kit::prelude::*;

// ✅ Good: Explicit import cho specific items
use babydra_core::services::wifi::{scan_wifi, connect_wifi};
use babydra_core::models::explore::FileEntry;

// ❌ Bad: Wildcard từ deep modules
use babydra_ui_kit::components::explore::dialogs::*;

// ❌ Bad: Import core từ ui-kit hoặc island
// use babydra_ui_kit::core::...  // KHÔNG CÓ core trong ui-kit
```

### 2. Service Calls trong UI
```rust
// ✅ Good: Async service call, update UI khi done
fn on_connect_wifi(&self, ssid: &str, password: &str) {
    let ssid = ssid.to_string();
    let password = password.to_string();
    let sender = self.sender.clone();
    
    glib::spawn_future_local(async move {
        let result = wifi::connect_wifi(&ssid, &password).await;
        sender.send(UiMsg::WifiResult(result)).ok();
    });
}

// ❌ Bad: Block main thread
fn on_connect_wifi(&self, ssid: &str, password: &str) {
    wifi::connect_wifi(ssid, password).unwrap();  // BLOCKS UI!
}
```

### 3. Config Access
```rust
// ✅ Good: Load once, cache, save on change
struct SettingsPage {
    config: Rc<RefCell<BabyDraConfig>>,
}

impl SettingsPage {
    fn on_theme_change(&self, dark: bool) {
        self.config.borrow_mut().theme.selection.dark = Some(dark);
        save_babydra_config(&self.config.borrow()).ok();
        set_dark_mode(dark);
    }
}

// ❌ Bad: Load config mỗi lần render
fn render(&self) {
    let config = load_babydra_config().unwrap();  // SLOW!
}
```

### 4. Memory Management
```rust
// ✅ Good: Weak references cho cycles
struct Feature {
    handle: Option<IslandViewHandle>,
    popover: RefCell<Option<Popover>>,
    // Service receiver giữ bằng Rc, drop khi feature drop
    receiver: Option<mpsc::UnboundedReceiver<Data>>,
}

// ✅ Good: Cleanup ở on_hide/dispose
impl IslandFeature for Feature {
    fn on_hide(&mut self) {
        self.popover.borrow().as_ref().map(|p| p.popdown());
    }
}
```

---

## Migration Guide (Từ code cũ)

| Cũ | Mới (core/ui-kit/island) |
|----|--------------------------|
| `gtk4::Button::with_label` | `create_button("label")` |
| Manual CSS classes | `apply_theme_class(&widget, "card")` |
| Custom icon loading | `get_icon("name")`, `get_resolved_icon("name", size)` |
| Manual animation | `slide_in`, `genie_in`, `island_zoom_in` |
| Manual DBus calls | `babydra_core::services::system::*` |
| Manual config parsing | `load_babydra_config()`, `save_babydra_config()` |
| Custom context menu | `ContextMenuBuilder` |
| Custom dialogs | `PasswordDialog`, `WifiConfigDialog`, `ConfirmDialog` |
| Manual island logic | `babydra_island::create_system_island()`, `IslandFeature` |

---

## Testing Services

```rust
// tests/test_wifi.rs
use babydra_core::services::system::wifi;

#[test]
fn test_scan_wifi() {
    let networks = wifi::scan_wifi();
    assert!(!networks.is_empty());  // Có thể mock
}

#[test]
fn test_connect_wifi() {
    // Integration test cần DBus thật
}
```

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `CoreError::Config` | Check `~/.config/babydra/babydra.conf` permissions |
| Theme không apply | Gọi `init_theme()` trước khi tạo widgets |
| Icons không hiện | Kiểm tra icon theme (Adwaita, Papirus), dùng `get_fallback_icon` |
| Island không animate | Đảm bảo `babydra_ui_kit::ui::animation::island` functions available |
| Service trả về None | Hardware không hỗ trợ (vd: không có battery, bluetooth) |
| Deadlock | Không lock `RefCell`/`Mutex` khi gọi GTK callbacks |