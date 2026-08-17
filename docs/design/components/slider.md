# Component: Slider

Tài liệu quy định cách sử dụng, kiểu dáng, và mã nguồn chuẩn cho **Slider** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `libs/babydra-ui-kit/src/components/slider/mod.rs`

---

## 1. Tổng quan

**CustomSlider** — slider tùy biến vẽ bằng Cairo, thay thế `gtk4::Scale` mặc định. Hỗ trợ:

- Dải giá trị tùy chỉnh (`min`, `max`) và bước nhảy (`step`).
- Hiển thị **tick marks** + nhãn phần trăm dọc theo track.
- Tương tác bằng **click** hoặc **kéo thả** (GestureClick + GestureDrag).

---

## 2. API

```rust
pub fn new(initial_value: u32, on_changed: impl Fn(u32) + 'static) -> Self
pub fn new_range(min: u32, max: u32, step: u32, initial_value: u32, on_changed: impl Fn(u32) + 'static) -> Self
```

Các phương thức:

| Phương thức | Mô tả |
| :--- | :--- |
| `slider.value() -> u32` | Giá trị hiện tại |
| `slider.set_value(u32)` | Đặt giá trị (clamp trong min–max) |
| `slider.connect_change(f: impl Fn(u32))` | Đăng ký callback khi giá trị đổi |
| `slider.container` | Widget `DrawingArea` (height 56px, hexpand) |

**Thông số kỹ thuật:**

| Thuộc tính | Giá trị |
| :--- | :--- |
| Mặc định | `new()` = range 10–90, step 10, giá trị khởi đầu 10 |
| Kích thước | `content_height 56`, chiều ngang tự giãn (`hexpand`) |
| Track | Đường tròn (round cap) 6px; nền dark `rgba(255,255,255,0.15)` / light `rgba(0,0,0,0.12)` |
| Phần đã điền | `#3b82f6` (rgba 0.23, 0.51, 0.96, 1.0) |
| Knob | Tròn 9px trắng, viền `#3b82f6` 2.5px, có bóng đổ |
| Tick | Vạch 2px tại mỗi bước; tick đã qua màu xanh, chưa qua màu mờ |
| Nhãn | Text `%` cỡ 11px; tick đang chọn in đậm |
| Lề | `margin_x 24px` hai bên track |

---

## 3. Ví dụ sử dụng

```rust
// Slider mặc định (10–90, step 10)
let slider = CustomSlider::new(50, |value| {
    set_brightness(value);
});

// Slider tùy chỉnh range
let vol = CustomSlider::new_range(0, 100, 5, 60, |value| {
    volume::set_volume(value);
});
container.append(&vol.container);
```

---

## 4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Dùng `CustomSlider` cho mọi thanh điều chỉnh số (volume, backlight...) |
| DO | Range `min..max` luôn hợp lệ — constructor tự sửa nếu `min > max` |
| DO | Luôn truyền callback để cập nhật logic ngay khi người dùng click/kéo |
| DO NOT | Không dùng `gtk4::Scale` thô — kiểu dáng không đồng nhất |
| DO NOT | Không thay đổi màu track/knob ngoài `#3b82f6` và các alpha chuẩn |
