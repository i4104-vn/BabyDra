# Tài liệu BabyDra

Đây là bộ tài liệu dành cho người phát triển và người bảo trì BabyDra. Tài liệu mô tả kiến trúc hiện tại, cách build, cách mở rộng giữa branch nguồn với bộ cài đặt.

## Đọc theo mục tiêu

| Mục tiêu | Tài liệu |
| :--- | :--- |
| Hiểu dự án và mô hình branch | [01 — Tổng quan](01-overview.md) |
| Hiểu cách các module giao tiếp | [02 — Kiến trúc](02-architecture.md) và [06 — Luồng hệ thống](06-system-flows.md) |
| Cài đặt hoặc build từ nguồn | [03 — Cài đặt và build](03-setup.md) |
| Tìm đúng vị trí để sửa code | [04 — Cấu trúc dự án](04-structure.md) |
| Thêm theme mới | [05 — Theme](05-themes.md) |
| Thêm feature cho Dynamic Island | [07 — Dynamic Island](07-dynamic-island.md) |
| Tra cứu service hoặc widget | [08 — API](08-apis.md) và [10 — Component library](10-components.md) |
| Thiết kế giao diện | [09 — Ngôn ngữ thiết kế](09-design.md) |

## Bản đồ tài liệu

```text
docs/
├── README.md                 Mục lục và quy tắc đọc tài liệu
├── 01-overview.md            Phạm vi dự án và mô hình branch
├── 02-architecture.md        Ranh giới module và dependency direction
├── 03-setup.md               Cài đặt, build và workspace.toml
├── 04-structure.md           Cây thư mục và quy tắc phát triển
├── 05-themes.md              Theme package và cấu hình giao diện
├── 06-system-flows.md        Luồng khởi động, runtime và installer
├── 07-dynamic-island.md      Runtime Dynamic Island
├── 08-apis.md                API các thư viện dùng chung
├── 09-design.md              Ngôn ngữ thiết kế
└── 10-components.md          Component GTK4 dùng chung
```

## Nguyên tắc cập nhật

Tài liệu ở `main` mô tả cơ chế tổng quát. Không thêm vào đó danh sách binary, package hoặc cấu hình chỉ đúng với một branch nguồn. Những dữ liệu này phải nằm trong `workspace.toml` của branch chứa mã nguồn.