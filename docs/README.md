# BabyDra — Mục lục Tài liệu

**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-07-23
**Phạm vi:** Toàn bộ tài liệu kỹ thuật và thiết kế của dự án BabyDra

---

## Giới thiệu

Đây là trang chủ của hệ thống tài liệu BabyDra. Mỗi tài liệu được tổ chức thành các chương độc lập, mỗi chương tập trung vào một chủ đề cụ thể. Bạn có thể đọc từng chương riêng lẻ mà không cần đọc theo thứ tự, tuy nhiên nếu bạn là developer mới, nên bắt đầu từ Chương 01.

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
| 01 | [01-overview.md](./01-overview.md) | Giới thiệu tổng quan: BabyDra là gì, các thành phần, mục tiêu |
| 02 | [02-architecture.md](./02-architecture.md) | Kiến trúc mã nguồn: 4 pattern thiết kế, luồng dữ liệu, Daemon-Client |
| 03 | [03-project-structure.md](./03-project-structure.md) | Quy chuẩn đặt tên thư mục, phân tách file, quy tắc viết mã |
| 04 | [04-setup-and-build.md](./04-setup-and-build.md) | Hướng dẫn cài đặt, build và chạy dự án từ đầu |

### Tài liệu Thiết kế Giao diện

| Số thứ tự | File | Mô tả ngắn |
| :--- | :--- | :--- |
| D01 | [design/01-design-philosophy.md](./design/01-design-philosophy.md) | Triết lý thiết kế: Glassmorphism, quy tắc hover, animation |
| D02 | [design/02-design-tokens.md](./design/02-design-tokens.md) | Bảng token: màu sắc, typography, border-radius, spacing, shadow |

### Hướng dẫn Từng Component Giao diện

Xem [design/components/README.md](./design/components/README.md) để xem bảng mục lục đầy đủ các component.

| Component | File | Mô tả ngắn |
| :--- | :--- | :--- |
| Badge và Chip | [design/components/badge.md](./design/components/badge.md) | Badge PRO, keyword chip, floating icon badge |
| Buttons | [design/components/buttons.md](./design/components/buttons.md) | Nút Primary, Share Pill, Upgrade, Icon Button |
| Cards | [design/components/card.md](./design/components/card.md) | Khung preview, popover, variation, reference, input shell |
| Dropdowns | [design/components/dropdowns.md](./design/components/dropdowns.md) | Menu popover, profile header, menu rows, badge PRO |
| Input Group | [design/components/input_group.md](./design/components/input_group.md) | Khung nhập liệu nổi và toolbar thông số |
| Navbar | [design/components/navbar.md](./design/components/navbar.md) | Header: logo, nav pill, nhóm tiện ích, avatar |
| Navs và Tabs | [design/components/navs_tabs.md](./design/components/navs_tabs.md) | Center nav pill, session sidebar |
| Preview Panel | [design/components/preview_panel.md](./design/components/preview_panel.md) | Vùng ảnh chính và cột 4 biến thể |
| Progress | [design/components/progress.md](./design/components/progress.md) | Vòng credit meter |
| Prompt Panel | [design/components/prompt_panel.md](./design/components/prompt_panel.md) | Panel bên trái: ảnh tham chiếu, keyword chips, văn bản mô tả |
| Spinners | [design/components/spinners.md](./design/components/spinners.md) | Skeleton pulse và button spinner |
| Tooltips | [design/components/tooltips.md](./design/components/tooltips.md) | Tooltip, cơ chế hiển thị, style |

---

## Quy ước Ký hiệu Trong Tài liệu

Các tài liệu sử dụng bảng markdown thuần túy và các quy ước sau:

- **NOTE:** Thông tin bổ sung giúp hiểu rõ hơn.
- **IMPORTANT:** Yêu cầu bắt buộc phải tuân theo.
- **DO:** Việc cần làm, mẫu tốt.
- **DO NOT:** Việc cấm làm, mẫu xấu.

---

## Liên kết Nhanh đến Mã nguồn

| Thư mục | Đường dẫn | Mô tả |
| :--- | :--- | :--- |
| CSS dùng chung | `libs/babydra-utils/src/styles/` | Toàn bộ CSS của hệ thống |
| Logic lõi | `libs/babydra-common/src/` | Services, models, dữ liệu hệ thống |
| Components UI | `libs/babydra-utils/src/components/` | Widget dùng chung |
| Ứng dụng | `crates/` | Các crate có thể thực thi |
