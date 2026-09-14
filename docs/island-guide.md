# Tài liệu Kiến trúc và Hướng dẫn Phát triển Dynamic Island (babydra-island)

Tài liệu này đặc tả kiến trúc kỹ thuật, quy chuẩn phân rã mã nguồn, cách thức tái sử dụng các thành phần cốt lõi và quy trình mở rộng tính năng cho hệ thống Dynamic Island (`babydra-island`) trong giao diện hệ điều hành BabyDra.

---

## 1. Tổng quan Kiến trúc Hệ thống

Dynamic Island là hệ thống hiển thị thông tin dạng notch capsule tích hợp trên panel điều khiển. Hệ thống hoạt động theo cơ chế **điều phối tập trung (Centralized Arbitration)** nhằm quản lý việc chuyển đổi, hiển thị và tự động thu gọn các view dựa trên mức độ ưu tiên và sự kiện thời gian thực.

### 1.1. Luồng dữ liệu và Điều phối (Arbitration Mechanism)

Tại một thời điểm xác định, hệ thống capsule chỉ hiển thị một view chủ đạo hoặc hiển thị trạng thái chờ (Idle view). Việc xác định view chiến thắng (Winner view) tuân thủ theo 4 tầng ưu tiên:

1. **Override Deadline (`override_show_for`)**:
   Khi một sự kiện hệ thống khẩn cấp hoặc mang tính tức thời phát sinh (thay đổi âm lượng, độ sáng, cắm sạc, ngắt kết nối mạng), view gửi yêu cầu override trong một khoảng thời gian xác định (`Duration`). Trong thời gian này, view được giữ quyền hiển thị tuyệt đối, bỏ qua độ ưu tiên tĩnh.
2. **Arbitration Priority (`priority`)**:
   Khi có nhiều view cùng yêu cầu hiển thị mà không có override, view có chỉ số `priority` (từ 0 đến 255) cao hơn sẽ chiếm quyền hiển thị.
3. **Request Sequence (`request_seq`)**:
   Nếu hai view có cùng độ ưu tiên, hệ thống sử dụng bộ đếm đơn điệu nguyên tử (`AtomicU64`) để ưu tiên view phát sinh yêu cầu gần nhất.
4. **Active State / Liveness (`is_alive`)**:
   Nếu thời hạn override đã kết thúc, hệ thống kiểm tra trạng thái sống của feature thông qua phương thức `is_alive()`. Các feature có trạng thái duy trì (ví dụ popover đang mở, media đang phát) sẽ tiếp tục giữ capsule mà không bị thu hồi về trạng thái idle.

### 1.2. Chỉ báo Đa tác vụ (Bracket Indicators)

Khi có từ hai view trở lên cùng ở trạng thái hoạt động đồng thời (ví dụ: trình phát nhạc đang chạy nền và âm lượng được điều chỉnh), hệ thống tự động kích hoạt hiển thị cặp dấu ngoặc bao quanh capsule `( [Capsule Content] )` thông qua phương thức `update_brackets()`. Khi chỉ còn một view duy nhất, các chỉ báo này tự động ẩn để tối ưu diện tích thị giác.

---

## 2. Phân loại Tính năng (Feature Categorization)

Mã nguồn trong thư mục `src/features/` được phân tách thành hai nhóm kiến trúc rõ rệt dựa trên bản chất tương tác và vòng đời:

### 2.1. Nhóm Chỉ báo Hệ thống Ngắn hạn (Ephemeral Status Indicators - `features/system/*`)

* **Bao gồm**:
  * `volume`: Chỉ báo âm lượng và trạng thái tắt tiếng.
  * `brightness`: Chỉ báo độ sáng màn hình vật lý và DDC/CI.
  * `network`: Trạng thái kết nối Ethernet, Wi-Fi (kèm % tín hiệu), trạng thái ngắt kết nối.
  * `bluetooth`: Trạng thái kết nối thiết bị ngoại vi, % pin phụ kiện, trạng thái ngắt kết nối.
  * `battery`: Trạng thái cắm nguồn sạc AC (hiển thị 5 giây), cảnh báo pin yếu khi chạm ngưỡng `<= 20%` và không cắm sạc.
