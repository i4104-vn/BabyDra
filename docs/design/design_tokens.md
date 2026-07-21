# Design Tokens

Áp dụng các giá trị dưới đây cho mọi thành phần giao diện. Không tự ý đặt giá trị ngoài hệ thống này.

---

## 1. Bảng màu Nền & Bề mặt

### Dark Theme

| Token              | Giá trị                                 | Dùng cho                                                    |
| :----------------- | :-------------------------------------- | :---------------------------------------------------------- |
| `surface`          | `rgba(14, 14, 18, 0.96)`                | Nền chính của mọi bề mặt: card, popover, panel, input shell |
| `blur`             | `-gtk-background-blur: 24`              | Đặt kèm `surface` để tạo hiệu ứng Acrylic                   |
| `border`           | `1px solid rgba(255, 255, 255, 0.14)`   | Viền ngoài của mọi bề mặt                                   |
| `border-top-bevel` | `rgba(255, 255, 255, 0.28)`             | Chỉ dùng cho viền cạnh trên để tạo phản quang               |
| `text-primary`     | `#ffffff` / `rgba(255, 255, 255, 0.95)` | Tiêu đề, tên người dùng, nhãn chức năng                     |
| `text-secondary`   | `rgba(255, 255, 255, 0.50)`             | Email, mô tả phụ, placeholder                               |
| `hover-bg`         | `rgba(255, 255, 255, 0.08)`             | Nền khi hover dòng menu, nút, chip                          |
| `separator`        | `1px solid rgba(255, 255, 255, 0.10)`   | Đường phân cách giữa các nhóm chức năng                     |

### Light Theme

| Token              | Giá trị                                 | Dùng cho                                  |
| :----------------- | :-------------------------------------- | :---------------------------------------- |
| `surface`          | `rgba(255, 255, 255, 0.98)` / `#ffffff` | Nền chính của mọi bề mặt                  |
| `blur`             | `-gtk-background-blur: 24`              | Đặt kèm `surface` để tạo hiệu ứng Acrylic |
| `border`           | `1px solid rgba(0, 0, 0, 0.08)`         | Viền ngoài của mọi bề mặt                 |
| `border-top-bevel` | `rgba(0, 0, 0, 0.06)`                   | Chỉ dùng cho viền cạnh trên               |
| `text-primary`     | `#1c1c1e` / `rgba(28, 28, 30, 0.95)`    | Tiêu đề, tên người dùng, nhãn chức năng   |
| `text-secondary`   | `rgba(28, 28, 30, 0.50)`                | Email, mô tả phụ, placeholder             |
| `hover-bg`         | `rgba(0, 0, 0, 0.05)` / `#f4f4f5`       | Nền khi hover dòng menu, nút, chip        |
| `separator`        | `1px solid rgba(0, 0, 0, 0.06)`         | Đường phân cách giữa các nhóm chức năng   |

---

## 2. Bảng màu Điểm nhấn & Trạng thái

| Token            | Giá trị               | Dùng cho                                              |
| :--------------- | :-------------------- | :---------------------------------------------------- |
| `accent`         | `#3b82f6`             | Viền active, checkmark, nút primary, thanh tiến trình |
| `accent-pressed` | `#2563eb`             | Nền nút khi nhấn (pressed state)                      |
| `success`        | `#10b981` / `#4ade80` | Vòng credit, badge PRO xanh lá                        |
| `pink`           | `#f472b6`             | Badge PRO hồng, thành phần trong rainbow ring         |
| `badge-pink-bg`  | `#fce7f3`             | Nền badge PRO biến thể hồng                           |
| `badge-pink-fg`  | `#be185d`             | Chữ và icon badge PRO biến thể hồng                   |
| `badge-green-bg` | `#dcfce7`             | Nền badge PRO biến thể xanh lá                        |
| `badge-green-fg` | `#15803d`             | Chữ và icon badge PRO biến thể xanh lá                |

---

## 3. Typography

Dùng phông `Inter` (Sans-serif) làm phông duy nhất cho toàn bộ ứng dụng.

| Cấp bậc      | font-size       | font-weight      | Dùng cho                                       |
| :----------- | :-------------- | :--------------- | :--------------------------------------------- |
| Header Name  | `14px` - `15px` | 700 (Bold)       | Tên người dùng trong dropdown, tiêu đề section |
| Body / Label | `13px` - `14px` | 500 (Medium)     | Nhãn dòng menu, nhãn nút bấm, tiêu đề chip     |
| Subtext      | `12px`          | 400 (Regular)    | Email, mô tả phụ, tooltip                      |
| Badge        | `10px` - `11px` | 800 (Extra-bold) | Nhãn PRO, chữ viết hoa trong badge             |

---

## 4. Border Radius

| Token           | Giá trị         | Dùng cho                                         |
| :-------------- | :-------------- | :----------------------------------------------- |
| `radius-pill`   | `9999px`        | Nút bấm chính, nút Share, nav pill, badge, chip  |
| `radius-lg`     | `20px`          | Popover dropdown, khung preview lớn              |
| `radius-xl`     | `24px`          | Khung ảnh chính, input floating shell            |
| `radius-md`     | `10px` - `12px` | Dòng menu hover, thẻ biến thể, nút chức năng nhỏ |
| `radius-circle` | `50%`           | Avatar, nút biểu tượng tròn                      |

---

## 5. Spacing

| Token            | Giá trị         | Dùng cho                                                        |
| :--------------- | :-------------- | :-------------------------------------------------------------- |
| `space-micro`    | `4px` - `6px`   | Khoảng cách giữa icon và chữ cùng dòng                          |
| `space-standard` | `8px` - `12px`  | Padding bên trong dòng menu, gap giữa phần tử cùng cấp          |
| `space-section`  | `16px` - `20px` | Padding viền trong popover, khoảng cách giữa các nhóm chức năng |

---

## 6. Shadow & Elevation

Dùng shadow duy nhất cho mọi bề mặt nổi (popover, card, input shell):

- **Light Theme**: `box-shadow: 0 10px 30px rgba(0, 0, 0, 0.08)`
- **Dark Theme**: `box-shadow: 0 10px 30px rgba(0, 0, 0, 0.35)`

---

## 7. Quy tắc Hover (Bắt buộc)

Không dùng `translateY`, `scale`, `transform` hay bất kỳ dịch chuyển hình học nào khi hover. Chỉ thay đổi `background-color` và/hoặc `color` với `transition: all 200ms ease`.
