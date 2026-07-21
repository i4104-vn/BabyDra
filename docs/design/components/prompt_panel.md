# Hướng dẫn Thiết kế: Prompt Panel (Panel Ngữ cảnh)

---

## 1. Bố cục tổng thể

Đặt panel ở cột bên trái màn hình. Xếp dọc từ trên xuống: Ảnh tham chiếu -> Keyword Chips -> Đoạn văn mô tả.

---

## 2. Cách tạo Ảnh Tham chiếu

Đặt `border-radius: 16px`. Tỷ lệ ngang `16:9`. Đặt Floating Icon Badge (xem `badge.md` mục 4) ở góc dưới trái bằng `position: absolute`.

Hover: chỉ đổi opacity hoặc brightness nhẹ, `transition: all 200ms ease`. Không dùng `scale` hay `zoom`.

---

## 3. Cách tạo Keyword Chips

Xếp nối tiếp nhau theo dòng (Flex wrap). Dùng 2 loại:

- **Highlight Chip**: tạo theo `badge.md` mục 3 (Keyword Chip có khung nền). Dùng cho các từ khóa quan trọng (`dapper fox`, `in a green suit`).
- **Static Text**: chữ thường `text-secondary`, `font-weight: 400`, không có khung nền. Dùng cho cụm nối (`Create a`, `wearing`).

---

## 4. Cách tạo Đoạn mô tả Ngữ cảnh

Căn trái. `font-size: 12px` - `13px`. Màu `text-secondary`. Line-height `1.5`. Dùng để thuyết minh phong cách và thẩm mỹ, không cạnh tranh thị giác với khung ảnh chính.
