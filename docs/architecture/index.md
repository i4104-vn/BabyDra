# Kiến trúc mã nguồn BabyDra

**Phạm vi:** 4 pattern thiết kế cốt lõi, luồng dữ liệu một chiều, mô hình Daemon-Client, quy trình khởi tạo cửa sổ, dịch vụ nền của babydra-core.
**Phiên bản:** 1.3.0
**Cập nhật lần cuối:** 2026-08-17

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

Toàn bộ mã nguồn hoạt động theo luồng một chiều: thao tác người dùng → cập nhật trạng thái → vẽ lại giao diện.

```text
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

Kiến trúc được xây dựng trên 4 pattern thiết kế cốt lõi, trình bày ở các mục dưới.

---

## 2. Pattern 1: Phân tách Giao diện và Nghiệp vụ

### 2.1. Vấn đề cần giải quyết

Nếu mã giao diện (GTK widget) và mã nghiệp vụ (đọc file hệ thống, tính toán) trộn lẫn trong cùng một file, codebase khó test, khó tái sử dụng, dễ sinh bug khi sửa đổi.

### 2.2. Giải pháp

**Tầng View (GUI Layer)** — trong `crates/` và `libs/` (widget):

- Chỉ bắt sự kiện tương tác và hiển thị thông tin lên widget GTK.
- **Không được:** đọc file hệ thống, gọi lệnh terminal, tương tác D-Bus trực tiếp.
- **Được phép:** gọi API từ `babydra-core` để lấy dữ liệu rồi hiển thị.

**Tầng Engine (Core Logic Layer)** — trong `libs/babydra-core/`:

- Toàn bộ nghiệp vụ + tương tác hệ điều hành (đọc ghi `/sys/class`, điều khiển WiFi/Bluetooth, D-Bus daemon) đóng gói thành API độc lập.
- Không biết gì về GTK, widget, giao diện.

### 2.3. Ví dụ thực tế

Kéo thanh trượt âm lượng:

```text
Tầng View (crates/babydra-panel/src/widgets/panel/items/volume/)
  Bắt sự kiện kéo thanh trượt
        |
        | Gọi: babydra_core::services::system::volume::set_volume(value)
        v
Tầng Engine (libs/babydra-core/src/services/system/volume/)
  Ghi giá trị mới vào PipeWire / WirePlumber API
```

### 2.4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Tầng View chỉ gọi hàm từ `babydra-core` để lấy/set dữ liệu |
| DO | Tầng Engine chỉ chứa logic thuần túy, không import GTK |
| DO NOT | Không viết `std::process::Command` hay `std::fs::read` trong file `render.rs`/`mod.rs` của tầng View |
| DO NOT | Không import `gtk4` trong `babydra-core` |

> [!NOTE]
> Ngoại lệ duy nhất: `libs/babydra-ui-kit` chứa **widget GTK dùng chung** và được phép import GTK4 — đây là tầng UI infrastructure, không phải nghiệp vụ.

---

## 3. Pattern 2: Hướng trạng thái và Luồng dữ liệu một chiều

### 3.1. Vấn đề cần giải quyết

Giao diện GTK có nhiều widget (slider, label, button) cùng hiển thị một thông tin. Nếu mỗi widget tự lưu dữ liệu riêng, chúng dễ mất đồng bộ.

### 3.2. Giải pháp

Mỗi cửa sổ phức tạp liên kết với một **State struct** duy nhất, chia sẻ qua `Rc<RefCell<T>>`.

**Giải thích `Rc<RefCell<T>>`:**

- `Rc<T>` (Reference Counted): nhiều nơi cùng giữ tham chiếu mà không cần sao chép; khi hết tham chiếu, giá trị tự giải phóng.
- `RefCell<T>`: mutate giá trị ngay cả khi đang được giữ bởi nhiều tham chiếu — kiểm tra borrow rule ở runtime.
- Kết hợp `Rc<RefCell<T>>`: nhiều widget cùng đọc/ghi một State duy nhất một cách an toàn.

Ví dụ trong codebase:

- `babydra-panel`: `Rc<RefCell<Option<gtk4::ApplicationWindow>>>` cho `control_center_window`, `calendar_window`, `launcher_window` — chỉ một cửa sổ nổi mở tại một thời điểm.
- `babydra-explore`: `Rc<RefCell<SessionState>>` chứa toàn bộ trạng thái phiên làm việc.

### 3.3. Luồng dữ liệu một chiều

**Bước 1 — Thao tác kích hoạt cập nhật State:**

```rust
scale.connect_value_changed(clone!(@strong state => move |s| {
    state.borrow_mut().volume = s.value() as u8;
}));
```

**Bước 2 — State thay đổi kích hoạt vẽ lại giao diện:**

```rust
volume_label.set_text(&format!("{}%", state.borrow().volume));
```

### 3.4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Dùng `Rc<RefCell<State>>` làm nguồn dữ liệu duy nhất cho mỗi cửa sổ phức tạp |
| DO | Luồng dữ liệu chạy một chiều: Người dùng → State → Giao diện |
| DO NOT | Không lưu business data trực tiếp trong widget |
| DO NOT | Không để hai widget cùng ghi dữ liệu mà không qua State |

---

## 4. Pattern 3: Mô hình Daemon-Client

### 4.1. Vấn đề cần giải quyết

Cold start một ứng dụng GTK tốn 200–500ms (nạp thư viện, khởi tạo GDK, dựng cây widget). Nhấn phím tắt mở panel sẽ bị giật lag.

### 4.2. Giải pháp

Mô hình **Daemon-Client** loại bỏ hoàn toàn độ trễ:

```text
+------------------+          +----------------------------+
| Client (siêu nhẹ)|          | Daemon (chạy ngầm 24/7)   |
|                  |          |                            |
| Được gọi khi     | Socket / | Giữ cửa sổ ẩn sẵn trong   |
| người dùng nhấn  | D-Bus    | bộ nhớ (set_visible false) |
| phím tắt         |--------> |                            |
|                  |          | Nhận tín hiệu → gọi        |
| Chỉ gửi 1 tín    |          | window.set_visible(true)   |
| hiệu rồi thoát   |          | + window.present()         |
+------------------+          +----------------------------+
                                           |
                               Cửa sổ hiện ra < 10ms
