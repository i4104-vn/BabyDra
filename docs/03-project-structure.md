# Chương 03: Cấu trúc Dự án và Quy chuẩn Viết mã

**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-07-23
**Phạm vi:** Quy chuẩn đặt tên thư mục, triết lý phân tách file, quy tắc viết mã nguồn mới

---

## Mục lục

- [1. Tổng quan cấu trúc thư mục](#1-tổng-quan-cấu-trúc-thư-mục)
- [2. Quy chuẩn đặt tên thư mục](#2-quy-chuẩn-đặt-tên-thư-mục)
- [3. Triết lý phân tách file mã nguồn](#3-triết-lý-phân-tách-file-mã-nguồn)
- [4. Quy tắc tổ chức thư viện babydra-common](#4-quy-tắc-tổ-chức-thư-viện-babydra-common)
- [5. Sơ đồ cấu trúc tham chiếu](#5-sơ-đồ-cấu-trúc-tham-chiếu)
- [6. Quy tắc chung khi viết mã nguồn mới](#6-quy-tắc-chung-khi-viết-mã-nguồn-mới)

---

## 1. Tổng quan cấu trúc thư mục

```
BabyDra/                          <- Thư mục gốc workspace Rust
    Cargo.toml                    <- Định nghĩa workspace, liệt kê tất cả members
    Cargo.lock                    <- Khóa phiên bản dependency (không chỉnh tay)
    docs/                         <- Toàn bộ tài liệu dự án
    libs/                         <- Các thư viện dùng chung (không thể chạy độc lập)
        babydra-common/           <- Thư viện lõi: logic nghiệp vụ, OS interaction
        babydra-utils/            <- Thư viện tiện ích: CSS, widget, theme
        babydra-island/           <- Widget Dynamic Island
        babydra-launcher/         <- Logic tìm kiếm ứng dụng
    crates/                       <- Các ứng dụng có thể thực thi
        babydra-panel/            <- Thanh taskbar chính
        babydra-switcher/         <- Alt-Tab Switcher
        babydra-screenshot/       <- Công cụ chụp màn hình
        babydra-lock/             <- Màn hình khóa
        babydra-preview/          <- Xem trước tệp
        babydra-settings/         <- Cài đặt hệ thống
        babydra-explore/          <- Trình quản lý tệp
    configs/                      <- File cấu hình mặc định
    install.sh                    <- Script cài đặt toàn bộ DE
    start.sh                      <- Script khởi động tất cả daemon
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
| `src/system_tray/` | `src/systemTray/` | camelCase không được phép |
| `src/control_center/` | `src/ControlCenter/` | PascalCase không được phép |
| `src/volume_slider/` | `src/volume-slider/` | kebab-case chỉ dùng ở tầng crate |

Ví dụ đúng:
- `libs/babydra-common/src/services/system/volume/`
- `libs/babydra-common/src/services/system/backlight/`
- `crates/babydra-panel/src/widgets/panel/items/darkmode/`

### 2.3. Tên file Rust

File Rust (`*.rs`) cũng dùng snake_case:

- `mod.rs` — file entry point của mỗi module (bắt buộc phải có tên này)
- `render.rs` — file dựng giao diện
- `helper.rs` — các hàm tiện ích

---

## 3. Triết lý phân tách file mã nguồn

Nguyên tắc cốt lõi là **Separation of Concerns (SoC)** — phân tách trách nhiệm. Mỗi file chỉ làm một việc cụ thể, không làm nhiều việc cùng lúc.

### 3.1. Cặp file chuẩn: mod.rs và render.rs

Mỗi module giao diện điển hình được tách thành tối thiểu hai file:

**`mod.rs` — Controller (Bộ điều phối)**

- Vai trò: File entry point của module. Đóng vai trò như một "bộ điều phối" kết nối dữ liệu và giao diện.
- Làm gì: Đăng ký các module con, ánh xạ dữ liệu nghiệp vụ từ `babydra-common`, gắn các callback xử lý sự kiện khi người dùng tương tác.
- Không làm gì: Không dựng widget GTK (không gọi `GtkBox::new()`, `GtkButton::new()`, v.v.).

```rust
// Ví dụ: crates/babydra-panel/src/widgets/panel/items/volume/mod.rs
pub mod render;

use babydra_common::services::system::volume;
use std::rc::Rc;
use std::cell::RefCell;

pub fn setup(state: Rc<RefCell<PanelState>>) -> gtk4::Widget {
    let widget = render::build();           // Gọi render.rs để dựng widget
    
    // Gắn callback: khi thanh trượt thay đổi, cập nhật hardware
    widget.scale.connect_value_changed(clone!(@strong state => move |s| {
        volume::set_volume(s.value() as u8);
        state.borrow_mut().volume = s.value() as u8;
    }));
    
    widget.container.into()
}
```

**`render.rs` — View (Bộ dựng giao diện)**

- Vai trò: Tập trung hoàn toàn vào việc dựng layout GTK (GtkBox, GtkButton, GtkScale), gán thuộc tính hiển thị và thêm CSS class cho widget.
- Làm gì: Tạo widget, xếp layout, đặt CSS class, set text placeholder.
- Không làm gì: Không đọc/ghi hệ thống, không gắn callback sự kiện nghiệp vụ (chỉ được gắn callback thuần giao diện như animation).

```rust
// Ví dụ: crates/babydra-panel/src/widgets/panel/items/volume/render.rs
use gtk4::prelude::*;
use gtk4::{Box, Scale, Image, Orientation};

pub struct VolumeWidget {
    pub container: Box,
    pub scale: Scale,
    pub icon: Image,
}

pub fn build() -> VolumeWidget {
    let container = Box::new(Orientation::Horizontal, 8);
    container.add_css_class("volume-slider-row");
    
    let icon = Image::from_icon_name("audio-volume-high-symbolic");
    icon.add_css_class("volume-icon");
    
    let scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
    scale.add_css_class("volume-scale");
    scale.set_hexpand(true);
    
    container.append(&icon);
    container.append(&scale);
    
    VolumeWidget { container, scale, icon }
}
```

### 3.2. Khi nào cần tách thêm file?

Nếu `mod.rs` vượt quá **150 dòng**, bắt buộc phải tách logic vào file con chuyên biệt:

| File con | Dùng khi |
| :--- | :--- |
| `helper.rs` | Các hàm tính toán nhỏ, chuyển đổi đơn vị, format string |
| `state.rs` | Định nghĩa State struct nếu phức tạp |
| `handler.rs` | Các callback sự kiện dài và phức tạp |

---

## 4. Quy tắc tổ chức thư viện babydra-common

Thư viện lõi `babydra-common` được tổ chức theo hai tầng hoàn toàn tách biệt:

### 4.1. Tầng dữ liệu: `src/models/`

- **Chứa gì:** Chỉ định nghĩa `struct` và `enum` thuần túy để vận chuyển và lưu trữ trạng thái. Không chứa thuật toán xử lý.
- **Không chứa:** Logic tính toán, gọi hệ thống, async code.

```rust
// Ví dụ: libs/babydra-common/src/models/audio.rs
#[derive(Debug, Clone)]
pub struct AudioState {
    pub volume: u8,          // 0-100
    pub is_muted: bool,
    pub device_name: String,
}
```

### 4.2. Tầng xử lý: `src/services/`

- **Chứa gì:** Mã lệnh giao tiếp trực tiếp với hệ điều hành (lệnh shell, đọc ghi file hệ thống `/sys/class`, socket IPC, D-Bus).
- **Không chứa:** GTK code, dữ liệu UI, struct model.

```
libs/babydra-common/src/services/
    system/
        volume/      <- Điều khiển âm lượng PipeWire/ALSA
        backlight/   <- Điều khiển độ sáng màn hình
        wifi/        <- Quản lý WiFi qua NetworkManager D-Bus
        bluetooth/   <- Quản lý Bluetooth qua BlueZ D-Bus
        battery/     <- Đọc thông tin pin từ /sys/class/power_supply/
    apps/            <- Quét ứng dụng .desktop, xếp hạng theo tần suất dùng
    window/          <- Lắng nghe sự kiện cửa sổ qua Wayland protocol
    notification/    <- D-Bus notification server
    screenshot/      <- Chụp màn hình qua XDG portal
```

### 4.3. Re-export để tạo API phẳng

Sử dụng `pub use` tại `lib.rs` hoặc `mod.rs` cấp cha để rút ngắn đường dẫn gọi API:

```rust
// libs/babydra-common/src/lib.rs
pub use services::system::auth::verify_password;
pub use services::system::volume::{get_volume, set_volume};
pub use models::audio::AudioState;
```

Nhờ đó, tầng View chỉ cần gọi ngắn gọn:

```rust
// Trong crates/babydra-panel/
use babydra_common::verify_password;   // Thay vì:
// use babydra_common::services::system::auth::verify_password;
```

---

## 5. Sơ đồ cấu trúc tham chiếu

Dưới đây là sơ đồ cấu trúc thực tế của một crate ứng dụng để làm mẫu khi phát triển module mới:

```
crates/babydra-panel/
    Cargo.toml
    src/
        main.rs              <- Điểm vào: khởi tạo GTK Application, gọi init_theme()
        render.rs            <- Cấu hình Layer Shell, ghép layout chính cửa sổ
        widgets/
            mod.rs           <- Khai báo và xuất bản danh sách tất cả widgets
            panel/
                mod.rs       <- Quản lý trạng thái Control Center
                items/
                    mod.rs   <- Xuất bản các toggle con
                    volume/
                        mod.rs    <- Bắt sự kiện âm lượng thay đổi, cập nhật hardware
                        render.rs <- Tạo icon loa, thanh trượt âm lượng GtkScale
                    backlight/
                        mod.rs    <- Bắt sự kiện độ sáng thay đổi, cập nhật hardware
                        render.rs <- Tạo icon độ sáng, thanh trượt GtkScale
                    wifi/
                        mod.rs
                        render.rs
                    darkmode/
                        mod.rs
                        render.rs
```

```
libs/babydra-common/
    Cargo.toml
    src/
        lib.rs               <- Re-export API phẳng cho toàn bộ services và models
        models/
            mod.rs
            audio.rs         <- AudioState struct
            network.rs       <- NetworkState struct
            battery.rs       <- BatteryState struct
        services/
            mod.rs
            system/
                mod.rs
                volume/
                    mod.rs   <- get_volume(), set_volume(), toggle_mute()
                backlight/
                    mod.rs   <- get_brightness(), set_brightness()
                wifi/
                    mod.rs   <- get_networks(), connect(), disconnect()
```

```
libs/babydra-utils/
    Cargo.toml
    src/
        lib.rs
        ui/
            theme/
                mod.rs       <- init_theme(): nạp CSS vào GDK Display
        styles/
            dark/            <- CSS chế độ tối
                shared/
                    button.css
                    switch.css
                    scrollbar.css
                    sidebar.css
                panel/
                    control_center.css
                    ...
            light/           <- CSS chế độ sáng (cùng tên file, giá trị khác)
                shared/
                    button.css
                    ...
        components/          <- Widget GTK tái sử dụng
```

---

## 6. Quy tắc chung khi viết mã nguồn mới

### Quy tắc 1: Không lạm dụng mod.rs

Hạn chế viết logic tính toán hoặc giao diện dài quá 150 dòng trong `mod.rs`. Khi vượt ngưỡng, tạo file con chuyên biệt và gọi từ `mod.rs`.

| Đúng | Sai |
| :--- | :--- |
| `mod.rs` chỉ gọi `render::build()` và gắn callback | `mod.rs` chứa cả code dựng widget lẫn logic tính toán |
| Tách logic dài vào `helper.rs` | Để 300 dòng trong một `mod.rs` |

### Quy tắc 2: Re-export trực quan

Dùng `pub use` tại `mod.rs` hoặc `lib.rs` cấp cha để tạo API phẳng.

### Quy tắc 3: Cách ly CSS hoàn toàn

Toàn bộ CSS của toàn bộ dự án phải đặt trong `libs/babydra-utils/src/styles/`. Không viết CSS inline trong Rust code.

| Đúng | Sai |
| :--- | :--- |
| `widget.add_css_class("volume-icon")` | `widget.set_css_classes(&["color: red; font-size: 14px"])` |
| Định nghĩa `.volume-icon { ... }` trong `styles/dark/panel/volume.css` | Gọi `GtkCssProvider::load_from_data()` trong file Rust riêng |

### Quy tắc 4: Không import GTK trong babydra-common

`babydra-common` phải là thư viện thuần Rust logic, không phụ thuộc vào GTK. Điều này đảm bảo logic nghiệp vụ có thể được test bằng `cargo test` bình thường mà không cần môi trường đồ họa.

### Quy tắc 5: Đặt tên CSS class theo BEM-like convention

CSS class đặt theo dạng `component-element` hoặc `component-element--modifier`:

- `volume-slider-row` — container chính của volume slider
- `volume-slider-row--muted` — trạng thái khi tắt tiếng
- `volume-icon` — icon loa
- `panel-toggle-btn` — nút toggle trong panel
- `panel-toggle-btn--active` — trạng thái đang bật
