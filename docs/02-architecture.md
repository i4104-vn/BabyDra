# Chương 02: Kiến trúc Mã nguồn BabyDra

**Phiên bản:** 1.2.0
**Cập nhật lần cuối:** 2026-08-14
**Phạm vi:** 4 pattern thiết kế cốt lõi, luồng dữ liệu một chiều, mô hình Daemon-Client, quy trình khởi tạo cửa sổ

---

## Mục lục

- [1. Tổng quan kiến trúc](#1-tổng-quan-kiến-trúc)
- [2. Pattern 1: Phân tách Giao diện và Nghiệp vụ](#2-pattern-1-phân-tách-giao-diện-và-nghiệp-vụ)
- [3. Pattern 2: Hướng trạng thái và Luồng dữ liệu một chiều](#3-pattern-2-hướng-trạng-thái-và-luồng-dữ-liệu-một-chiều)
- [4. Pattern 3: Mô hình Daemon-Client](#4-pattern-3-mô-hình-daemon-client)
- [5. Pattern 4: Module hóa Giao diện](#5-pattern-4-module-hóa-giao-diện)
- [6. Quy trình khởi tạo cửa sổ chuẩn](#6-quy-trình-khởi-tạo-cửa-sổ-chuẩn)
- [7. Các dịch vụ nền của babydra-core](#7-các-dịch-vụ-nền-của-babydra-core)
- [8. Câu hỏi thường gặp](#8-câu-hỏi-thường-gặp)

---

## 1. Tổng quan kiến trúc

Toàn bộ mã nguồn BabyDra hoạt động theo luồng một chiều: thao tác người dùng kích hoạt cập nhật trạng thái, trạng thái kích hoạt vẽ lại giao diện.

```
Thao tác người dùng
        |
        v
Cập nhật State (Rc<RefCell<T>>)
        |
        v
Vẽ lại giao diện (queue_draw / rebuild widget)
        |
        v
Tầng View (Crates / GTK Window)
        |
        v
Tầng Engine (babydra-core)
        |
        v
Tương tác Hệ điều hành / D-Bus / sysfs
```

Kiến trúc này được xây dựng trên 4 pattern thiết kế cốt lõi, trình bày chi tiết ở các section bên dưới.

---

## 2. Pattern 1: Phân tách Giao diện và Nghiệp vụ

### 2.1. Vấn đề cần giải quyết

Nếu mã giao diện (GTK widget) và mã nghiệp vụ (đọc file hệ thống, tính toán) được trộn lẫn trong cùng một file, codebase sẽ trở nên khó test, khó tái sử dụng, và dễ tạo ra bug khi sửa đổi.

### 2.2. Giải pháp

BabyDra tách mã nguồn thành hai tầng rõ ràng:

**Tầng View (GUI Layer) — nằm trong `crates/` và `libs/` (widget)**

- Vai trò: Lớp hiển thị bên ngoài. Chỉ chịu trách nhiệm bắt sự kiện tương tác của người dùng và hiển thị thông tin lên widget GTK.
- Không được: Trực tiếp đọc file hệ thống, gọi lệnh terminal, tương tác D-Bus.
- Được phép: Gọi API từ `babydra-core` để lấy dữ liệu, sau đó hiển thị dữ liệu đó lên widget.

**Tầng Engine (Core Logic Layer) — nằm trong `libs/babydra-core/`**

- Vai trò: Toàn bộ nghiệp vụ tính toán và tương tác với hệ điều hành (đọc ghi file `/sys/class`, điều khiển phần cứng WiFi/Bluetooth, quản lý D-Bus daemon) đều được đóng gói thành các hàm API độc lập.
- Không biết gì về GTK, widget, hay giao diện.
- Được phép: Đọc ghi hệ thống, thực thi lệnh shell, lắng nghe D-Bus.

### 2.3. Ví dụ thực tế

Khi người dùng kéo thanh trượt âm lượng:

```
Tầng View (crates/babydra-panel/src/widgets/panel/items/volume/)
  Bắt sự kiện kéo thanh trượt
        |
        | Gọi: babydra_core::services::system::volume::set_volume(value)
        v
Tầng Engine (libs/babydra-core/src/services/system/volume/)
  Ghi giá trị mới vào PipeWire / WirePlumber API
```

Khi người dùng bật/tắt WiFi trong control center:

```
Tầng View (crates/babydra-panel/src/widgets/panel/popover/network.rs)
        |
        | Gọi: babydra_core::services::system::wifi::connect(ssid, password)
        v
Tầng Engine (libs/babydra-core/src/services/system/wifi/)
  Gọi NetworkManager qua D-Bus
```

### 2.4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Tầng View chỉ gọi hàm từ `babydra-core` để lấy/set dữ liệu |
| DO | Tầng Engine chỉ chứa logic thuần túy, không import GTK |
| DO NOT | Không viết `std::process::Command` hay `std::fs::read` trong file `render.rs` hoặc `mod.rs` của tầng View |
| DO NOT | Không import `gtk4` trong `babydra-core` |

> [!NOTE]
> Ngoại lệ duy nhất: `kits/babydra-ui-kit` chứa **widget GTK dùng chung** và được phép import GTK4, vì đây là tầng UI infrastructure, không phải nghiệp vụ.

---

## 3. Pattern 2: Hướng trạng thái và Luồng dữ liệu một chiều

### 3.1. Vấn đề cần giải quyết

Giao diện GTK có nhiều widget khác nhau (slider, label, button) cùng hiển thị một thông tin (ví dụ: mức âm lượng). Nếu mỗi widget tự lưu trữ dữ liệu riêng, chúng sẽ dễ mất đồng bộ với nhau.

### 3.2. Giải pháp

Mỗi cửa sổ phức tạp liên kết với một **cấu trúc trạng thái** (State struct) duy nhất, được chia sẻ qua con trỏ `Rc<RefCell<T>>`.

**Giải thích `Rc<RefCell<T>>`:**

- `Rc<T>` (Reference Counted): Cho phép nhiều nơi cùng giữ tham chiếu đến một giá trị mà không cần sao chép. Khi tất cả tham chiếu bị hủy, giá trị tự động được giải phóng.
- `RefCell<T>`: Cho phép thay đổi (mutate) giá trị bên trong ngay cả khi đang được giữ bởi nhiều tham chiếu, bằng cách kiểm tra quy tắc borrow ở runtime thay vì compile time.
- Kết hợp lại `Rc<RefCell<T>>`: Cho phép nhiều widget cùng đọc và ghi vào một State struct duy nhất một cách an toàn.

Ví dụ thực tế trong codebase:

- `babydra-panel`: `Rc<RefCell<Option<gtk4::ApplicationWindow>>>` cho `control_center_window`, `calendar_window`, `launcher_window` — đảm bảo chỉ một cửa sổ nổi mở tại một thời điểm.
- `babydra-explore`: `Rc<RefCell<SessionState>>` chứa toàn bộ trạng thái phiên làm việc (tabs, thư mục hiện tại, pane đang active).

### 3.3. Luồng dữ liệu một chiều

Luồng hoạt động cố định theo 2 bước:

**Bước 1: Thao tác kích hoạt cập nhật State**

Người dùng tương tác (nhấp chuột, gõ phím, kéo thanh trượt) → callback được gọi → giá trị bên trong State struct thay đổi.

```rust
// Ví dụ: khi kéo thanh trượt âm lượng
scale.connect_value_changed(clone!(@strong state => move |s| {
    state.borrow_mut().volume = s.value() as u8;
    // Bước tiếp theo: cập nhật giao diện
}));
```

**Bước 2: State thay đổi kích hoạt vẽ lại giao diện**

Sau khi State được cập nhật, phát tín hiệu yêu cầu vẽ lại (`queue_draw`) hoặc tái tạo cây widget (`rebuild`) để đồng bộ giao diện với trạng thái mới.

```rust
// Sau khi cập nhật state, yêu cầu vẽ lại label hiển thị phần trăm
volume_label.set_text(&format!("{}%", state.borrow().volume));
```

### 3.4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Sử dụng `Rc<RefCell<State>>` làm nguồn dữ liệu duy nhất cho mỗi cửa sổ phức tạp |
| DO | Luồng dữ liệu chỉ chạy một chiều: Người dùng → State → Giao diện |
| DO NOT | Không lưu dữ liệu kinh doanh (business data) trực tiếp trong widget (ví dụ: không dùng `button.set_data("volume", 50)` thay cho State) |
| DO NOT | Không để hai widget cùng ghi vào dữ liệu mà không thông qua State |

---

## 4. Pattern 3: Mô hình Daemon-Client

### 4.1. Vấn đề cần giải quyết

Khởi động lạnh một ứng dụng GTK (cold start) tốn hàng trăm mili-giây (thường 200ms–500ms) do phải nạp thư viện, khởi tạo GDK context, và dựng cây widget từ đầu. Điều này khiến trải nghiệm nhấn phím tắt để mở panel bị giật lag.

### 4.2. Giải pháp

BabyDra áp dụng mô hình **Daemon-Client** để loại bỏ hoàn toàn độ trễ:

```
+------------------+          +----------------------------+
| Client (siêu nhẹ)|          | Daemon (chạy ngầm 24/7)   |
|                  |          |                            |
| Được gọi khi     | Socket / | Giữ cửa sổ ẩn sẵn trong   |
| người dùng nhấn  | D-Bus    | bộ nhớ (set_visible false) |
| phím tắt         |--------> |                            |
|                  |          | Nhận tín hiệu → gọi        |
| Chỉ gửi 1 tín    |          | window.set_visible(true)   |
| hiệu, sau đó     |          | + window.present()         |
| thoát ngay       |          |                            |
+------------------+          +----------------------------+
                                           |
                               Cửa sổ hiện ra < 10ms
```

**Ví dụ triển khai thực tế — `babydra-switcher`:**

```bash
# Daemon chạy từ autostart của labwc (xem configs/labwc/autostart)
babydra-switcher --daemon &
```

- Daemon giữ overlay (danh sách cửa sổ + preview) dựng sẵn trong bộ nhớ và lắng nghe Unix socket `/tmp/babydra-switcher.socket`.
- Khi người dùng nhấn `Alt+Tab` (keybind trong `rc.xml`), labwc gọi binary `babydra-switcher` (chế độ one-shot client). Client chỉ làm một việc: gửi tín hiệu `show`/`next` vào socket rồi thoát ngay.
- Daemon nhận tín hiệu và hiện overlay ngay lập tức — không có cold start.

**Kết quả:** Tốc độ hiển thị từ nhấn phím đến cửa sổ xuất hiện dưới 10ms.

### 4.3. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Mỗi crate ứng dụng phải chạy ở chế độ Daemon: khởi động xong thì ẩn cửa sổ và lắng nghe socket |
| DO | Client phải cực nhỏ: chỉ gửi tín hiệu socket rồi thoát, không làm gì khác |
| DO NOT | Không để người dùng nhấn phím tắt mà phải khởi động toàn bộ ứng dụng từ đầu |

---

## 5. Pattern 4: Module hóa Giao diện

### 5.1. Vấn đề cần giải quyết

Nếu mỗi crate ứng dụng tự định nghĩa riêng CSS và widget cơ bản, sẽ xuất hiện:

- Không nhất quán về màu sắc, kích thước, font chữ giữa các ứng dụng.
- Trùng lặp mã CSS: mỗi ứng dụng copy-paste cùng một đoạn CSS.
- Khó thay đổi theme: cần sửa ở nhiều nơi cùng lúc.

### 5.2. Giải pháp

Toàn bộ CSS và widget dùng chung được tập trung tại thư viện `babydra-ui-kit`:

```
kits/babydra-ui-kit/
    src/
        styles/
            shared/        <- CSS cấu trúc & layout dùng chung (không phụ thuộc theme)
                panel/     <- Panel: panel, taskbar, clock, status, sys_monitor, tray, workspaces
                control_center/
                island/    <- system_island, notification
                launcher/
                calendar/
                apps/      <- lock, preview, screenshot, settings, switcher
                explore/   <- window, header_bar, content_view, info_panel, status_bar, context_menu, dialogs
                shared/    <- button, switch, sidebar, scrollbar
            dark/          <- Màu sắc chế độ tối (cùng cây thư mục như shared/)
            light/         <- Màu sắc chế độ sáng (cùng cây thư mục như shared/)
        components/        <- Widget GTK dùng chung (button, card, modal, switch, navbar, slider...)
        explore/           <- Context menu & dialogs dùng riêng cho babydra-explore
        ui/
            theme/         <- Module khởi tạo và nạp theme
            icon/          <- Icon resolver & assets
            animation/     <- easing, genie, island, slide
            battery.rs     <- Helper đọc pin
            window.rs      <- Helper cửa sổ
```

**Cơ chế nạp CSS:**

Khi bất kỳ ứng dụng nào khởi động, nó gọi hàm `babydra_ui_kit::ui::theme::init_theme()` (không tham số). Hàm này:

1. Gộp toàn bộ nội dung CSS `shared/` (cấu trúc) với CSS `dark/` hoặc `light/` (màu sắc) tùy theo chế độ hiện tại.
2. Nạp CSS vào `GtkCssProvider` toàn cục của GDK Display Context.
3. Đăng ký lắng nghe sự kiện thay đổi `color-scheme` của GSettings — khi người dùng chuyển Dark ↔ Light, gọi lại `init_theme()` để áp dụng ngay mà không cần khởi động lại ứng dụng.

Nhờ đó, mọi widget trên mọi ứng dụng đều tự động nhận style đúng mà không cần mỗi ứng dụng tự quản lý CSS.

### 5.3. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Toàn bộ CSS phải đặt trong `kits/babydra-ui-kit/src/styles/` |
| DO | Gọi `babydra_ui_kit::ui::theme::init_theme()` trong hàm `main()` hoặc `activate()` của mỗi ứng dụng |
| DO NOT | Không viết CSS inline trong mã Rust (không dùng `css_classes`, `widget.set_css_classes()` với giá trị style cụ thể) |
| DO NOT | Không tạo `GtkCssProvider` mới trong từng ứng dụng. Chỉ dùng provider toàn cục do `babydra-ui-kit` quản lý |

---

## 6. Quy trình khởi tạo cửa sổ chuẩn

Mỗi cửa sổ GTK trong dự án phải được khởi tạo theo đúng 3 bước sau:

### Bước 1: Cấu hình Layer Shell

BabyDra chạy trên Wayland và dùng `gtk4-layer-shell` để định vị cửa sổ chính xác trên màn hình mà không cần window manager can thiệp.

**Layer (Tầng lớp hiển thị):** Quyết định cửa sổ nằm ở vị trí nào trong không gian Z (độ sâu hiển thị).

| Layer | Dùng cho | Ví dụ |
| :--- | :--- | :--- |
| `Layer::Top` | Giao diện hệ thống luôn nổi trên ứng dụng thông thường | Panel, thanh trạng thái |
| `Layer::Overlay` | Giao diện cần bao phủ toàn màn hình | Alt-Tab Switcher, Lock Screen, Screenshot |

**Edge (Neo cạnh màn hình):** Gắn cửa sổ vào một cạnh màn hình cụ thể để cố định vị trí.

**Exclusive Zone (Vùng loại trừ):** Chỉ định khoảng không gian trên màn hình mà các ứng dụng khác không được phóng to đè lên. Ví dụ: Panel cao 48px thì đặt `exclusive_zone = 48` để các ứng dụng khác tự động bắt đầu hiển thị từ pixel 49 trở đi, không bị che khuất.

### Bước 2: Nạp Stylesheet toàn cục

```rust
// Trong hàm activate() của mỗi ứng dụng
babydra_ui_kit::ui::theme::init_theme();
```

Hàm `init_theme()` sẽ:

1. Gộp CSS `shared/` + CSS `dark/` hoặc `light/` theo chế độ màu hiện tại của GSettings (`org.gnome.desktop.interface color-scheme`).
2. Nạp toàn bộ CSS vào provider toàn cục của GDK Display.
3. Lắng nghe sự kiện thay đổi theme (khi người dùng chuyển Dark ↔ Light trong lúc chạy) và nạp lại tự động.

### Bước 3: Khởi động các dịch vụ chạy ngầm

Tùy theo ứng dụng, đây có thể bao gồm:

- Khởi động D-Bus StatusNotifierWatcher (system tray) — `babydra_core::tray::spawn_watcher_service()`.
- Khởi động thread theo dõi cửa sổ đang ở tiêu điểm (focus) — `babydra_core::spawn_switcher_tracker()`.
- Refresh cache danh sách ứng dụng desktop bất đồng bộ — `babydra_core::refresh_desktop_apps_cache()`.
- Phát hiện bus DDC/CI cho màn hình ngoài — `widgets::panel::detect_ddc_bus()`.

---

## 7. Các dịch vụ nền của babydra-core

`libs/babydra-core/src/services/` chứa toàn bộ nghiệp vụ hệ thống, được tái export phẳng qua `lib.rs`:

| Nhóm service | Module | Chức năng |
| :--- | :--- | :--- |
| Hệ thống | `system/` | battery, backlight (DDC/CI + sysfs), bluetooth, certificates, clean (cache/logs/pacman/trash), display, gpu, monitor, network, power (profile + saver), startup, storage, theme, updates, volume (PipeWire), vpn (NetworkManager), wifi (NetworkManager) |
| Cửa sổ | `window/` | `tracker` (theo dõi cửa sổ focus cho switcher), `mru` (Most Recently Used — lịch sử cửa sổ) |
| Thông báo | `notification/` | `service` (gửi thông báo), `island` (trạng thái island) |
| Tray | `tray/` | `client`, `dbusmenu`, `watcher` (StatusNotifierWatcher) |
| Ứng dụng | `apps/` | `discovery` (quét .desktop files), `pacman` |
| Đa phương tiện | `mpris/` | Điều khiển media player qua MPRIS D-Bus |
| Khác | `actions`, `clock`, `exif`, `explore/` (cmd, dbus, dir_size, filter, fs_ops, preview, sort, watcher), `logger`, `screenshot`, `search`, `wallpaper`, `utils` | — |

Các helper quan trọng tái export tại gốc (`babydra_core::`):

- `init_logger(app_name, log_file)` — logger chia sẻ, log vào `~/.cache/babydra/`.
- `capture_screen_to_temp()`, `handle_fullscreen_capture()`, `trigger_save()`, `trigger_copy()` — chụp màn hình (grim/slurp + wl-clipboard).
- `verify_password()`, `poweroff()`, `reboot()`, `suspend()`, `set_performance_profile()` — quyền & nguồn điện (PAM).
- `send_notification(...)` — gửi thông báo desktop.
- `set_wallpaper()`, `set_greeter_wallpaper()`, `set_avatar()` — hình nền & avatar.
- `i18n::t("settings.notif_auto_saver_title")` — dịch chuỗi theo ngôn ngữ hiện tại (en/vi).

**Đa ngôn ngữ (i18n):**

- File JSON: `libs/babydra-core/src/i18n/locales/<app>/en.json` và `vi.json` (các app: `common`, `explore`, `greeter`, `launcher`, `settings`).
- Mọi chuỗi hiển thị trong UI phải đi qua hàm `babydra_core::i18n::t("namespace.key")` thay vì hardcode.

---

## 8. Câu hỏi thường gặp

**Hỏi: Tại sao không dùng async/await thay vì Daemon?**

Trả lời: Async/await giải quyết vấn đề chờ đợi I/O mà không block thread, nhưng không giải quyết được vấn đề thời gian khởi động GTK. Dù dùng async, GTK vẫn phải dựng widget từ đầu, vẫn tốn 200ms+. Daemon giữ widget đã dựng sẵn trong bộ nhớ, đây là cách duy nhất để đạt dưới 10ms.

**Hỏi: `Rc<RefCell<T>>` có an toàn không? Có thể gây panic không?**

Trả lời: `RefCell` kiểm tra borrow rule ở runtime. Nếu code cố gắng mượn (borrow) cùng lúc nhiều lần không hợp lệ (ví dụ: `borrow_mut()` trong khi đang `borrow()`), chương trình sẽ panic. Để tránh: không bao giờ gọi `borrow()` hay `borrow_mut()` bên trong một closure đang giữ borrow khác của cùng RefCell.

**Hỏi: Khi nào cần dùng `queue_draw()` và khi nào cần `rebuild` toàn bộ widget?**

Trả lời: Dùng `queue_draw()` khi chỉ cần vẽ lại nội dung (ví dụ: cập nhật số liệu trên label, thay đổi màu). Dùng `rebuild` (tạo lại cây widget) khi cấu trúc giao diện thay đổi (ví dụ: thêm/xóa một dòng trong danh sách).

**Hỏi: `babydra-explore` dùng tokio — có phá vỡ pattern không?**

Trả lời: Không. `babydra-explore` dùng `tokio::runtime` để thực hiện các thao tác I/O nặng (đọc thư mục lớn, tính kích thước thư mục) không block UI thread. State vẫn là `Rc<RefCell<SessionState>>`, luồng dữ liệu vẫn một chiều — async chỉ là công cụ chạy nền, không thay đổi kiến trúc.
