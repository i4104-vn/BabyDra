# Surfaces

Trong BabyDra, **surface** (bề mặt) là bất kỳ vùng UI nào có nền riêng — panel, card, dropdown, input shell, thanh trạng thái. Hiểu đúng cách xây dựng surface là nền tảng để mọi thành phần giao diện trông nhất quán và đúng phong cách.

---

## Công thức của một surface chuẩn

Mỗi surface được xây dựng từ bốn thành phần theo thứ tự lớp sau:

```
[Shadow bên dưới]
[Background bán trong suốt]  ← nền kính mờ
[Blur hiệu ứng]              ← làm mờ nội dung phía sau
[Border mỏng]                ← tạo ảnh sáng phản chiếu
```

Không có surface nào trong hệ thống thiếu một trong bốn thành phần này khi nó là bề mặt nổi. Nếu thấy mình đang bỏ qua một thành phần, cần có lý do rõ ràng.

---

## Cấp độ bề mặt (Elevation)

BabyDra không dùng nhiều cấp độ shadow phức tạp như Material Design. Thay vào đó chỉ có hai cấp độ:

**Cấp 1 — Bề mặt ứng dụng**
Panel chính, sidebar, thanh taskbar. Đây là lớp nền tảng của giao diện, nằm trực tiếp trên wallpaper. Shadow vừa phải, blur đủ dùng.

**Cấp 2 — Bề mặt nổi**
Dropdown, popover, tooltip, dialog. Xuất hiện phía trên bề mặt cấp 1. Shadow lớn hơn để tạo khoảng cách thị giác rõ ràng với cấp 1.

Không có cấp 3. Nếu cần một dropdown xuất hiện bên trong một dropdown khác, cần xem xét lại luồng UX thay vì thêm cấp độ elevation mới.

---

## Cách viền tạo ra chiều sâu

Viền trong BabyDra không phải đường kẻ phân vùng đơn thuần. Nó giả lập ánh sáng phản chiếu trên mặt kính:

- **Cạnh trên** — luôn sáng hơn các cạnh khác (bevel). Ánh sáng từ trên xuống chạm vào cạnh trên của bề mặt tạo ra phản quang.
- **Ba cạnh còn lại** — mờ hơn, dùng màu border chuẩn từ tokens.

Nếu tất cả bốn cạnh cùng màu, surface trông phẳng và thiếu chiều sâu. Đây là điều cần tránh.

---

## Bo góc theo kích thước

Bo góc không phải giá trị tuỳ chọn — nó tương quan với kích thước của surface:

- **Thành phần rất nhỏ** (badge, chip, tag, nút viên thuốc): `9999px` — tròn hoàn toàn.
- **Thành phần vừa** (dropdown, dialog nhỏ): `20px`.
- **Thành phần lớn** (khung ảnh chính, input shell, panel lớn): `24px`.
- **Avatar, nút icon tròn**: `50%`.

Nguyên tắc: **thành phần càng lớn thì góc càng nhỏ tương đối**. Một khung ảnh `600px` với `border-radius: 9999px` sẽ trông kỳ lạ; một chip `80px` với `border-radius: 4px` sẽ trông cứng nhắc và lạc lõng.

---

## Khi nào không dùng đủ bốn thành phần

Có những trường hợp hợp lệ để bỏ một số thành phần:

- **Phần tử nằm bên trong surface khác** — ví dụ dòng menu bên trong dropdown không cần blur hay shadow riêng.
- **Phần tử phẳng hoàn toàn** — icon button trên nền trong suốt, không có nền nổi bật.
- **Divider và separator** — không phải surface, không cần bốn thành phần.

Nguyên tắc kiểm tra: nếu phần tử này bị xóa đi, layout vẫn hoạt động và đọc được không? Nếu có — nó có thể là phần tử phẳng, không phải surface.
