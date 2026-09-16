# 08 — API dùng chung

## Phạm vi

Trang này là bản đồ API thực hành của các thư viện được nhiều application sử dụng. API chi tiết nhất vẫn là public item trong source Rust; khi API thay đổi, cập nhật trang này cùng commit.

## `babydra-core`

`babydra-core` không phụ thuộc GTK. Application dùng thư viện này cho service, config, model và i18n.

### Service

| Module | Trách nhiệm |
| :--- | :--- |
| `services::system::wifi` | Scan, connect, disconnect và quản lý network. |
| `services::system::vpn` | Liệt kê, kết nối và ngắt VPN. |
| `services::system::volume` | Đọc, đặt volume và mute qua PipeWire/PulseAudio. |
| `services::system::brightness` | Đọc và đặt độ sáng qua backend được hỗ trợ. |
| `services::system::battery` | Trạng thái pin và nguồn điện. |
| `services::system::cpu` | Load, nhiệt độ và governor. |
| `services::wallpaper` | Đổi và theo dõi wallpaper. |
| `services::updates` | Kiểm tra cập nhật package. |
| `services::notification` | Gửi notification qua D-Bus. |

Application không nên gọi trực tiếp command hệ thống nếu service tương ứng đã có trong core.

### Config

```rust
use babydra_core::config;

let config = config::load_babydra_config();
config::apply_all_saved_settings();
```

Config chính nằm ở `~/.babydra/babydra.conf`. Các module phải dùng model config thay vì tự parse cùng một file theo cách riêng.

### i18n

```rust
use babydra_core::i18n::t;

let title = t("settings.wifi");
```

Key dịch nằm trong locale data của project. Không hardcode câu hiển thị trong widget nếu key đã tồn tại hoặc cần hỗ trợ nhiều ngôn ngữ.

## `babydra-ui-kit`

### Theme initialization

```rust
use babydra_ui_kit::ui::theme::init_theme;

init_theme();
```

Gọi một lần khi application khởi động, trước khi dựng UI. Theme resolver đọc selection từ config và tìm theme theo thứ tự được mô tả trong [05-themes-variants.md](05-themes-variants.md).

### Component factory

Các nhóm API chính:

| Nhóm | API tiêu biểu |
| :--- | :--- |
| Button | `create_icon_button`, `create_close_button` |
| Badge | `create_status_badge`, `create_icon_badge` |
| Card/list | `create_card`, `create_item_row`, `create_list_row`, `clear_list_box` |
| Switch/slider | `create_switch`, `ToggleRow`, `CustomSlider` |
| Modal | `PasswordDialog`, `WifiPasswordDialog`, `WifiConfigDialog`, `VpnConfigDialog` |
| Popover | `create_popover`, `attach_hover_popover` |
| Progress | `create_progress_bar`, `create_disk_progress` |
| Feedback | `create_placeholder_row`, `create_spinner`, `create_loading_box` |
| Icon/tooltip | `get_icon`, `get_system_or_file_icon`, `set_tooltip` |

Dùng factory chung để giữ class, spacing, accessibility và animation đồng nhất.

## `babydra-theme`

Theme API chịu trách nhiệm:

- resolve theme theo id;
- đọc `tokens.json`, `fonts.json` và CSS;
- resolve `base` theme;
- tạo CSS cuối cho dark/light;
- trả lỗi hoặc fallback khi package không đầy đủ.

Application không tự tìm theme bằng đường dẫn riêng. Mọi application phải đi qua theme library hoặc helper của ui-kit.

## `babydra-island`

API public gồm các thành phần để tạo island, đăng ký feature, cập nhật view và gửi dữ liệu từ background service. Quy tắc vòng đời được mô tả trong [07-dynamic-island.md](07-dynamic-island.md).

## Nguyên tắc sử dụng API

- Dùng service core cho system operation.
- Dùng component ui-kit cho widget chuẩn.
- Gọi `init_theme()` trước khi render.
- Gửi dữ liệu nền qua channel hoặc main-context callback.
- Thêm test cho logic mới không cần GTK nếu có thể.
- Nếu API chưa phù hợp, sửa abstraction dùng chung thay vì tạo bản sao trong application.
