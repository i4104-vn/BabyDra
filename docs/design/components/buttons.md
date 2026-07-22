# Hướng dẫn Thiết kế: Nút bấm (Buttons)

---

## 1. Nguyên tắc chung

Mọi nút bấm đều dùng dạng khoang nhộng bo tròn (`border-radius: 9999px`). Không dùng góc vuông hay bo góc nhỏ cho nút bấm. Chữ trong nút dùng `font-weight: 500` - `600`, cỡ `13px` - `14px`.

Đặt `transition: all 200ms ease` cho mọi nút. Không dùng `translateY`, `scale` hay bất kỳ dịch chuyển hình học nào khi hover.

---

## 2. Cách tạo từng loại nút

### 2.1. Nút Primary (Generate / Action)

Đặt `border-radius: 9999px`. Nền dùng `#3b82f6` (accent) hoặc `#1c1c1e` (đen sẫm) tùy theo ngữ cảnh. Chữ trắng `#ffffff`, `font-weight: 600`.

Khi hover: tối nền thêm 10% (ví dụ `#2563eb`). Khi pressed: tối thêm một bậc nữa. Khi disabled: chuyển nền sang `rgba(0, 0, 0, 0.20)` (Light) hoặc `rgba(255, 255, 255, 0.20)` (Dark), bỏ nhận click.

### 2.2. Nút Share (Pill Button phụ)

Đặt `border-radius: 9999px`. Nền dùng `rgba(0, 0, 0, 0.05)` / `#f2f2f2` (Light) hoặc `rgba(255, 255, 255, 0.08)` (Dark). Chữ dùng `text-primary`. Bên trong chứa nhãn "Share" kèm icon dấu cộng bên phải.

Khi hover: đậm nền thêm 6% - 8%.

### 2.3. Nút Upgrade (Secondary)

Đặt `border-radius: 9999px`. Nền đen tuyền `#1c1c1e` hoặc xanh sẫm. Chữ trắng. Tích hợp icon kim cương/trái tim bên trái chữ "Upgrade now".

Khi hover: sáng nền nhẹ.

### 2.4. Nút Biểu tượng (Icon Button)

Đặt `border-radius: 50%` cho nút tròn chứa icon đơn (nút `+`, nút theme toggle). Kích thước cố định (ví dụ `32px` x `32px`). Nền trong suốt hoặc xám mờ nhẹ. Icon căn giữa.

Khi hover: đổi nền sang `hover-bg` token.