* **Đặc tính kỹ thuật**:
  * Thời gian tồn tại ngắn hạn (1.5 giây đến 5 giây).
  * Không có giao diện phụ (Popover/Dropdown).
  * Không chiếm quyền điều khiển bàn phím (`focus() = false`).
  * Chỉ sử dụng giao diện capsule chuẩn (`NotchWidget`).

### 2.2. Nhóm Tính năng Tương tác Phức hợp (Interactive Stateful Features)

* **Bao gồm**:
  * `media_player`: Điều khiển phát nhạc MPRIS, thanh tiến trình, artwork, popover chi tiết.
  * `clipboard`: Lịch sử bộ nhớ tạm, tìm kiếm, xem trước ảnh, popover phân trang.
  * `power`: Bảng điều khiển tắt máy, khởi động lại, ngủ, đăng xuất.
  * `notification`: Quản lý thông báo desktop, nội dung đa dòng, hành động tương tác.
* **Đặc tính kỹ thuật**:
  * Tồn tại lâu dài hoặc do người dùng trực tiếp kích hoạt.
  * Có Popover riêng thả xuống từ capsule (`attach(ctx)`).
  * Có bộ điều khiển bàn phím riêng biệt (`focus() = true`, chiếm `KeyboardMode::Exclusive` trên Wayland Layer Shell để điều hướng phím mũi tên, phím số, Enter, Escape).

---

## 3. Quy chuẩn Cấu trúc Thư mục và Chia File

Để đảm bảo nguyên tắc Clean Code và ngăn chặn hiện tượng phát sinh mã trung gian không cần thiết, quy chuẩn cấu trúc thư mục được thiết lập như sau:

### 3.1. Cấu trúc một System Feature chuẩn (`features/system/<feature_name>/`)

Mỗi tính năng trong nhóm `system` chỉ bao gồm đúng **2 file**:

```
libs/babydra-island/src/features/system/<feature_name>/
├── mod.rs        # Định nghĩa struct, triển khai trait IslandFeature, liên kết NotchWidget
└── service.rs    # Luồng chạy ngầm giám sát sự kiện, dispatch dữ liệu sang GTK Context
```

* **Lý do tối giản số lượng file**:
  1. **Không tạo lớp giao diện riêng**: Toàn bộ bố cục hiển thị capsule đã được chuẩn hóa bởi `NotchWidget`. Không tạo thêm thư mục `ui/` hay file CSS riêng.
  2. **Tận dụng dịch vụ lõi**: Toàn bộ thao tác truy xuất D-Bus, ALSA, Sysfs đã nằm sẵn trong `libs/babydra-core`. `service.rs` chỉ làm nhiệm vụ lắng nghe và đóng gói sự kiện.
  3. **Không tạo module re-export trung gian**: Khai báo trực tiếp trong `features/system/mod.rs` và đăng ký thẳng vào `IslandBuilder`.

### 3.2. Cấu trúc một Interactive Feature chuẩn (`features/<feature_name>/`)

Đối với các tính năng có giao diện popover và tương tác bàn phím, phân rã theo mô hình 3 lớp:

```
libs/babydra-island/src/features/<feature_name>/
├── mod.rs                  # Điểm nhập, triển khai IslandFeature, quản lý trạng thái
├── controller/             # Xử lý sự kiện người dùng
│   ├── keyboard.rs         # Bắt phím tắt, điều hướng mũi tên, phím số
│   └── mod.rs
├── service/                # Giao tiếp dịch vụ nền, polling, cache dữ liệu
│   └── mod.rs
└── ui/                     # Giao diện đồ họa
    ├── popover.rs          # Layout popover mở rộng
    └── mod.rs
```

---

## 4. Khai thác Các Thành phần Có sẵn

Khi xây dựng hoặc mở rộng tính năng, lập trình viên bắt buộc phải sử dụng lại các thành phần cốt lõi sẵn có thay vì triển khai lặp lại.

### 4.1. Capsule Layout Chuẩn hóa: `NotchWidget`

Module: `crate::island::ui::NotchWidget`

