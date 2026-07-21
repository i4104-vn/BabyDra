# Hướng dẫn Thiết kế: Progress (Tiến trình & Chỉ số)

---

## 1. Cách tạo Credit Meter

Tạo khung chứa `border-radius: 9999px` (pill). Nền mờ nhẹ (`hover-bg` token hoặc nhạt hơn). Bên trong chứa:

- **Vòng tròn tiến trình**: SVG hoặc conic-gradient nhỏ, viền `#10b981` / `#4ade80` (success), track nền xám mờ. Tỷ lệ hiển thị phần trăm đã dùng.
- **Nhãn chữ**: "18% Daily Credits", `font-size: 12px`, `font-weight: 400`, màu `text-secondary`.

Hover: đậm nền khung pill nhẹ, `transition: all 200ms ease`. Không dùng `translateY` hay `scale`. Hiển thị tooltip số lượt sinh ảnh còn lại.
