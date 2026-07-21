# Quy chuẩn Thiết kế (Design Tokens & Visual Guidelines)

Tài liệu này định nghĩa hệ thống biến thiết kế (Design Tokens), quy chuẩn thẩm mỹ và hướng dẫn thị giác cho toàn bộ giao diện ứng dụng **BabyDra**.

---

## 🎨 1. Hệ thống Bảng màu (Color System & Palette)

### 1.1. Màu nền & Bề mặt (Background & Surface Colors)

- **Màu nền ứng dụng (App Canvas Background)**: Tone xám kem hồng ấm (`Warm Pinkish Gray`). Tạo cảm giác dễ chịu, nghệ thuật và làm nổi bật tác phẩm chính ở trung tâm.
- **Màu bề mặt Card (White Surface)**: Màu trắng tuyền tinh khiết. Dùng cho thẻ ảnh chính, thanh nhập liệu, thanh side-bar và các thẻ thông tin.
- **Màu bề mặt mờ nhạt (Subtle Surface)**: Màu xám kem nhạt mờ. Dùng cho các chip từ khóa, nút lựa chọn thông số và hiệu ứng hover.
- **Màu nền tương phản cao (Dark Surface)**: Màu đen tuyền nhám. Dùng cho nút hành động chính (`Generate`), nút nâng cấp (`Upgrade now`) và thanh điều hướng trung tâm.

### 1.2. Màu điểm nhấn & Trạng thái (Accent & Status Colors)

- **Màu điểm nhấn chính (Green Accent)**: Màu xanh lá cây trang nhã (lấy cảm hứng từ bộ suit xanh của nhân vật mẫu). Dùng cho vòng tiến trình credit và các icon điểm nhấn.
- **Màu hồng Pastel (Pink Accent)**: Dùng cho phông nền ảnh tạo không gian hiện đại, siêu thực.
- **Màu viền Active (Active Border Color)**: Viền đen đậm sắc nét. Dùng để đánh dấu biến thể ảnh đang được chọn.

---

## ✏️ 2. Quy chuẩn Chữ & Phân cấp Thị giác (Typography Scale)

- **Phông chữ chủ đạo**: Chữ không chân hiện đại (Sans-serif) với nét bo tròn nhẹ ở các góc chữ, mang lại cảm giác thân thiện và cao cấp.

### Phân cấp Typography:

1. **Heading (Tiêu đề lớn)**: Cỡ chữ lớn, nét đậm (Bold). Dùng cho các tiêu đề trang hoặc nhóm nội dung chính.
2. **Body Text (Văn bản nội dung)**: Cỡ chữ vừa, nét vừa (Medium). Dùng cho ô nhập liệu prompt và đoạn văn bản mô tả ý tưởng.
3. **Chip / Label Text**: Cỡ chữ nhỏ, nét đậm vừa (Semi-bold). Dùng cho các tag từ khóa prompt, nút điều chỉnh tỷ lệ `4:3`, `4K`.
4. **Caption Text**: Cỡ chữ siêu nhỏ, dùng cho bộ đếm credit phần trăm `18%` và các nhãn ghi chú phụ.

---

## 🔲 3. Quy chuẩn Bo góc & Tỉ lệ (Border Radius & Spacing)

### 3.1. Độ Bo góc (Border Radius Hierarchy)

- **Khoang nhộng tròn (Pill Shape - Full Radius)**: Áp dụng cho nút bấm (`Generate`, `Upgrade`), thanh Nav Pills, thanh nhập liệu chính và các tag từ khóa. Giúp giao diện mềm mại, mượt mà.
- **Bo góc cực lớn (XL Radius - 24px)**: Áp dụng cho Khung xem ảnh kết quả chính (Main Preview Card) và Khung chứa ô nhập liệu nổi ở phía dưới.
- **Bo góc vừa (Large Radius - 16px)**: Áp dụng cho các ô xem ảnh biến thể (Variations) và thẻ ảnh tham chiếu (Reference Card).

### 3.2. Thang Khoảng cách (Spacing Scale)

- **Khoảng cách siêu nhỏ (Micro Spacing - 4px to 8px)**: Khoảng cách giữa icon và chữ trong cùng một nút bấm/chip.
- **Khoảng cách tiêu chuẩn (Standard Spacing - 12px to 16px)**: Khoảng cách giữa các ô ảnh biến thể, khoảng cách giữa các chip prompt.
- **Khoảng cách khu vực (Section Spacing - 24px to 32px)**: Lề phân tách giữa cột bên trái, khung ảnh trung tâm và cột bên phải.

---

## 🌫️ 4. Độ Đổ bóng & Phân tầng Không gian (Elevation & Shadows)

- **Shadow nhẹ (Subtle Elevation)**: Đổ bóng mờ nhạt cho các nút bấm phẳng, tạo cảm giác bề mặt hơi nổi nhẹ so với nền.
- **Shadow trung bình (Card Elevation)**: Đổ bóng dịu cho Khung xem ảnh chính và các ô biến thể, giúp tách biệt ảnh khỏi phông nền ứng dụng.
- **Shadow cao (Floating Elevation)**: Đổ bóng sâu và rộng cho Khung nhập liệu nổi ở phía dưới màn hình và các Menu Dropdown xổ xuống, tạo cảm giác lơ lửng trên cùng của không gian giao diện.