`NotchWidget` cung cấp bố cục capsule chuẩn gồm 3 vùng:
* Vùng bắt đầu (Start - trái): Icon vector kích thước 14px (sử dụng icon core tự động theo theme).
* Vùng trung tâm (Center - giữa): Tiêu đề văn bản (tự động cắt ngắn với dấu `...` khi tràn chuỗi, căn giữa tuyệt đối).
* Vùng kết thúc (End - phải, tùy chọn): Giá trị trạng thái hoặc tỷ lệ phần trăm (ví dụ: `80%`, `Mute`, `20%`).

```rust
use crate::island::ui::NotchWidget;

// 1. Khởi tạo widget có giá trị bên phải
let widgets = NotchWidget::with_value(
    "battery",                          // Tên định danh icon core
    &babydra_core::i18n::trans("island.charging"), // Tiêu đề i18n
    "85%"                               // Giá trị hiển thị
);

// 2. Cập nhật đồng thời icon, tiêu đề và giá trị khi có sự kiện
widgets.update_all("battery", "Low Battery", "20%");

// 3. Cập nhật chỉ icon và giá trị (giữ nguyên tiêu đề)
widgets.update("volume", "65%");
```

### 4.2. Hằng số Kích thước Tiêu chuẩn: `view.rs`

Module: `crate::island::view`

Mọi feature khi cài đặt phương thức `size(&self)` phải sử dụng các hằng số kích thước đã định nghĩa:

| Hằng số | Giá trị | Phạm vi áp dụng |
|:---|:---:|:---|
| `CAPSULE_WIDTH` | `180` px | Chiều rộng chuẩn cho toàn bộ System Features, Clipboard, Notification. |
| `PLAYER_CAPSULE_WIDTH` | `200` px | Chiều rộng mở rộng dành riêng cho Media Player nhằm chứa tiêu đề bài hát. |
| `CAPSULE_HEIGHT` | `30` px | Chiều cao cố định đồng nhất cho toàn bộ capsule trên notch. |

### 4.3. Hệ thống Icon Vector: `babydra-ui-kit`

Tất cả icon được nhúng dưới dạng SVG và biên dịch trực tiếp vào nhị phân thông qua `babydra_ui_kit::ui::icon`. Không sử dụng icon ngoài danh mục hoặc load file từ ổ cứng runtime.

* Lấy icon có màu mặc định theo theme: `babydra_ui_kit::ui::icon::get_icon(name, size)`
* Lấy icon có màu sắc chỉ định: `babydra_ui_kit::ui::icon::get_icon_colored(name, size, color_css)`
* Các định danh icon chuẩn cho System: `"volume"`, `"volume-low"`, `"volume-mute"`, `"brightness"`, `"brightness-low"`, `"brightness-medium"`, `"ethernet"`, `"wifi"`, `"bluetooth"`, `"battery"`, `"power"`, `"shield"`.

### 4.4. Hệ thống Đa ngôn ngữ (i18n)

Mọi chuỗi văn bản hiển thị trên capsule bắt buộc phải đi qua hàm dịch `babydra_core::i18n::trans`. Tuyệt đối không hardcode chuỗi ký tự cố định vào giao diện.

* Tệp cấu hình tiếng Anh: `libs/babydra-core/src/i18n/locales/common/en.json`
* Tệp cấu hình tiếng Việt: `libs/babydra-core/src/i18n/locales/common/vi.json`

Quy ước tiền tố: Tất cả khóa dịch thuộc Dynamic Island phải bắt đầu bằng `island.` (ví dụ: `island.disconnected`, `island.charging`, `island.low_battery`).

---

## 5. Hướng dẫn Từng bước Mở rộng Tính năng Mới

Dưới đây là quy trình chuẩn hóa để xây dựng một tính năng mới trong nhóm `features/system/`. Ví dụ triển khai tính năng chỉ báo trạng thái khóa phím hoa (`caps_lock`).

### Bước 1: Khai báo khóa bản địa hóa (i18n)

Bổ sung khóa vào `en.json` và `vi.json`:

```json
// en.json
"island.caps_lock_on": "Caps Lock On",
"island.caps_lock_off": "Caps Lock Off"

// vi.json
"island.caps_lock_on": "Bật Caps Lock",
"island.caps_lock_off": "Tắt Caps Lock"
```

### Bước 2: Xây dựng luồng lắng nghe sự kiện (`service.rs`)

