# 09 — Ngôn ngữ thiết kế

**Phạm vi:** visual language, tokens, màu, typography, spacing, motion, states, theming.
**Phiên bản:** 2.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## 1. Ngôn ngữ thị giác: Glassmorphic Acrylic

Giao diện BabyDra là **mặt kính mờ đặt trên nền phức tạp**. Người dùng phải cảm nhận: nhẹ và hiện đại, hòa vào desktop (wallpaper hiện qua lớp mờ), sang trọng nhưng không phô.

Ba nguyên lý cốt lõi:

| Nguyên lý | Chi tiết |
| :--- | :--- |
| **Trong suốt có chủ đích** | Không bề mặt nào là màu đặc 100% — alpha nền chính `0.94–0.98` |
| **Blur là linh hồn** | `-gtk-background-blur: 24` biến nền bán trong suốt thành kính thật |
| **Viền tạo chiều sâu** | Viền cạnh trên (bevel) luôn sáng hơn 3 cạnh còn lại — giả lập ánh sáng từ trên xuống |

---

## 2. Công thức surface

Mỗi bề mặt nổi gồm 4 lớp, theo thứ tự:

```text
[Shadow]  →  [Nền bán trong suốt]  →  [Blur]  →  [Border mỏng]
```

| Cấp elevation | Bề mặt | Shadow |
| :--- | :--- | :--- |
| Cấp 1 — bề mặt ứng dụng | Panel, sidebar, taskbar | Vừa phải |
| Cấp 2 — bề mặt nổi | Dropdown, popover, tooltip, dialog | Lớn hơn (tách rõ khỏi cấp 1) |

Không có cấp 3 — cần dropdown lồng dropdown thì xem lại UX, đừng thêm elevation.

**Bo góc theo kích thước:** càng nhỏ càng tròn — chip/badge `9999px` (pill), dialog `20px`, khung ảnh/panel `24px`, avatar `50%`.

**Khi nào bỏ bớt lớp:** phần tử nằm bên trong surface khác (dòng menu trong dropdown) hoặc phẳng hoàn toàn (icon button trong suốt) — không cần blur/shadow riêng.

---

## 3. Màu sắc — ít màu, nhiều biểu cảm

| Màu | Giá trị | Dùng cho |
| :--- | :--- | :--- |
| **Accent** | `#3b82f6` (Blue-500) | Viền active, primary button, fill progress, toggle on — **màu chức năng duy nhất** |
| Accent pressed | `#2563eb` | Nền khi nhấn giữ |
| **Success** | `#10b981` / `#4ade80` | Chỉ báo hoàn thành, credit meter, badge thành công |
| Còn lại | Trắng/đen + alpha | Nền, viền, text phụ, hover — không chọn tay |

Quy tắc:

- Nhấn mạnh → dùng accent blue, **không** dùng bold/tăng size.
- Phân cấp thông tin → dùng alpha của `text-primary` vs `text-secondary`.
- **Không tự thêm màu mới** (tím, cam, đỏ…) — nếu cần phải cập nhật bảng token + kiểm tra cả 2 theme.

---

## 4. Typography — một phông, bốn cấp

Duy nhất **Inter**, tối ưu cho HiDPI. Phân cấp qua **font-weight + opacity**, không qua font-size.

| Cấp | size / weight | Dùng cho |
| :--- | :--- | :--- |
| 1 — Header | `14–15px` / `700` | Tiêu đề, tên người dùng |
| 2 — Label | `13–14px` / `500–600` | Nhãn nút, menu, chip |
| 3 — Subtext | `12–13px` / `400` | Mô tả phụ, placeholder |
| 4 — Badge | `10–11px` / `700–800` | Nhãn viết hoa ngắn ("PRO") |

- Viết hoa chỉ cho badge (`letter-spacing 0.3–0.5px`).
- Nhiều dòng: `line-height 1.5–1.6`; không dùng `line-height: 1`.

---

## 5. Spacing — ba cấp khoảng cách

| Cấp | Giá trị | Ý nghĩa |
| :--- | :--- | :--- |
| **Micro** | `4–6px` | Trong cùng 1 phần tử (icon ↔ text) — đọc như một đơn vị |
| **Standard** | `8–12px` | Giữa các phần tử cùng nhóm — thoải mái, rõ cùng nhóm |
| **Section** | `16–20px` | Giữa các nhóm chức năng — tín hiệu "nhóm mới" |