```

**Ví dụ thực tế — `babydra-switcher`:**

- Daemon giữ overlay (danh sách cửa sổ + preview) dựng sẵn, lắng nghe Unix socket `/tmp/babydra-switcher.socket`.
- Nhấn `Alt+Tab` (keybind trong `rc.xml`) gọi binary `babydra-switcher` (one-shot client) — chỉ gửi tín hiệu `show`/`next` rồi thoát.
- Daemon nhận tín hiệu và hiện overlay ngay — không có cold start.

### 4.3. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Mỗi crate ứng dụng chạy ở chế độ Daemon: khởi động xong ẩn cửa sổ và lắng nghe socket |
| DO | Client cực nhỏ: chỉ gửi tín hiệu rồi thoát |
| DO NOT | Để phím tắt phải khởi động lại toàn bộ ứng dụng từ đầu |

---

## 5. Pattern 4: Module hóa Giao diện

### 5.1. Vấn đề cần giải quyết

Mỗi crate tự định nghĩa CSS/widget riêng → không nhất quán màu sắc, trùng lặp CSS, khó đổi theme.

### 5.2. Giải pháp

Toàn bộ CSS và widget dùng chung tập trung tại `babydra-ui-kit`:

```text
libs/babydra-ui-kit/
    src/
        styles/
            shared/        <- CSS cấu trúc & layout dùng chung (không phụ thuộc theme)
                panel/     <- panel, taskbar, clock, status, sys_monitor, tray, workspaces
                control_center/
                island/    <- system_island, notification
                launcher/
                calendar/
                apps/      <- lock, preview, screenshot, settings, switcher
                explore/   <- window, header_bar, content_view, info_panel, status_bar, context_menu, dialogs
                shared/    <- button, switch, sidebar, scrollbar
            dark/          <- (đã chuyển sang theme packages trong themes/<id>/css/)
            light/
        components/        <- Widget GTK dùng chung (button, card, modal, switch, navbar, slider...)
        components/explore/<- Feature components cho babydra-explore
        ui/
            theme/         <- init_theme(): nạp CSS + theme package
            icon/          <- Icon resolver & assets
            animation/     <- easing, genie, island, slide
            battery.rs     <- Helper đọc pin
            window.rs      <- Helper cửa sổ
