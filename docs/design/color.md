# Color

BabyDra sử dụng một bảng màu tối giản có chủ đích: một màu nhấn chủ đạo duy nhất, một màu thành công duy nhất, và toàn bộ phần còn lại là biến thể của trắng hoặc đen với alpha khác nhau. Sự tối giản này không phải hạn chế — đây là một lựa chọn thiết kế để giao diện luôn trông nhất quán và không bao giờ loạn màu.

---

## Triết lý: Ít màu, nhiều biểu cảm

Hầu hết các hệ thống thiết kế dùng 5–10 màu chức năng khác nhau (primary, secondary, warning, error, info...). BabyDra chủ ý không làm vậy. Lý do:

- **Nhất quán thị giác** — Người dùng nhìn vào bất kỳ chỗ nào cũng thấy cùng ngôn ngữ màu sắc. Không có sự ngạc nhiên.
- **Tập trung sự chú ý** — Khi chỉ có một màu nhấn, bất cứ thứ gì dùng màu đó đều là quan trọng. Nếu dùng năm màu nhấn, không cái nào còn là điểm nhấn nữa.
- **Dễ maintain** — Thay đổi accent color chỉ cần sửa một chỗ, không phải tìm trong hàng chục biến màu.

---

## Màu nhấn chủ đạo: `#3b82f6`

Màu xanh dương `#3b82f6` (Tailwind Blue-500) là màu chức năng duy nhất trong hệ thống. Nó xuất hiện ở:

- Viền của phần tử đang được chọn (active state)
- Thanh tiến trình và fill
- Nút hành động chính (primary button)
- Chỉ báo trạng thái kích hoạt (toggle on)
- Kiểu chữ chỉ số quan trọng (percentage, label nhấn mạnh)

**Tại sao xanh dương?** Xanh dương là màu trung tính về cảm xúc, không gây lo lắng (đỏ), không gây cảm giác "tiền bạc" (xanh lá), và dễ đọc trên cả nền tối lẫn nền sáng. Ở độ sáng `500`, nó đủ rõ trên dark mode mà không chói trên light mode.

**Pressed state**: `#2563eb` (Blue-600) — tối hơn 10% khi nhấn.

---

## Màu thứ hai: Success Green

`#10b981` (Emerald-500) hoặc `#4ade80` (Green-400) chỉ dùng cho:

- Chỉ báo hoàn thành hoặc trạng thái tốt
- Credit meter khi mức còn nhiều
- Badge trạng thái thành công

Màu này không bao giờ dùng thay thế cho accent blue. Hai màu này song song nhau nhưng không chồng chéo chức năng.

---

## Phần còn lại: Alpha trên đen/trắng

Tất cả màu nền, viền, text phụ, hover background đều là màu đen hoặc trắng với alpha khác nhau — không phải màu cụ thể được chọn tay. Điều này đảm bảo:

- Trên dark theme: `rgba(255, 255, 255, X)` — trắng với alpha thay đổi theo cấp độ.
- Trên light theme: `rgba(0, 0, 0, X)` — đen với alpha thay đổi theo cấp độ.

Cùng một alpha value trên nền tối và nền sáng cho ra kết quả thị giác nhất quán nhau, không cần đặt màu riêng cho từng theme.

---

## Hướng dùng màu đúng

**Khi muốn nhấn mạnh điều gì đó** — dùng accent blue `#3b82f6`. Không dùng bold font hay tăng size.

**Khi muốn phần tử trông "đang hoạt động"** — dùng nền accent blue hoặc viền accent blue. Không dùng màu tùy ý.

**Khi muốn phân cấp thông tin** — dùng opacity alpha của text-primary vs text-secondary. Text quan trọng ở opacity cao, text phụ ở opacity thấp hơn. Không dùng màu khác nhau để phân cấp.

**Khi muốn trạng thái lỗi hoặc cảnh báo** — hiện tại hệ thống chưa định nghĩa error color riêng. Dùng text-secondary đủ để thông báo lỗi nhẹ, hoặc nếu cần nổi bật dùng alpha của accent blue giảm xuống để tạo nền cảnh báo nhẹ.

---

## Màu KHÔNG được tự ý thêm

Không tự thêm màu mới vào hệ thống (tím, cam, hồng nóng, đỏ...) mà không có lý do chức năng rõ ràng. Nếu cần màu mới cho một trạng thái mới, phải cập nhật `tokens.md` và đảm bảo màu đó hoạt động trên cả dark và light theme trước khi dùng.
