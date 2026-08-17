# Component: Buttons

Tài liệu quy định cách sử dụng, kiểu dáng, và mã CSS chuẩn cho toàn bộ nút bấm (Button) trong hệ thống BabyDra.

---

## 1. Nguyên tắc thiết kế Button

1. **Hình dạng (Geometry)**:
   - **Nút hành động (Primary & Secondary)**: Luôn dùng dạng viên thuốc **Pill** (`border-radius: 9999px`).
   - **Nút icon**: Luôn dùng dạng **Tròn** (`border-radius: 50%`).
   - **Tuyệt đối không** dùng góc bo vuông/chữ nhật (`8px`, `10px`) cho các nút hành động tiêu chuẩn trong dialog và settings.

2. **Phản hồi tương tác (State Feedback)**:
   - Chỉ thay đổi màu sắc nền (`background-color`) và viền (`border-color`).
   - **Không** sử dụng `transform: translateY()`, `scale()`, hay thay đổi kích thước/hình dạng khi hover/active.
   - Transition chuẩn: `transition: background-color 200ms ease, border-color 200ms ease`.

3. **Phân cấp chức năng**:
   - **Primary Action** (`.suggested-action`, `.btn-primary`): Dùng màu xanh nhấn `#3b82f6` cho hành động chính duy nhất trong màn hình/dialog (Save Changes, Configure IP, Connect, Apply Changes).
   - **Secondary Action** (`.connect-pill-btn`, `.btn-share-pill`): Dùng màu kính mờ translucent mờ nhạt cho các hành động phụ (Close, Cancel, Refresh, Disconnect, Add New).

---

## 2. Các Class CSS Chuẩn

### Primary Button (`.suggested-action`, `.btn-primary`)

- **Công dụng**: Nút hành động chính.
- **HTML/CSS Selector**: `button.suggested-action`, `button.btn-primary`
- **Hình dáng**: `border-radius: 9999px`
- **Màu nền**: `#3b82f6` (Tailwind Blue-500)
- **Màu Hover**: `#2563eb` (Blue-600)
- **Màu Active**: `#1d4ed8` (Blue-700)
- **Chữ**: `#ffffff`, `font-size: 13px`, `font-weight: 600`

### Secondary Pill Button (`.connect-pill-btn`, `.btn-share-pill`)

- **Công dụng**: Nút hành động phụ, hủy, đóng.
- **HTML/CSS Selector**: `button.connect-pill-btn`, `button.btn-share-pill`
- **Hình dáng**: `border-radius: 9999px`
- **Màu nền Dark mode**: `rgba(255, 255, 255, 0.08)`, viền `rgba(255, 255, 255, 0.14)`, top bevel `rgba(255, 255, 255, 0.28)`
- **Màu Hover Dark mode**: `rgba(255, 255, 255, 0.15)`
- **Màu nền Light mode**: `rgba(0, 0, 0, 0.05)`, viền `rgba(0, 0, 0, 0.08)`
- **Màu Hover Light mode**: `rgba(0, 0, 0, 0.10)`
- **Chữ**: Dark: `rgba(255, 255, 255, 0.95)` | Light: `rgba(28, 28, 30, 0.95)`, `font-size: 13px`, `font-weight: 500`

---

## 3. Vị trí File Mã Nguồn CSS

Mọi style của nút bấm được tập trung khai báo tại:
- `kits/babydra-ui-kit/src/styles/shared/shared/button.css` (Cấu trúc & layout dùng chung)
- `themes/babydra-default/css/dark.css` (Màu sắc chế độ Tối — theme package)
- `themes/babydra-default/css/light.css` (Màu sắc chế độ Sáng — theme package)

> [!IMPORTANT]
> **Quy tắc cho Developer**: Không tự ý ghi đè `border-radius`, `transform`, hay `box-shadow` của `button.suggested-action` hoặc `.connect-pill-btn` trong các file CSS riêng lẻ của ứng dụng (`settings.css`, `dialogs.css`). Mọi widget GTK phải kế thừa trực tiếp từ hệ thống CSS dùng chung này.