Nguyên tắc bắt buộc:
* Lắng nghe trong một thread riêng có đặt tên (`std::thread::Builder::new().name(...)`).
* Không thực hiện tính toán nặng hay gọi lệnh shell bên ngoài (`Command::new`) trong vòng lặp chính. Sử dụng D-Bus hoặc đọc Sysfs.
* Sử dụng kênh truyền tin bất đồng bộ `tokio::sync::mpsc::unbounded_channel` để chuyển sự kiện về GTK Main Context thông qua `glib::MainContext::default().spawn_local`.

```rust
//! libs/babydra-island/src/features/system/caps_lock/service.rs

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapsLockEvent {
    Enabled,
    Disabled,
}

pub fn spawn_caps_lock_listener<F>(on_change: F)
where
    F: Fn(CapsLockEvent) + 'static,
{
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<CapsLockEvent>();

    // Chuyển tiếp sự kiện sang GTK main thread an toàn
    glib::MainContext::default().spawn_local(async move {
        while let Some(event) = receiver.recv().await {
            on_change(event);
        }
    });

    let running = Arc::new(AtomicBool::new(true));

    std::thread::Builder::new()
        .name("babydra-island-capslock-listener".into())
        .spawn(move || {
            let mut last_state = false;

            while running.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(200));

                let current_state = read_caps_lock_state(); // Hàm đọc trạng thái từ core/sysfs
                if current_state != last_state {
                    last_state = current_state;
                    let event = if current_state {
                        CapsLockEvent::Enabled
                    } else {
                        CapsLockEvent::Disabled
                    };
                    if sender.send(event).is_err() {
                        break;
                    }
                }
            }
        })
        .ok();
}
```

### Bước 3: Cài đặt Trait `IslandFeature` (`mod.rs`)

Triển khai cấu trúc dữ liệu chính, khởi tạo `NotchWidget` và liên kết với `IslandViewHandle`.

```rust
//! libs/babydra-island/src/features/system/caps_lock/mod.rs

pub mod service;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk4::prelude::*;

use crate::island::ui::NotchWidget;
use crate::island::view::{CAPSULE_HEIGHT, CAPSULE_WIDTH};
use crate::island::{IslandCtx, IslandFeature, IslandViewHandle};
use service::{spawn_caps_lock_listener, CapsLockEvent};

pub const PRIORITY: u8 = 93;
pub const SHOW_DURATION: Duration = Duration::from_millis(1500);

pub struct CapsLockFeature {
    handle_rc: Rc<RefCell<Option<IslandViewHandle>>>,
    widgets: NotchWidget,
}

impl CapsLockFeature {
    pub fn new() -> Self {
        let title = babydra_core::i18n::trans("island.caps_lock_off");
        let widgets = NotchWidget::new("lock", &title);

        Self {
            handle_rc: Rc::new(RefCell::new(None)),
            widgets,
        }
    }
}

impl IslandFeature for CapsLockFeature {
    fn id(&self) -> &str {
        "system_caps_lock"
    }

    fn priority(&self) -> u8 {
        PRIORITY
    }

    fn size(&self) -> (i32, i32) {
        (CAPSULE_WIDTH, CAPSULE_HEIGHT)
    }

    fn build_view(&mut self) -> gtk4::Widget {
        self.widgets.container.clone().upcast()
    }

    fn init(&mut self, handle: &IslandViewHandle) {
        *self.handle_rc.borrow_mut() = Some(handle.clone());

        let handle_rc = self.handle_rc.clone();
        let widgets = self.widgets.clone();

        spawn_caps_lock_listener(move |event: CapsLockEvent| {
            let (title_key, color) = match event {
                CapsLockEvent::Enabled => ("island.caps_lock_on", "#ffffff"),
                CapsLockEvent::Disabled => ("island.caps_lock_off", "rgba(255, 255, 255, 0.60)"),
            };

            widgets.set_title(&babydra_core::i18n::trans(title_key));
            widgets.set_icon("lock", color);

            // Kích hoạt hiển thị override trong 1.5s
            if let Some(h) = handle_rc.borrow().as_ref() {
                h.override_show_for(SHOW_DURATION);
            }
            crate::island::tick_default_island();
        });
    }

    fn tick(&mut self, _ctx: &IslandCtx) {
        // Tối ưu: Sự kiện kích hoạt hoàn toàn hướng sự kiện (Event-driven), tick để trống
    }
}
```

