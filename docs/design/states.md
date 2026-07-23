# States

Trạng thái tương tác — hover, active, disabled — là cách giao diện phản hồi với người dùng. BabyDra có một triết lý rõ ràng và nhất quán cho tất cả trạng thái này, khác biệt với nhiều hệ thống thiết kế phổ biến.

---

## Triết lý tổng quan: Phản hồi qua màu sắc, không qua hình học

BabyDra không bao giờ thay đổi vị trí, kích thước hay hình dạng của phần tử để phản hồi tương tác. Mọi phản hồi tương tác — hover, focus, active — chỉ được biểu hiện qua sự thay đổi màu sắc.

**Tại sao?**

Khi một nút nhô lên, phóng to, hay dịch chuyển khi hover, layout bị phá vỡ tạm thời — các phần tử xung quanh có thể bị đẩy, viewport bị rung nhẹ. Điều này tạo ra cảm giác bồn chồn và thiếu ổn định, đặc biệt trong giao diện desktop dày đặc thông tin như BabyDra. Phản hồi qua màu sắc không ảnh hưởng layout, không gây rung, không làm mất tập trung.

Thêm vào đó, transition màu sắc nhanh hơn và nhẹ hơn về mặt render so với transition transform. Trên hệ thống có nhiều widget cùng lúc, điều này có ý nghĩa.

---

## Hover

Hover là tín hiệu "tôi đang chú ý đến phần tử này". Phản hồi phải đủ rõ để người dùng nhận ra, nhưng không đủ nổi bật để gây mất tập trung.

**Đối với phần tử có nền** (nút, dòng menu, chip): Tăng opacity của nền thêm khoảng `4%–8%`. Trên dark mode, nền tối trở nên sáng hơn nhẹ. Trên light mode, nền sáng trở nên đậm hơn nhẹ.

**Đối với phần tử chỉ có icon/text** (icon button trong suốt): Thêm nền mờ nhạt phía sau.

**Đối với phần tử đang ở trạng thái active** (toggle đang bật, tab đang chọn): Hover làm tối thêm màu accent `#3b82f6` xuống `#2563eb`, không thêm transform hay shadow.

Tất cả hover đều có `transition: all 200ms ease`. Không có hover tức thì không có transition.

---

## Active / Pressed

Active là khoảnh khắc người dùng đang nhấn giữ. Phản hồi phải nhanh hơn hover và rõ hơn một bậc.

- Nền đậm thêm so với hover.
- Thời gian transition rút ngắn xuống còn khoảng `100ms` để cảm giác tức thì.
- Không dùng `box-shadow` inset để tạo hiệu ứng nhấn vào — điều này tạo ra ảo giác 3D không phù hợp với ngôn ngữ phẳng của BabyDra.

---

## Trạng thái được chọn / Đang hoạt động (Selected / On)

Đây là trạng thái bền vững — khác với hover là tạm thời. Tab đang active, toggle đang bật, variation card đang chọn — tất cả là "selected state".

**Hướng làm:**
- Thêm viền accent `#3b82f6` cho phần tử được chọn (approach rõ ràng nhất).
- Hoặc thêm nền accent với opacity thấp (ví dụ: `rgba(59, 130, 246, 0.15)`) khi viền không phù hợp.
- Icon và text đổi sang màu accent để tăng cường tín hiệu.

**Không** dùng scale hay shadow thêm để báo hiệu selected state.

---

## Disabled

Phần tử bị vô hiệu hóa cần trông rõ ràng là không thể tương tác, nhưng không được chiếm quá nhiều sự chú ý.

- Opacity tổng thể giảm xuống khoảng `40%–50%`.
- `cursor: not-allowed`.
- `pointer-events: none` để không nhận click.
- Không thay đổi màu nền hay thêm màu riêng cho disabled — giảm opacity đủ để truyền đạt.

---

## Focus (Keyboard navigation)

Focus state dùng cho điều hướng bàn phím. BabyDra chưa có spec đầy đủ cho focus indicator, nhưng hướng làm là: dùng outline accent `#3b82f6` với `outline-offset: 2px` thay vì browser default focus ring.

---

## Tóm tắt quy tắc

| Trạng thái | Phản hồi | Không làm |
| :--- | :--- | :--- |
| Hover | Đổi màu nền nhẹ, 200ms ease | Transform, scale, translateY |
| Active/Pressed | Đậm màu hơn hover, 100ms | inset shadow, border thay đổi |
| Selected/On | Viền accent hoặc nền accent mờ | Shadow to lên, kích thước thay đổi |
| Disabled | Opacity 40–50%, no pointer events | Màu nền khác, text riêng |