```

**Cơ chế nạp CSS:**

Mọi ứng dụng gọi `babydra_ui_kit::ui::theme::init_theme()` lúc khởi động. Hàm này:

1. Đọc theme package đang chọn (`~/.babydra/babydra.conf` → `theme.selection`).
2. Gộp CSS `shared/` (cấu trúc) với lớp màu dark/light từ theme package.
3. Nạp vào `GtkCssProvider` toàn cục; lắng nghe `color-scheme` của GSettings — đổi Dark ↔ Light áp dụng ngay không cần khởi động lại.

### 5.3. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | CSS cấu trúc đặt trong `babydra-ui-kit/src/styles/shared/`; lớp màu thuộc theme packages (`themes/<id>/css/`) |
| DO | Gọi `init_theme()` trong `main()`/`activate()` của mỗi ứng dụng |
| DO NOT | Không viết CSS inline trong mã Rust |
| DO NOT | Không tự tạo `GtkCssProvider` riêng trong từng ứng dụng |

---

## 6. Quy trình khởi tạo cửa sổ chuẩn

### Bước 1: Cấu hình Layer Shell

Dùng `gtk4-layer-shell` để định vị cửa sổ chính xác trên màn hình:

| Layer | Dùng cho | Ví dụ |
| :--- | :--- | :--- |
| `Layer::Top` | Giao diện hệ thống luôn nổi | Panel, thanh trạng thái |
| `Layer::Overlay` | Giao diện bao phủ toàn màn hình | Alt-Tab Switcher, Lock Screen, Screenshot |

- **Edge:** gắn cửa sổ vào cạnh màn hình cụ thể.
- **Exclusive Zone:** vùng loại trừ để app khác không phóng to đè lên (panel 48px → `exclusive_zone = 48`).

### Bước 2: Nạp Stylesheet toàn cục

```rust
babydra_ui_kit::ui::theme::init_theme();
```

### Bước 3: Khởi động các dịch vụ chạy ngầm

Tùy theo ứng dụng: D-Bus StatusNotifierWatcher (`babydra_core::tray::spawn_watcher_service()`), thread theo dõi window focus (`spawn_switcher_tracker()`), refresh cache apps (`refresh_desktop_apps_cache()`), detect DDC bus.

---

## 7. Các dịch vụ nền của babydra-core

`libs/babydra-core/src/services/` chứa toàn bộ nghiệp vụ hệ thống, re-export phẳng qua `lib.rs`:

| Nhóm service | Module | Chức năng |
| :--- | :--- | :--- |
| Hệ thống | `system/` | battery, backlight (DDC/CI + sysfs), bluetooth, certificates, clean, display, gpu, monitor, network, power (profile + saver), startup, storage, theme, updates, volume (PipeWire), vpn, wifi (NetworkManager) |
| Cửa sổ | `window/` | `tracker` (window focus cho switcher), `mru` (lịch sử cửa sổ) |
| Thông báo | `notification/` | `service` (gửi thông báo), `island` (trạng thái island) |
| Tray | `tray/` | `client`, `dbusmenu`, `watcher` (StatusNotifierWatcher) |
| Ứng dụng | `apps/` | `discovery` (quét .desktop files), `pacman` |
| Đa phương tiện | `mpris/` | Điều khiển media player qua MPRIS D-Bus |
| Khác | `actions`, `clock`, `exif`, `explore/`, `logger`, `screenshot`, `search`, `wallpaper`, `utils` | — |

Các helper quan trọng re-export tại gốc (`babydra_core::`):

- `init_logger(app_name, log_file)` — log vào `~/.cache/babydra/`.
- `capture_screen_to_temp()`, `handle_fullscreen_capture()`, `trigger_save()`, `trigger_copy()` — chụp màn hình.
- `verify_password()`, `poweroff()`, `reboot()`, `suspend()`, `set_performance_profile()` — quyền & nguồn điện (PAM).
- `send_notification(...)` — gửi thông báo desktop.
- `set_wallpaper()`, `set_greeter_wallpaper()`, `set_avatar()` — hình nền & avatar.
- `i18n::t("settings.notif_auto_saver_title")` — dịch chuỗi theo locale (en/vi).

**Đa ngôn ngữ (i18n):** file JSON tại `libs/babydra-core/src/i18n/locales/<app>/{en,vi}.json`. Mọi chuỗi UI phải đi qua `babydra_core::i18n::t("namespace.key")`.

---

## 8. Câu hỏi thường gặp

**Hỏi: Tại sao không dùng async/await thay vì Daemon?**

Trả lời: Async giải quyết chờ đợi I/O, nhưng không giải quyết thời gian khởi động GTK — vẫn phải dựng widget từ đầu, vẫn tốn 200ms+. Daemon giữ widget dựng sẵn trong bộ nhớ — cách duy nhất đạt dưới 10ms.

**Hỏi: `Rc<RefCell<T>>` có an toàn không? Có gây panic không?**

Trả lời: `RefCell` kiểm tra borrow rule ở runtime — `borrow_mut()` trong khi đang `borrow()` sẽ panic. Tránh bằng cách: không bao giờ gọi `borrow()`/`borrow_mut()` bên trong closure đang giữ borrow khác của cùng RefCell.

**Hỏi: Khi nào dùng `queue_draw()` và khi nào `rebuild`?**

Trả lời: `queue_draw()` khi chỉ cần vẽ lại nội dung (cập nhật số trên label). `rebuild` khi cấu trúc giao diện thay đổi (thêm/xóa dòng trong danh sách).

**Hỏi: `babydra-explore` dùng tokio — có phá vỡ pattern không?**

Trả lời: Không. Tokio chỉ chạy thao tác I/O nặng (đọc thư mục lớn, tính kích thước) ở nền; State vẫn là `Rc<RefCell<SessionState>>`, luồng dữ liệu vẫn một chiều.

**Hỏi: Muốn thêm một widget/overlay mới cho Dynamic Island thì làm sao?**

Trả lời: Không cần sửa island core — đăng ký `IslandView` hoặc implement `IslandFeature` rồi `register_feature` vào `default_island()`. Xem hướng dẫn đầy đủ: [guides/island](../guides/island.md) và [guides/island-features](../guides/island-features.md).
