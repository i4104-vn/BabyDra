# Motion

Chuyển động trong BabyDra không phải để trang trí — mỗi animation đều có một lý do chức năng: hướng sự chú ý, giải thích quan hệ không gian, hay giảm cảm giác chờ đợi.

---

## Triết lý: Chuyển động phục vụ nội dung, không che lấp

Hầu hết animation trong giao diện thường được thêm vào vì trông "cool" hơn. BabyDra làm ngược lại: chỉ thêm animation khi không có nó, trải nghiệm sẽ kém đi rõ rệt. Mọi animation phải vượt qua câu hỏi: **"Nếu bỏ animation này, giao diện có mất đi thông tin hoặc cảm giác gì không?"**

Nếu câu trả lời là không — animation đó không cần thiết.

---

## Ba loại chuyển động trong hệ thống

### 1. Transition trạng thái (State Transition)

Dùng cho mọi thay đổi trạng thái của phần tử đang hiển thị: hover, active, selected, disabled.

- Thời gian: `200ms`
- Easing: `ease` (tăng tốc nhanh, giảm tốc từ từ)
- Áp dụng cho: màu nền, màu text, màu viền, opacity.

Đây là loại animation phổ biến nhất trong hệ thống. Nó xảy ra hàng trăm lần trong một phiên làm việc mà người dùng hầu như không nhận ra — đó là dấu hiệu nó đang làm đúng việc của mình.

### 2. Chuyển động xuất hiện/biến mất (Enter/Exit)

Dùng cho dropdown, popover, tooltip, dialog — những thành phần được tạo ra và hủy trong quá trình dùng.

**Xuất hiện**: Trượt nhẹ từ nguồn gốc về vị trí thực (`translateY(-8px)` → `translateY(0)`) kết hợp fade in (`opacity: 0` → `1`). Thời gian `200ms ease-out`.

**Biến mất**: Fade out nhanh hơn xuất hiện, `150ms ease-in`. Không cần reverse animation phức tạp.

Lưu ý: Transform chỉ được dùng trong keyframe animation của enter/exit — không bao giờ trong hover state.

### 3. Genie Animation (Cửa sổ đóng/mở)

Đây là animation đặc trưng nhất của BabyDra, lấy cảm hứng từ macOS Genie effect. Khi panel chính được đóng hoặc mở, cửa sổ co giãn và biến đổi hướng về phía nút bấm kích hoạt thay vì scale từ tâm màn hình.

Mục đích: Người dùng luôn biết "cửa sổ này đến từ đâu và sẽ về đâu". Điều này tạo ra cảm giác không gian liên tục thay vì nội dung bỗng nhiên xuất hiện/biến mất.

Thời gian: `450ms` với easing tùy chỉnh (khởi đầu nhanh, kết thúc chậm để cảm giác "đáp xuống" tự nhiên).

---

## Tốc độ animation: nhanh hơn bạn nghĩ

Một sai lầm phổ biến là dùng animation quá chậm vì "trông mượt mà hơn". Thực tế ngược lại: animation quá chậm trở thành vật cản người dùng phải chờ.

Quy tắc thực tế của BabyDra:
- Transition trạng thái: `200ms` — tối đa, không thêm.
- Enter animation: `200ms` — đủ để nhận ra có gì đó xuất hiện.
- Exit animation: `150ms` — phải biến mất nhanh hơn xuất hiện.
- Genie/slide animation: `400ms–450ms` — animation phức tạp được phép chậm hơn.

---

## Skeleton Loading: animation thay thế nội dung

Khi giao diện phải chờ dữ liệu, thay vì để trống hay dùng spinner quay, BabyDra dùng skeleton pulse: vùng placeholder nhịp nhàng sáng lên tối xuống ở tần suất `1.2s/cycle`.

Mục đích: Người dùng thấy bố cục đang được giữ chỗ — họ biết nội dung sẽ xuất hiện ở đúng chỗ đó, không phải đoán. Điều này giảm anxiety khi chờ tốt hơn spinner.

Skeleton chỉ dùng cho vùng nội dung lớn (khung ảnh chính, variation cards). Không dùng skeleton cho icon đơn lẻ hay text ngắn — những thứ đó chỉ cần để trống hoặc dùng text placeholder.

---

## Những animation không có trong hệ thống

- Parallax scrolling
- Particle effects
- Lottie animation phức tạp
- Transform 3D (rotateX, rotateY, perspective)
- Bounce, elastic easing
- Ripple effect (Material Design style)

Nếu cảm thấy muốn thêm một trong những thứ này — dừng lại và hỏi xem nó giải quyết vấn đề gì cụ thể.
