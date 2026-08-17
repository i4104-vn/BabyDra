# BabyDra — Mục lục Tài liệu

**Phiên bản:** 1.2.0
**Cập nhật lần cuối:** 2026-08-17
**Phạm vi:** Toàn bộ tài liệu kỹ thuật và thiết kế của dự án BabyDra

---

## Giới thiệu

Đây là trang chủ của hệ thống tài liệu BabyDra. Mỗi tài liệu được tổ chức thành các chương độc lập, mỗi chương tập trung vào một chủ đề cụ thể. Bạn có thể đọc từng chương riêng lẻ mà không cần đọc theo thứ tự, tuy nhiên nếu bạn là developer mới, nên bắt đầu từ Chương 01.

> [!NOTE]
> Kho mã nguồn BabyDra được phân tách theo mô hình 3 nhánh (xem [WORKFLOW.md](../WORKFLOW.md)):
> - **`main`** — kênh phân phối: chỉ chứa bộ cài đặt TUI (`install/`), script thực thi và tài liệu.
> - **`release`** — mã nguồn chính thức: toàn bộ ứng dụng (`crates/`), thư viện (`libs/`) và cấu hình (`configs/`).
> - **`develop`** — nền tảng phát triển cộng đồng, được checkout từ `release`.

Quy tắc chung khi đọc tài liệu này:

- Các thuật ngữ kỹ thuật giữ nguyên tiếng Anh (ví dụ: `struct`, `trait`, `Rc<RefCell<T>>`, CSS property).
- Các giải thích, mô tả, quy tắc dùng tiếng Việt.
- Mỗi khái niệm mới sẽ được định nghĩa trước khi sử dụng.
- Mỗi section "Quy tắc bắt buộc" chứa các quy tắc mà agent có thể quét và áp dụng trực tiếp.

---

## Mục lục

### Tài liệu Kỹ thuật Chính

| Số thứ tự | File | Mô tả ngắn |
| :--- | :--- | :--- |
| 01 | [01-overview.md](./01-overview.md) | Tổng quan: BabyDra là gì, các thành phần hệ thống, mô hình phân nhánh |
| 02 | [02-architecture.md](./02-architecture.md) | Kiến trúc mã nguồn: 4 pattern thiết kế, luồng dữ liệu, Daemon-Client |
| 03 | [03-project-structure.md](./03-project-structure.md) | Cấu trúc dự án: thư mục, trách nhiệm từng module, quy chuẩn viết mã |
| 04 | [04-setup-and-build.md](./04-setup-and-build.md) | Hướng dẫn cài đặt, build và chạy dự án từ đầu |
| 05 | [05-themes-variants.md](./05-themes-variants.md) | Themes & variants — cách tạo theme/variant mới |
| 06 | [06-kits-api.md](./06-kits-api.md) | Tổng hợp API của các kits (prelude, bắt đầu nhanh) |
| 07 | [07-codebase-report.md](./07-codebase-report.md) | Báo cáo đánh giá toàn diện mã nguồn — điểm mạnh, điểm yếu, chỉ số kỹ thuật |

### Kế hoạch & Đánh giá

| Tài liệu | Nội dung |
| :--- | :--- |
| [planning.md](./planning.md) | Kế hoạch cải thiện chi tiết theo phase (test-first) — bắt nguồn từ báo cáo 07 |

### Tài liệu Thiết kế Giao diện

Toàn bộ tài liệu thiết kế nằm trong thư mục [design/](./design/README.md), bao gồm:

| File | Mô tả ngắn |
| :--- | :--- |
| [design/README.md](./design/README.md) | Trang chủ hướng dẫn thiết kế — thứ tự đọc đề xuất |
| [design/visual-language.md](./design/visual-language.md) | Ngôn ngữ thị giác Glassmorphism — cảm giác và tư duy đằng sau |
| [design/surfaces.md](./design/surfaces.md) | Cách mọi bề mặt UI được xây dựng — cấu trúc và lớp nền |
| [design/color.md](./design/color.md) | Triết lý màu sắc — tại sao chọn từng màu, cách dùng đúng |
| [design/typography.md](./design/typography.md) | Phông chữ, phân cấp, và lý do đằng sau |
| [design/states.md](./design/states.md) | Trạng thái tương tác — hover, active, disabled |
| [design/motion.md](./design/motion.md) | Triết lý chuyển động — khi nào animate, khi nào không |
| [design/spacing.md](./design/spacing.md) | Không gian và khoảng cách — nguyên tắc và cách áp dụng |
| [design/theming.md](./design/theming.md) | Dark/Light theming — cách tiếp cận dual-theme |
| [design/tokens.md](./design/tokens.md) | Bảng tra cứu giá trị — màu, font-size, radius, spacing, shadow |