- Khoảng hở tối thiểu giữa 2 surface kính cạnh nhau: `8–12px` (để wallpaper lọt qua, giữ cảm giác floating).
- Không cứng nhắc bội số 8; nhưng hạn chế < 6 giá trị spacing trong 1 component.

---

## 6. Motion — chỉ khi cần thiết

Kiểm tra: **"Bỏ animation này thì giao diện mất gì?"** — mất gì không rõ ràng thì bỏ.

| Loại | Duration | Easing | Dùng cho |
| :--- | :--- | :--- | :--- |
| State transition | `200ms` | `ease` | Hover, active, đổi màu |
| Enter | `200ms` | `ease-out` | Dropdown, popover (slide −8px + fade) |
| Exit | `150ms` | `ease-in` | Dropdown, popover biến mất |
| Genie (panel đóng/mở) | `400–450ms` | custom | Co giãn hướng về nút kích hoạt |
| Skeleton pulse | `1.2s` | loop | Loading vùng nội dung lớn |

**Không có:** parallax, particle, Lottie, 3D transform, bounce, ripple.

---

## 7. States — phản hồi qua màu, không qua hình học

| State | Phản hồi | Không làm |
| :--- | :--- | :--- |
| Hover | Nền sáng thêm `4–8%`, `200ms` | Transform, scale |
| Active/Pressed | Đậm hơn hover, `100ms` | inset shadow |
| Selected/On | Viền accent hoặc nền accent mờ `rgba(59,130,246,0.15)` | Shadow to lên |
| Disabled | Opacity `40–50%`, `pointer-events: none` | Đổi màu nền riêng |

---

## 8. Theming — dark/light bình đẳng

- **Cùng ngôn ngữ, khác cảm giác**: dark chìm/tập trung, light thoáng/sạch — không phải âm bản đảo màu.
- Dark: `rgba(14,14,18,0.96)` — **không dùng đen đặc** (cắt đứt glassmorphism).
- Light: `rgba(255,255,255,0.98)`.
- **Giữ nguyên giữa 2 theme:** accent `#3b82f6`, radius, spacing, font, timing.
- **Thay đổi:** surface, text, border, hover, shadow.

### CSS nằm ở đâu (2 tầng bắt buộc)

```text
libs/babydra-ui-kit/src/styles/shared/   ← cấu trúc & layout (trong binary)
themes/<theme-id>/css/
    dark.css   ← lớp màu dark
    light.css  ← lớp màu light
    theme.css  ← override nạp cuối
```

> [!WARNING]
> Lỗi phổ biến nhất: thêm style màu vào `dark.css` mà quên `light.css` — mọi thay đổi màu phải làm cả 2 file. Luồng nạp theme: [05-themes-variants.md](./05-themes-variants.md).

---

## 9. Bảng token nhanh

| Token | Dark | Light |
| :--- | :--- | :--- |
| `surface` | `rgba(14,14,18,0.96)` | `rgba(255,255,255,0.98)` |
| `border` | `rgba(255,255,255,0.14)` | `rgba(0,0,0,0.08)` |
| `border-top-bevel` | `rgba(255,255,255,0.28)` | `rgba(0,0,0,0.06)` |
| `text-primary` | `rgba(255,255,255,0.95)` | `rgba(28,28,30,0.95)` |
| `text-secondary` | `rgba(255,255,255,0.50)` | `rgba(28,28,30,0.50)` |
| `hover-bg` | `rgba(255,255,255,0.08)` | `rgba(0,0,0,0.05)` |
| `separator` | `rgba(255,255,255,0.10)` | `rgba(0,0,0,0.06)` |
| `shadow` | `0 10px 30px rgba(0,0,0,0.35)` | `0 10px 30px rgba(0,0,0,0.08)` |

| Radius | Giá trị | Spacing | Giá trị |
| :--- | :--- | :--- | :--- |
| `pill` | `9999px` | `micro` | `4–6px` |
| `xl` | `24px` | `standard` | `8–12px` |
| `lg` | `20px` | `section` | `16–20px` |
| `md` | `16px` | — | — |
| `sm` | `10–12px` | — | — |