### Bước 4: Đăng ký Feature vào Module Quản lý

Thêm module mới vào `libs/babydra-island/src/features/system/mod.rs`:

```rust
pub mod battery;
pub mod bluetooth;
pub mod brightness;
pub mod caps_lock; // Module mới
pub mod network;
pub mod volume;

pub use battery::BatteryFeature;
pub use bluetooth::BluetoothFeature;
pub use brightness::BrightnessFeature;
pub use caps_lock::CapsLockFeature;
pub use network::NetworkFeature;
pub use volume::VolumeFeature;

pub fn register_system_features(
    builder: crate::island::IslandBuilder,
) -> crate::island::IslandBuilder {
    builder
        .feature(Box::new(VolumeFeature::new()))
        .feature(Box::new(BrightnessFeature::new()))
        .feature(Box::new(BatteryFeature::new()))
        .feature(Box::new(NetworkFeature::new()))
        .feature(Box::new(BluetoothFeature::new()))
        .feature(Box::new(CapsLockFeature::new())) // Đăng ký builder
}
```

Đồng thời đăng ký trong `libs/babydra-island/src/render.rs` (`build_default_island`).

---

## 6. Bảng Quy chuẩn Tham số Tính năng Hiện hành

Dưới đây là bảng thông số cấu hình chính thức của toàn bộ các tính năng đang vận hành trên Dynamic Island:

| Feature ID | Nhóm phân loại | Priority | Show Duration | Kích thước Capsule (W x H) | Ghi chú tương tác |
|:---|:---|:---:|:---:|:---:|:---|
| `system_volume` | System Ephemeral | 95 | 1500 ms | 180 x 30 px | Lăn chuột / phím âm lượng |
| `system_brightness` | System Ephemeral | 95 | 1500 ms | 180 x 30 px | Phím chức năng màn hình |
| `system_battery` | System Ephemeral | 94 | 5000 ms | 180 x 30 px | Cắm sạc AC / Cảnh báo pin <= 20% |
| `system_network` | System Ephemeral | 92 | 2000 ms | 180 x 30 px | Ethernet / Wi-Fi % / Ngắt kết nối |
| `system_bluetooth` | System Ephemeral | 92 | 2000 ms | 180 x 30 px | Kết nối thiết bị, % pin tai nghe |
| `clipboard` | Interactive | 80 | - | 180 x 30 px | Popover danh sách, bắt phím `↑↓` |
| `notification` | Interactive | 70 | 5000 ms | 180 x 30 px | Popover chi tiết thông báo |
| `media_player` | Interactive | 50 | - | 200 x 30 px | Trạng thái phát nhạc nền MPRIS |
| `idle_logo` | System Idle | 0 | - | 44 x 30 px | Hiển thị khi không có tác vụ active |

---

## 7. Nguyên tắc Kiểm thử và Đảm bảo Chất lượng

Trước khi hoàn tất tích hợp bất kỳ tính năng Dynamic Island nào, lập trình viên phải đảm bảo thỏa mãn các tiêu chí sau:

1. **Kiểm tra biên dịch và chuẩn mã tĩnh**:
   ```bash
   cargo check --workspace
   cargo clippy -p babydra-core -p babydra-island
   ```
   Mã nguồn phát triển mới phải đạt tuyệt đối **0 cảnh báo (warnings)** từ Clippy.
2. **Không khóa luồng chính (Zero Main Thread Blocking)**:
   Mọi I/O đọc sysfs, truy vấn D-Bus hay socket phải được cô lập hoàn toàn trong background thread. Main thread GTK chỉ nhận struct sự kiện thông qua channel để cập nhật widget.
3. **Giải phóng tài nguyên (Resource Cleanliness)**:
   Khi feature bị hủy hoặc ứng dụng reload (`update.sh`), các thread nền phải có cờ kiểm soát (`AtomicBool`) hoặc thoát tự nhiên khi receiver bị drop, tránh tạo tiến trình rác hay rò rỉ bộ nhớ.