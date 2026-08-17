# Theming

BabyDra hỗ trợ Dark mode và Light mode như hai trạng thái bình đẳng — không phải "light là mặc định, dark là tùy chọn" hay ngược lại. Hệ thống được thiết kế từ đầu để cả hai đều hoạt động tốt như nhau.

---

## Tư duy dual-theme: không đổi màu, đổi "cảm giác"

Cách tiếp cận sai phổ biến: lấy giao diện dark, đổi `#000` thành `#fff` và gọi là light mode. Cách này tạo ra light mode trông như âm bản của dark mode — cùng cấu trúc nhưng ngược màu, trông lạ và không tự nhiên.

BabyDra tiếp cận khác: **Dark mode và Light mode có cùng ngôn ngữ thị giác nhưng khác cảm giác tổng thể**. Dark mode cảm giác chìm, tập trung, sâu. Light mode cảm giác thoáng, nhẹ, sạch. Cùng một element xuất hiện khác nhau trên hai nền — không phải vì màu sắc đảo ngược mà vì các giá trị được điều chỉnh cho phù hợp với môi trường sáng/tối.

---

## Cách hệ thống quản lý hai theme

CSS được tổ chức thành ba tầng:

```
kits/babydra-ui-kit/src/styles/
    shared/   <- CSS cấu trúc & layout (không phụ thuộc theme)
    dark/     <- CSS màu sắc cho dark mode
    light/    <- CSS màu sắc cho light mode
```

Cấu trúc thư mục con giống nhau giữa ba tầng: `panel/`, `control_center/`, `island/`, `launcher/`, `calendar/`, `apps/`, `explore/`, `shared/` (button, switch, sidebar, scrollbar).

Khi ứng dụng khởi động, `init_theme()` trong `babydra-ui-kit` gộp toàn bộ CSS `shared/` với CSS `dark/` hoặc `light/` (tùy GSettings) rồi nạp vào provider toàn cục. Khi người dùng chuyển theme trong lúc chạy, hệ thống reload CSS và áp dụng ngay.

Điều quan trọng: **mọi thay đổi CSS phải được thực hiện ở đúng tầng của nó**. Style màu sắc phải được cập nhật ở cả `dark/` lẫn `light/`; style cấu trúc (layout, kích thước) chỉ cần đặt ở `shared/`. Thêm style mới vào `dark/` mà quên `light/` là lỗi phổ biến nhất trong dự án.

---

## Những gì thay đổi giữa hai theme

**Thay đổi:**

- Màu nền surface: tối đục ↔ sáng trắng.
- Màu text: trắng với alpha cao ↔ đen với alpha cao.
- Màu viền: trắng mờ ↔ đen mờ.
- Màu hover background: trắng mờ nhạt ↔ đen mờ nhạt.
- Shadow: đậm hơn trên dark ↔ nhẹ hơn trên light.

**Không thay đổi:**

- Màu accent `#3b82f6` — giống nhau trên cả hai theme.
- Border radius — cùng giá trị.
- Spacing — cùng giá trị.
- Font size và weight — cùng giá trị.
- Animation timing — cùng giá trị.

---

## Dark mode không phải là đổi màu nền đen

Dark mode của BabyDra không dùng `#000000` hay `#111111` đặc hoàn toàn làm nền. Nó dùng `rgba(14, 14, 18, 0.96)` — màu tối sẫm nhưng vẫn có kênh alpha, đủ để blur effect hoạt động và tạo cảm giác kính mờ.

Nền đen đặc `#000` không phải dark mode tốt — nó cắt đứt giao diện khỏi wallpaper phía sau và phá vỡ hiệu ứng Glassmorphism.

---

## Light mode không phải là màu trắng thuần túy

Tương tự, light mode dùng `rgba(255, 255, 255, 0.98)` — gần trắng nhưng vẫn bán trong suốt. Trên nền wallpaper sáng, hiệu ứng kính mờ vẫn hoạt động dù nhẹ hơn dark mode nhiều.

Light mode thách thức hơn dark mode để thiết kế vì khoảng tương phản thị giác hẹp hơn — viền mờ hơn, shadow nhẹ hơn. Cần chú ý hơn để đảm bảo phân cấp thông tin vẫn rõ ràng.

---

## Kiểm tra thiết kế mới trên cả hai theme

Khi hoàn thành một phần giao diện, bắt buộc kiểm tra trên cả hai theme trước khi coi là xong. Những vấn đề thường xuất hiện khi chuyển sang light mode:

- Text quá nhạt, khó đọc trên nền sáng.
- Border quá mờ, mất phân cách giữa các vùng.
- Shadow quá nhẹ, bề mặt trông phẳng không nổi.
- Màu accent `#3b82f6` trông khác trên nền sáng so với nền tối (thường vẫn ổn, nhưng cần xác nhận).

---

## Theme packages (Phase 3)

Từ `refactor/constructor` (Phase 3), theme được đóng gói thành **theme package**
trong `themes/<theme-id>/` và nạp qua engine `babydra-theme` (crate thuần logic,
không GTK):

```
themes/<theme-id>/
├── tokens.json        <- design tokens: surface, border, accent, font, radius (dark + light)
├── fonts.json         <- font families + fallbacks
└── css/               <- CSS tách riêng — KHÔNG nằm chung với file JSON
    ├── dark.css       <- lớp màu dark-mode
    ├── light.css      <- lớp màu light-mode
    └── theme.css      <- lớp override (tùy chọn, nạp cuối)
```

- `resolve_theme(id)` hỗ trợ kế thừa `base` (theme con override từng token).
- Giá trị dark/light tương ứng với bảng ở đầu tài liệu này.
- Xem hướng dẫn tạo theme mới: `docs/05-themes-variants.md`.
