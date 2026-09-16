# 07 — Dynamic Island

## Phạm vi

Dynamic Island là một capsule nằm trong panel, hiển thị một trạng thái ngữ cảnh tại một thời điểm. Thư viện `babydra-island` không chứa layout riêng của panel; nó cung cấp manager, view, feature và cơ chế chuyển trạng thái để panel hoặc ứng dụng khác lắp ghép.

## Cấu trúc mã nguồn

```text
libs/babydra-island/src/
├── lib.rs
├── render.rs                         Hàm dựng island mặc định
├── island/
│   ├── manager/                      Island, singleton và IslandCore
│   ├── controller/                   Tick, chọn view, keyboard, scroll
│   ├── models/                       Config, display state và ViewRecord
│   ├── ui/                           Builder, notch, popover, transition
│   └── models/view.rs                 View descriptor, handle và trait (được re-export)
├── features/
│   ├── clipboard/                    Clipboard listener và popover
│   ├── default/                      Idle logo
│   ├── media_player/                 playerctl, artwork, control popover
│   ├── power/                        Power action và popover
│   ├── recording/                    wf-recorder state và control
│   └── system/                       Battery, Bluetooth, brightness, network, volume
└── models.rs                          Notification model re-export
```

## Cụm runtime

| Thành phần | Vai trò |
| :--- | :--- |
| `Island` | Đối tượng public giữ widget, state và controller source. Có thể clone vì state dùng chung. |
| `IslandCore` | State nội bộ gồm các view đã đăng ký, view đang hiển thị, view chờ chuyển, hover và scroll selection. |
| `IslandBuilder` | Tạo `IslandConfig`, idle widget, descriptor view và trait feature trước khi build. |
| `IslandView` | Mô tả tĩnh cho một view: id, priority, kích thước, widget và callback. |
| `IslandViewHandle` | Handle nhẹ để gọi `show`, `hide`, override và thay nội dung view. |
| `IslandFeature` | Interface cho feature có state, service nền, lifecycle và widget riêng. |
| `IslandDisplay` | `Hidden`, `Idle` hoặc `View(index)`. |

## Cấu hình mặc định

```rust
use babydra_island::island::{IslandBuilder, IslandConfig};

let config = IslandConfig {
    idle_visible: false,
    poll_interval_ms: 150,
    expand_ms: 350,
    collapse_ms: 500,
};

let island = IslandBuilder::new().config(config).build();
```

Các hằng số kích thước trong `island::view`:

| Hằng số | Giá trị | Ý nghĩa |
| :--- | :--- | :--- |
| `CAPSULE_WIDTH` | `180` | Chiều rộng mặc định. |
| `PLAYER_CAPSULE_WIDTH` | `200` | Chiều rộng tham chiếu cho media player. |
| `CAPSULE_HEIGHT` | `30` | Chiều cao view hoạt động. |
| `IDLE_SIZE` | `(28, 16)` | Kích thước idle pill. |

## Island mặc định

`build_default_island()` đăng ký các feature sau:

| Feature | Nguồn trạng thái |
| :--- | :--- |
| `PowerFeature` | Power action và trạng thái nguồn. |
| `VolumeFeature` | Service volume. |
| `BrightnessFeature` | Service brightness. |
| `NetworkFeature` | Network state. |
| `BluetoothFeature` | Bluetooth state. |
| `BatteryFeature` | Battery state. |
| `ClipboardFeature` | Clipboard event và preview. |
| `RecordingFeature` | Recording state và action. |
| `MediaPlayerFeature` | playerctl, artwork và playback control. |
| Idle view | Logo hoặc nội dung tĩnh khi không có view active. |

`create_system_island()` trả về capsule widget để tương thích với code panel cũ. `build_default_island()` trả về `Island`, phù hợp khi caller cần giữ manager hoặc đăng ký thêm view.

## Hai cách đăng ký view

### Descriptor và handle

Dùng cho view đơn giản, state được quản lý bên ngoài feature:

```rust
use gtk4::prelude::*;
use babydra_island::island::IslandView;

let island = babydra_island::Island::builder()
    .view(
        IslandView::with_builder("sync", || {
            gtk4::Label::new(Some("Synchronizing")).upcast()
        })
        .priority(60)
        .size(180, 30)
        .hover_keep(true)
        .capsule_class("sync-mode")
        .on_show(|| {})
        .on_hide(|| {})
        .on_click(|| {}),
    )
    .build();

let handle = island.get_handle("sync").expect("view registered");
handle.show_for(std::time::Duration::from_secs(3));
```

Các method điều khiển chính:

