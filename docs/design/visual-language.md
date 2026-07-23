# Visual Language

BabyDra sử dụng ngôn ngữ thị giác **Glassmorphic Acrylic** — lấy cảm hứng từ mặt kính mờ đặt trên nền phức tạp. Đây không phải là một xu hướng thẩm mỹ đơn thuần mà là một triết lý thiết kế có chủ đích: giao diện phải cảm giác nhẹ, trong suốt, và hòa quyện vào môi trường desktop thay vì đứng rời rạc như một cửa sổ phần mềm thông thường.

---

## Cảm giác cần đạt được

Khi nhìn vào bất kỳ cửa sổ nào của BabyDra, người dùng phải cảm nhận:

- **Nhẹ và hiện đại** — không nặng nề, không đậm đặc màu sắc, không cứng nhắc.
- **Tích hợp với desktop** — nền wallpaper và môi trường xung quanh vẫn hiện qua lớp mờ nhạt, giao diện không cắt đứt người dùng khỏi không gian làm việc.
- **Sang trọng nhưng không phô** — tinh tế, không có màu sắc loè loẹt hay hiệu ứng quá mức.

Nếu một thiết kế mới tạo ra cảm giác nặng nề, đặc sệt, hay giống ứng dụng Windows XP — đó là tín hiệu cần xem xét lại.

---

## Nguyên lý cốt lõi

### Trong suốt có chủ đích

Mọi bề mặt trong BabyDra không bao giờ là màu đặc hoàn toàn. Luôn có kênh alpha đủ để nền phía sau "thở" qua. Tuy nhiên, trong suốt không có nghĩa là nhìn xuyên thấy rõ ràng — mức alpha được tính toán để giữ nội dung bên trong đọc được trong khi vẫn gợi cảm giác kính mờ.

Khi alpha quá thấp (quá trong suốt): nội dung khó đọc, giao diện trông mờ nhạt thiếu chủ thể.
Khi alpha quá cao (gần đặc): mất đi cảm giác nhẹ, trở thành cửa sổ thông thường.

Điểm cân bằng của BabyDra: khoảng `0.94–0.98` cho nền chính, đủ đặc để nội dung nổi bật nhưng vẫn giữ được gợi cảm giác kính.

### Blur là linh hồn của Glassmorphism

Hiệu ứng `-gtk-background-blur: 24` là thứ biến nền bán trong suốt thành kính mờ thực sự. Thiếu blur, nền trong suốt chỉ là nền mờ nhạt thông thường. Blur tạo ra ảo giác chiều sâu: nội dung phía sau bề mặt mờ đi, bề mặt phía trước trở nên rõ hơn.

### Viền không phải để phân cách — mà để tạo chiều sâu

Viền trong BabyDra không phải đường kẻ phân chia vùng như thiết kế truyền thống. Chức năng của nó là tạo ảo giác ánh sáng phản chiếu trên mặt kính. Viền cạnh trên (`border-top`) luôn sáng hơn các cạnh còn lại — giả lập ánh sáng chiếu từ phía trên xuống, tạo cảm giác bề mặt có độ nổi và thực.

---

## Hướng làm khi thiết kế bề mặt mới

Khi cần tạo bất kỳ bề mặt UI mới nào — panel, card, dropdown, hay bất cứ thứ gì — tư duy theo thứ tự này:

1. **Đây có phải bề mặt nổi không?** — Nếu có (nghĩa là nó nằm phía trên nội dung khác), áp dụng đủ ba yếu tố: nền bán trong suốt + blur + viền kính.
2. **Cấp độ nổi là bao nhiêu?** — Bề mặt chính của ứng dụng thấp hơn dropdown/popover. Bề mặt nổi hơn có shadow lớn hơn, blur mạnh hơn.
3. **Bo góc phù hợp với kích thước không?** — Thành phần càng nhỏ (chip, badge) thì bo góc `9999px` (pill). Thành phần càng lớn (cửa sổ, khung ảnh) thì bo góc vừa phải `20–24px`.
4. **Bóng đổ có cần không?** — Chỉ áp dụng shadow cho bề mặt thực sự nổi trên nền. Không thêm shadow vào phần tử nằm cùng phẳng với nội dung.

---

## Giới hạn cần tránh

Glassmorphism dễ bị lạm dụng và mất đi tính tinh tế. Những điều sau làm hỏng ngôn ngữ thị giác của BabyDra:

- **Quá nhiều lớp blur chồng nhau** — ba bề mặt lồng nhau đều blur sẽ tạo ra hình ảnh nhòe và khó nhìn.
- **Màu nền tự ý** — dùng màu tím, xanh lá, hay bất kỳ màu nào ngoài hệ thống surface token là phá vỡ tính nhất quán.
- **Bo góc không nhất quán** — dùng `15px` thay vì `16px`, hay `18px` thay vì `20px` là tạo ra cảm giác lộn xộn tinh tế mà người dùng không nhận ra nhưng cảm nhận được.