### API Reference của Kits

| Tài liệu | Nội dung |
| :--- | :--- |
| [kit-apis/ui-kit.md](./kit-apis/ui-kit.md) | API `babydra-ui-kit` — components, theme, icons, animation, battery, window |
| [kit-apis/explore-kit.md](./kit-apis/explore-kit.md) | API `babydra-explore-kit` — dialogs, context menu, drag & drop, file items |
| [06-kits-api.md](./06-kits-api.md) | Tổng hợp 2 kits, cách dùng prelude, bắt đầu nhanh |
| [core-api.md](./core-api.md) | API `babydra-core` — services, models, config, error handling |

**Components Giao diện** (thư mục `design/components/`) — mỗi component trong `kits/babydra-ui-kit/src/components/` có một tài liệu riêng:

| Tài liệu | Nội dung |
| :--- | :--- |
| [design/components/buttons.md](./design/components/buttons.md) | Nút bấm: Primary, Secondary Pill, Icon Button, Tile |
| [design/components/badge.md](./design/components/badge.md) | Badge trạng thái & icon badge tròn kính mờ |
| [design/components/card.md](./design/components/card.md) | Card kính mờ, switch card, danh sách cuộn |
| [design/components/switch.md](./design/components/switch.md) | CustomSwitch (Cairo, 160ms) & ToggleRow |
| [design/components/slider.md](./design/components/slider.md) | CustomSlider: range, step, tick marks |
| [design/components/modal.md](./design/components/modal.md) | Dialog: password, wifi, vpn (config/log) |
| [design/components/popovers.md](./design/components/popovers.md) | Popover chuẩn & Hover Popover |
| [design/components/navbar.md](./design/components/navbar.md) | Navigation row cho Sidebar |
| [design/components/list_group.md](./design/components/list_group.md) | List row chuẩn & helper danh sách |
| [design/components/placeholder.md](./design/components/placeholder.md) | Placeholder: Disabled / Loading / Empty |
| [design/components/progress.md](./design/components/progress.md) | Progress bar & disk progress |
| [design/components/spinners.md](./design/components/spinners.md) | Spinner & loading box |
| [design/components/tooltips.md](./design/components/tooltips.md) | Helper tooltip thống nhất |
| [design/components/close_button.md](./design/components/close_button.md) | Nút đóng icon / icon + nhãn |
| [design/components/alerts.md](./design/components/alerts.md) | Thông báo chiếm chỗ (placeholder message) |
| [design/components/wifi.md](./design/components/wifi.md) | Icon cường độ tín hiệu Wi-Fi (SVG 0–4 vạch) |

---

## Quy ước Ký Hiệu Trong Tài Liệu

Các tài liệu sử dụng bảng markdown thuần túy và các quy ước sau:

- **NOTE:** Thông tin bổ sung giúp hiểu rõ hơn.
- **IMPORTANT:** Yêu cầu bắt buộc phải tuân theo.
- **DO:** Việc cần làm, mẫu tốt.
- **DO NOT:** Việc cấm làm, mẫu xấu.

---

## Liên kết Nhanh đến Mã nguồn

> [!IMPORTANT]
> `crates/`, `libs/` và `configs/` **không tồn tại trên nhánh `main`** — chúng nằm trên nhánh `release` (và `develop`). Nhánh `main` chỉ chứa `install/` (bộ cài đặt) và tài liệu.

| Thành phần | Đường dẫn | Mô tả |
| :--- | :--- | :--- |
| Bộ cài đặt TUI | `install/` (nhánh `main`) | `babydra-installer`: wizard 8 bước, 3 kênh cài đặt, 3 preset |
| Ứng dụng đồ họa | `crates/` (nhánh `release`) | 8 ứng dụng: panel, switcher, screenshot, lock, greeter, settings, preview, explore |
| Logic lõi | `libs/babydra-core/` (nhánh `release`) | Services, models, i18n, D-Bus, sysfs |
| Widget & CSS dùng chung | `kits/babydra-ui-kit/` (nhánh `release`) | Components, styles (dark/light/shared), theme, icon, animation |
| Dynamic Island | `libs/babydra-island/` (nhánh `release`) | Thông báo, overlay âm lượng/độ sáng, media player |
| Launcher | `crates/babydra-launcher/` (nhánh `release`) | Tìm kiếm mờ (fuzzy), lưới ứng dụng, tìm file |
| Cấu hình hệ thống | `configs/` (nhánh `release`) | labwc, kitty, nvim, fastfetch, themes |
| Script khởi động | `start.sh`, `update.sh` (nhánh `release`) | Khởi động DE, hot-update & reload |
| Quy trình phân nhánh | [WORKFLOW.md](../WORKFLOW.md) | Quy chuẩn phân nhánh và phát triển |
