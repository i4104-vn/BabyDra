# Thiết kế Component: Badges & Chips (`badge.md`)

Tài liệu này đặc tả chi tiết hướng thiết kế cho các Thẻ nhãn từ khóa (Chips/Tokens) và Huy hiệu biểu tượng (Badges).

---

## 📌 1. Định hướng Thiết kế (Design Concept)

Badge và Chip giúp phân loại thông tin và trực quan hóa các từ khóa cốt lõi của Prompt sinh ảnh:
- **Prompt Token Chips**: Đóng gói các cụm từ quan trọng trong Prompt (`dapper fox`, `in a green suit`).
- **Floating Icon Badge**: Huy hiệu tròn biểu tượng bảng màu đính ở góc ảnh tham chiếu.

---

## 🎨 2. Phân loại & Đặc tả Trực quan

1. **Highlight Keyword Chip**: Dạng khoang nhộng bo tròn, nền xám mờ nhạt, chữ nét đậm màu đen tuyền.
2. **Static Text Chip**: Chữ tĩnh màu xám mờ không có khung nền (`Create a`).
3. **Floating Icon Badge**: Hình tròn nhỏ màu trắng tinh khiết đính ở góc ảnh, chứa biểu tượng icon bảng màu đen.

---

## 👆 3. Trạng thái Tương tác (UX States)

- **Hover vào Highlight Chip**: Nền chip đậm hơn nhẹ, báo hiệu khả năng tương tác.
- **Active Chip**: Đổi sang tông nền tương phản cao khi được nhấn chọn.
