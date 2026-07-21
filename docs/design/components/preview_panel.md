# Hướng dẫn Thiết kế: Preview Panel (Vùng Hiển thị Ảnh & Biến thể)

---

## 1. Bố cục tổng thể

Chia vùng hiển thị thành 2 phần ngang: Khung ảnh chính (chiếm phần lớn diện tích) + Cột biến thể (4 ô vuông xếp dọc ở mép phải).

---

## 2. Cách tạo Khung Ảnh chính

Đặt `border-radius: 24px`. Thêm shadow hệ thống. Kích thước lớn, chiếm trung tâm. Tỷ lệ linh hoạt theo lựa chọn người dùng (1:1, 4:3, 3:4).

Trạng thái loading: dùng hiệu ứng Skeleton Pulse (nền mờ dần sáng lên liên tục, `animation: pulse 1.2s ease-in-out infinite`).

---

## 3. Cách tạo Cột Biến thể

Xếp 4 ô vuông dọc. Mỗi ô đặt `border-radius: 16px`.

- **Ô active**: thêm `border: 2px solid #3b82f6` (accent).
- **Ô không active**: không viền màu nhấn.
- **Hover**: tăng brightness nhẹ hoặc thêm viền mờ, `transition: all 200ms ease`. Không dùng `translateY` hay `scale`.
- **Click**: viền accent di chuyển sang ô mới, ảnh ở khung chính chuyển đổi mượt mà.