| Method | Hành vi |
| :--- | :--- |
| `show()` | Gửi yêu cầu hiển thị theo priority. |
| `show_for(duration)` | Hiển thị rồi tự rút yêu cầu sau thời gian chỉ định. |
| `hide()` | Xóa yêu cầu, override và deadline. |
| `override_show()` | Ép view thắng arbitration cho tới khi release. |
| `override_show_for(duration)` | Ép hiển thị trong một khoảng thời gian rồi tự release. |
| `release_override()` | Kết thúc override và trả quyền chọn cho view khác. |
| `set_content(widget)` | Thay widget bên trong view. |
| `is_requested()` / `is_active()` | Đọc trạng thái request và trạng thái đang hiển thị. |

`show()` không tạo request sequence mới nếu view đã được request. Vì vậy gọi lặp trong poller không làm thay đổi thứ tự ưu tiên và không kéo dài deadline ngoài ý muốn.

### Trait `IslandFeature`

Dùng cho feature có service, state, lifecycle hoặc popover:

```rust
use babydra_island::island::{IslandCtx, IslandFeature, IslandViewHandle};

struct ExampleFeature {
    handle: Option<IslandViewHandle>,
}

impl IslandFeature for ExampleFeature {
    fn id(&self) -> &str { "example" }
    fn priority(&self) -> u8 { 50 }
    fn size(&self) -> (i32, i32) { (180, 30) }
    fn build_view(&mut self) -> gtk4::Widget { todo!() }

    fn init(&mut self, handle: &IslandViewHandle) {
        self.handle = Some(handle.clone());
    }

    fn tick(&mut self, ctx: &IslandCtx) {
        let _ = ctx.is_current();
        let _ = ctx.is_hovered();
    }
}

let island = babydra_island::Island::builder()
    .feature(Box::new(ExampleFeature { handle: None }))
    .build();
```

Các method lifecycle quan trọng:

- `build_view`: tạo widget một lần khi feature được đăng ký.
- `init`: nhận handle của chính view đó.
- `attach`: nhận capsule để gắn popover hoặc controller.
- `tick`: chạy trong mỗi chu kỳ controller trên GTK main thread.
- `on_show` và `on_hide`: bắt đầu hoặc dừng trạng thái hiển thị.
- `on_click`: xử lý click vào capsule.
- `is_alive`: giữ feature active khi không dùng timeout.
- `is_popover_open`: khóa view hiện tại khi người dùng đang tương tác với popover.
- `focus`: yêu cầu keyboard focus cho feature cần nhận phím.

## Vòng đời một tick

```text
glib timeout, mặc định 150 ms
  → purge timeout và view đã hết hạn
  → gọi tick() cho mọi feature
  → cập nhật bracket khi có nhiều view active
  → chọn winner
  → áp dụng transition nếu display state thay đổi
```

Transition ẩn view cũ trước, gọi `on_hide`, cập nhật CSS class, hiển thị view mới, gọi `on_show`, rồi chạy animation kích thước. Khi island chuyển về `Hidden` hoặc `Idle`, các popover đang mở được đóng.

## Thứ tự chọn view

`select_winner` xử lý theo thứ tự sau:

1. Feature có popover đang mở được giữ lại để event nền không thay thế nội dung người dùng đang xem.
2. View được người dùng chọn bằng scroll được giữ nếu chưa hết hạn và chưa có request mới hơn.
3. View có `override_active` thắng; nếu có nhiều view, request sequence mới hơn thắng.
4. Trong các view `requested`, priority cao hơn thắng.
5. Nếu priority bằng nhau, request sequence mới hơn thắng; sau cùng dùng thứ tự đăng ký.
6. Không có winner thì hiển thị idle nếu `idle_visible` bật, nếu không thì ẩn capsule.

## Thread và GTK

`IslandFeature::tick`, `on_show`, `on_hide` và thao tác widget chạy trên GTK main thread. Poller như playerctl, D-Bus hoặc file watcher phải đưa dữ liệu về main context trước khi sửa widget. Không gọi GTK từ thread nền và không block tick loop bằng I/O.

Khi panel rebuild island:

1. Gọi `dispose()` để remove GLib source.
2. Dừng hoặc dispose service của feature.
3. Tạo manager mới, đăng ký feature và controller mới.
4. Cập nhật widget panel bằng island mới.

## Kiểm thử feature

- Kiểm tra feature không có dữ liệu: không được giữ view active vô hạn.
- Kiểm tra `show_for` và `override_show_for` tự hết hạn.
- Kiểm tra popover mở thì request nền không đổi view.
- Kiểm tra click, scroll và keyboard focus không làm kẹt `animating`.
- Kiểm tra dispose không còn timer, channel hoặc callback giữ widget cũ.
