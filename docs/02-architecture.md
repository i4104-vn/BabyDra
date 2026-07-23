# Chương 02: Kiến trúc Mã nguồn BabyDra

**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-07-23
**Phạm vi:** 4 pattern thiết kế cốt lõi, luồng dữ liệu một chiều, mô hình Daemon-Client, quy trình khởi tạo cửa sổ

---

## Mục lục

- [1. Tổng quan kiến trúc](#1-tổng-quan-kiến-trúc)
- [2. Pattern 1: Phân tách Giao diện và Nghiệp vụ](#2-pattern-1-phân-tách-giao-diện-và-nghiệp-vụ)
- [3. Pattern 2: Hướng trạng thái và Luồng dữ liệu một chiều](#3-pattern-2-hướng-trạng-thái-và-luồng-dữ-liệu-một-chiều)
- [4. Pattern 3: Mô hình Daemon-Client](#4-pattern-3-mô-hình-daemon-client)
- [5. Pattern 4: Module hóa Giao diện](#5-pattern-4-module-hóa-giao-diện)
- [6. Quy trình khởi tạo cửa sổ chuẩn](#6-quy-trình-khởi-tạo-cửa-sổ-chuẩn)
- [7. Câu hỏi thường gặp](#7-câu-hỏi-thường-gặp)

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
Tầng Engine (babydra-common)
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

**Tầng View (GUI Layer) — nằm trong `crates/`**

- Vai trò: Lớp hiển thị bên ngoài. Chỉ chịu trách nhiệm bắt sự kiện tương tác của người dùng và hiển thị thông tin lên widget GTK.
- Không được: Trực tiếp đọc file hệ thống, gọi lệnh terminal, tương tác D-Bus.
- Được phép: Gọi API từ `babydra-common` để lấy dữ liệu, sau đó hiển thị dữ liệu đó lên widget.

**Tầng Engine (Core Logic Layer) — nằm trong `libs/babydra-common/`**

- Vai trò: Toàn bộ nghiệp vụ tính toán và tương tác với hệ điều hành (đọc ghi file `/sys/class`, điều khiển phần cứng WiFi/Bluetooth, quản lý D-Bus daemon) đều được đóng gói thành các hàm API độc lập.
- Không biết gì về GTK, widget, hay giao diện.
- Được phép: Đọc ghi hệ thống, thực thi lệnh shell, lắng nghe D-Bus.

### 2.3. Ví dụ thực tế

Khi người dùng kéo thanh trượt âm lượng:

```
Tầng View (babydra-panel/src/widgets/panel/items/volume/mod.rs)
  Bắt sự kiện kéo thanh trượt
        |
        | Gọi: babydra_common::services::system::volume::set_volume(value)
        v
Tầng Engine (babydra-common/src/services/system/volume/)
  Ghi giá trị mới vào /sys/class/sound/... hoặc gọi PipeWire API
```

### 2.4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Tầng View chỉ gọi hàm từ `babydra-common` để lấy/set dữ liệu |
| DO | Tầng Engine chỉ chứa logic thuần túy, không import GTK |
| DO NOT | Không viết `std::process::Command` hay `std::fs::read` trong file `render.rs` hoặc `mod.rs` của tầng View |
| DO NOT | Không import `gtk4` trong `babydra-common` |

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

**Chi tiết cơ chế:**

1. Khi hệ thống khởi động, mỗi Daemon được khởi động một lần. Nó dựng toàn bộ cửa sổ GTK nhưng đặt `set_visible(false)`.
2. Daemon lắng nghe liên tục trên Unix Domain Socket hoặc D-Bus.
3. Khi người dùng nhấn phím tắt (ví dụ: Super+Space để mở Launcher), một tiến trình client cực kỳ nhỏ được gọi. Client chỉ làm một việc: gửi tín hiệu vào socket và thoát.
4. Daemon nhận tín hiệu, gọi ngay `window.set_visible(true)` và `window.present()`. Cửa sổ xuất hiện lập tức vì nó đã được dựng sẵn trong bộ nhớ.

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

Toàn bộ CSS và widget dùng chung được tập trung tại thư viện `babydra-utils`:

```
libs/babydra-utils/
    src/
        styles/
            dark/        <- CSS cho chế độ tối
                shared/  <- CSS dùng chung (button, switch, scrollbar, sidebar)
                panel/   <- CSS riêng cho babydra-panel
                ...
            light/       <- CSS cho chế độ sáng
                shared/
                panel/
                ...
        components/      <- Widget GTK dùng chung
        ui/
            theme/       <- Module khởi tạo và nạp theme
```

**Cơ chế nạp CSS:**

Khi bất kỳ ứng dụng nào khởi động, nó gọi hàm `init_theme()` từ `babydra-utils`. Hàm này:

1. Đọc GSettings để biết hệ thống đang dùng Dark hay Light mode.
2. Gộp toàn bộ nội dung CSS từ thư mục theme tương ứng.
3. Nạp CSS vào `GtkCssProvider` toàn cục của GDK Display Context.

Nhờ đó, mọi widget trên mọi ứng dụng đều tự động nhận style đúng mà không cần mỗi ứng dụng tự quản lý CSS.

### 5.3. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Toàn bộ CSS phải đặt trong `libs/babydra-utils/src/styles/` |
| DO | Gọi `init_theme()` từ `babydra-utils` trong hàm `main()` hoặc `activate()` của mỗi ứng dụng |
| DO NOT | Không viết CSS inline trong mã Rust (không dùng `css_classes`, `widget.set_css_classes()` với giá trị style cụ thể) |
| DO NOT | Không tạo `GtkCssProvider` mới trong từng ứng dụng. Chỉ dùng provider toàn cục do `babydra-utils` quản lý |

---

## 6. Quy trình khởi tạo cửa sổ chuẩn

Mỗi cửa sổ GTK trong dự án phải được khởi tạo theo đúng 3 bước sau:

### Bước 1: Cấu hình Layer Shell

BabyDra chạy trên Wayland và dùng `gtk4-layer-shell` để định vị cửa sổ chính xác trên màn hình mà không cần trình quản lý cửa sổ (window manager) can thiệp.

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
babydra_utils::ui::theme::init_theme(&display);
```

Hàm `init_theme()` sẽ:

1. Đọc biến `gtk-application-prefer-dark-theme` từ GSettings.
2. Lắng nghe sự kiện thay đổi theme (khi người dùng chuyển Dark ↔ Light trong lúc chạy).
3. Nạp toàn bộ CSS tương ứng vào provider toàn cục của GDK Display.

### Bước 3: Khởi động các dịch vụ chạy ngầm

Tùy theo ứng dụng, đây có thể bao gồm:

- Khởi động luồng (thread) chạy ngầm theo dõi ứng dụng đang ở tiêu điểm (focus) — dùng cho Panel Dock.
- Khởi động D-Bus notification server — dùng cho Notification Center.
- Đăng ký bộ lắng nghe sự kiện đóng/mở cửa sổ — dùng để kích hoạt hiệu ứng Genie Animation khi ẩn/hiện cửa sổ.

---

## 7. Câu hỏi thường gặp

**Hỏi: Tại sao không dùng async/await thay vì Daemon?**

Trả lời: Async/await giải quyết vấn đề chờ đợi I/O mà không block thread, nhưng không giải quyết được vấn đề thời gian khởi động GTK. Dù dùng async, GTK vẫn phải dựng widget từ đầu, vẫn tốn 200ms+. Daemon giữ widget đã dựng sẵn trong bộ nhớ, đây là cách duy nhất để đạt dưới 10ms.

**Hỏi: `Rc<RefCell<T>>` có an toàn không? Có thể gây panic không?**

Trả lời: `RefCell` kiểm tra borrow rule ở runtime. Nếu code cố gắng mượn (borrow) cùng lúc nhiều lần không hợp lệ (ví dụ: `borrow_mut()` trong khi đang `borrow()`), chương trình sẽ panic. Để tránh: không bao giờ gọi `borrow()` hay `borrow_mut()` bên trong một closure đang giữ borrow khác của cùng RefCell.

**Hỏi: Khi nào cần dùng `queue_draw()` và khi nào cần `rebuild` toàn bộ widget?**

Trả lời: Dùng `queue_draw()` khi chỉ cần vẽ lại nội dung (ví dụ: cập nhật số liệu trên label, thay đổi màu). Dùng `rebuild` (tạo lại cây widget) khi cấu trúc giao diện thay đổi (ví dụ: thêm/xóa một dòng trong danh sách).
