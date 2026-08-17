<p align="center">
  <img src="../logo.png" width="120" height="120" alt="BabyDra logo">
</p>

<h3 align="center">BabyDra — Documentation</h3>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.80+-de a5844f?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/GTK4-0.9-blue?style=for-the-badge&logo=gtk&logoColor=white" alt="GTK4">
  <img src="https://img.shields.io/badge/Wayland-labwc-4bc0c0?style=for-the-badge" alt="Wayland">
  <img src="https://img.shields.io/badge/license-MIT-green?style=for-the-badge" alt="License">
</p>

<p align="center">Tài liệu kỹ thuật & hướng dẫn của môi trường desktop BabyDra — viết cho người phát triển, không phải người dùng cuối.</p>

---

## Mục lục

- [Giới thiệu](#-giới-thiệu)
- [Bắt đầu từ đâu](#-bắt-đầu-từ-đâu)
- [Cấu trúc tài liệu](#-cấu-trúc-tài-liệu)
- [Quy ước viết tài liệu](#-quy-ước-viết-tài-liệu)
- [Đóng góp tài liệu](#-đóng-góp-tài-liệu)

---

## Giới thiệu

BabyDra là một **môi trường desktop Linux nhẹ** viết bằng Rust + GTK4 trên
compositor labwc (Wayland). Tài liệu này mô tả toàn bộ mã nguồn: kiến trúc,
cấu trúc thư mục, API của từng thư viện, cách cài đặt/build, và các hướng dẫn
mở rộng (theme, variant, Dynamic Island…).

> [!NOTE]
> Kho mã nguồn tách theo mô hình 3 nhánh — chi tiết xem [overview](./overview/index.md).
> Các thư mục `crates/`, `libs/`, `configs/` chỉ nằm trên nhánh `release`.

---

## Bắt đầu từ đâu

Chưa biết gì về BabyDra? Đọc theo thứ tự sau:

1. [Tổng quan dự án](./overview/index.md) — BabyDra là gì, có những thành phần nào.
2. [Kiến trúc](./architecture/index.md) — 4 pattern thiết kế cốt lõi.
3. [Cấu trúc dự án](./structure/index.md) — thư mục nào nằm ở đâu, vì sao.
4. [Cài đặt & build](./setup/index.md) — dựng môi trường phát triển.

Muốn **mở rộng** BabyDra? Đọc:

- [Tạo theme & variant mới](./themes/index.md) — không cần sửa code.
- [Mở rộng Dynamic Island](./guides/island.md) — đăng ký view/feature mới.
- [Tạo island feature mới](./guides/island-features.md) — cấu trúc chuẩn.
- [Island hoạt động như thế nào](./guides/island-internals.md) — kiến trúc runtime & luồng dữ liệu hiện tại.

Muốn hiểu **luồng hoạt động hiện tại** của từng thành phần? Mở:

- [Luồng hoạt động — tổng quan](./flows/index.md) — khởi động DE, daemon-client, bản đồ luồng theo crate/lib.
- [Luồng từng crate/lib](./flows/index.md) — core, ui-kit, theme, island, panel, switcher, settings, explore...

Cần **tra cứu API**? Mở:

- [API babydra-core](./apis/core.md) — services, models, config.
- [API babydra-ui-kit](./apis/ui-kit.md) — widget, theme, icon, animation.
- [API Explore](./apis/explore-kit.md) — file manager components.

---

## Cấu trúc tài liệu

```text
docs/
├── README.md              <- Trang này — mục lục & điểm bắt đầu
├── overview/              <- Tổng quan dự án, thành phần hệ thống
├── architecture/          <- Kiến trúc, pattern, luồng dữ liệu
├── structure/             <- Cấu trúc thư mục, trách nhiệm module, quy chuẩn code
├── setup/                 <- Cài đặt, build, chạy từng crate, xử lý lỗi
├── themes/                <- Theme packages & variants — hướng dẫn mở rộng
├── apis/                  <- API reference của từng thư viện (core, ui-kit, explore)
├── flows/                 <- Luồng hoạt động hiện tại của từng crate/lib (core, panel, island...)
├── guides/                <- Hướng dẫn sử dụng & mở rộng code tái sử dụng (island, island-internals…)
└── design/                <- Ngôn ngữ thiết kế giao diện (glassmorphism, tokens…)
```

| Thư mục | Nội dung | Đối tượng |
| :--- | :--- | :--- |
| `overview/` | BabyDra là gì, các crate, mô hình phân nhánh | Mọi người |
| `architecture/` | 4 pattern, Daemon-Client, luồng dữ liệu | Developer |
| `structure/` | Cây thư mục + trách nhiệm từng module + quy chuẩn viết mã | Developer |
| `setup/` | Yêu cầu hệ thống, dependencies, build, chạy | Developer |
| `themes/` | Tạo theme/variant mới | Người dùng thứ 3 |
| `apis/` | API reference từng thư viện | Developer |
| `flows/` | Luồng hoạt động hiện tại từng crate/lib — ai gọi ai, khi nào | Developer |
| `guides/` | Hướng dẫn sử dụng + mở rộng (island, island-features, island-internals…) | Developer |
| `design/` | Thiết kế giao diện, tokens, motion | Designer + Developer |

---

## Quy ước viết tài liệu

| Quy ước | Chi tiết |
| :--- | :--- |
| Ngôn ngữ | Tiếng Việt; thuật ngữ kỹ thuật giữ tiếng Anh (`struct`, `trait`, `Rc<RefCell<T>>`) |
| Format | Chỉ `docs/README.md` dùng header trình bày (logo/badges/emoji); mọi trang khác là tài liệu thuần |
| Phong cách | Ngắn gọn, thân thiện, coi người đọc quan tâm nhưng chưa biết trước ngữ cảnh |
| Ví dụ | Luôn kèm code snippet; inline thông tin liên quan ngay, link đi chỉ khi cần đọc thêm |
| API docs | Bảng tham số thống nhất (Tên / Kiểu / Bắt buộc / Mô tả) + ví dụ trước |
| Callout | `> [!NOTE]`, `> [!IMPORTANT]`, `> [!TIP]`, `> [!WARNING]` |
| Quy tắc | Section "Quy tắc" dùng bảng DO / DO NOT để agent quét được |

---

## Đóng góp tài liệu

- Sửa tài liệu cùng PR với code — nếu đổi API, đổi luôn docs.
- Thêm tài liệu mới: tạo file trong đúng sub-folder theo domain (không thêm file ở gốc `docs/`).
- Mỗi file bắt đầu bằng metadata: **Phạm vi** / **Phiên bản** / **Cập nhật lần cuối**.
- Kiểm tra liên kết chéo sau khi đổi tên/di chuyển file.

Xem thêm: [CONTRIBUTING.md](../CONTRIBUTING.md) — quy trình đóng góp, [structure](./structure/index.md) — quy chuẩn viết mã.
