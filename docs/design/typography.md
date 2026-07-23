# Typography

BabyDra dùng duy nhất phông **Inter** cho toàn bộ giao diện. Đây là quyết định có chủ đích, không phải vì thiếu lựa chọn.

---

## Tại sao Inter?

Inter được thiết kế từ đầu cho màn hình kỹ thuật số, không phải cho in ấn. Ở kích thước nhỏ (`11px–14px`) — vùng kích thước phổ biến nhất của giao diện desktop — Inter giữ được độ rõ ràng và cân bằng thị giác tốt hơn hầu hết các phông khác. Đặc biệt, Inter được tối ưu cho màn hình HiDPI (Retina/4K) — môi trường mà BabyDra được thiết kế để chạy.

Không dùng nhiều phông. Hệ thống một phông duy nhất với nhiều `font-weight` tạo ra giao diện nhất quán và chuyên nghiệp hơn hai phông khác nhau.

---

## Phân cấp thông tin qua font-weight, không qua font-size

Sai lầm phổ biến trong thiết kế giao diện là dùng `font-size` lớn hơn để nhấn mạnh. BabyDra làm ngược lại: **phân cấp thông tin chủ yếu qua `font-weight` và `opacity`**, giữ `font-size` trong dải hẹp.

Lý do: Thay đổi font-size phá vỡ sự cân bằng thị giác của layout, đặc biệt trong không gian nhỏ như panel hay dropdown. Thay đổi weight và opacity giữ layout ổn định trong khi vẫn truyền đạt được thứ tự quan trọng.

---

## Bốn cấp bậc phân cấp

**Cấp 1 — Tiêu đề / Tên quan trọng**
`font-size: 14px–15px`, `font-weight: 700`
Dùng cho: tên người dùng trong profile, tiêu đề section, nhãn chức năng chính.

**Cấp 2 — Nhãn / Nội dung tương tác**
`font-size: 13px–14px`, `font-weight: 500–600`
Dùng cho: nhãn nút bấm, nhãn dòng menu, tiêu đề chip, text trong toolbar.

**Cấp 3 — Văn bản phụ / Mô tả**
`font-size: 12px–13px`, `font-weight: 400`
Dùng cho: email, mô tả phụ, placeholder, chú thích, văn bản ngữ cảnh.

**Cấp 4 — Nhãn cực nhỏ**
`font-size: 10px–11px`, `font-weight: 700–800`
Dùng riêng cho badge và nhãn viết hoa ngắn. Không dùng ở cấp bậc 10px cho văn bản đọc thông thường.

---

## Viết hoa có chủ đích

Chỉ viết hoa (`text-transform: uppercase`) cho một loại thành phần: badge label ngắn như "PRO". Không viết hoa nhãn menu, tiêu đề section, hay chú thích. Viết hoa quá nhiều làm giao diện trông cứng nhắc và khó đọc.

---

## Letter spacing

Chỉ thêm `letter-spacing` cho badge viết hoa (`0.3px–0.5px`) để bù lại việc chữ hoa nhìn bị dồn cụm. Không áp dụng letter-spacing cho text thông thường — Inter đã được thiết kế với spacing tốt nhất ở giá trị mặc định.

---

## Line height

- Text đơn dòng (label, badge, nút): không cần đặt line-height, dùng mặc định.
- Text nhiều dòng (mô tả, ngữ cảnh): `line-height: 1.5–1.6` để đủ thoáng.
- Không dùng `line-height: 1` — chữ bị dính nhau theo chiều dọc.
