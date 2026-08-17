<p align="center">
  <img src="../logo.png" width="120" height="120" alt="BabyDra logo">
</p>

<h3 align="center">BabyDra — Documentation</h3>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.80+-a5844f?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/GTK4-0.9-blue?style=for-the-badge&logo=gtk&logoColor=white" alt="GTK4">
  <img src="https://img.shields.io/badge/Wayland-labwc-4bc0c0?style=for-the-badge" alt="Wayland">
  <img src="https://img.shields.io/badge/license-Apache_2.0-blue?style=for-the-badge" alt="License">
</p>

<p align="center">Tài liệu kỹ thuật của môi trường desktop BabyDra — viết cho người phát triển, không phải người dùng cuối.</p>

---

## Mục lục

- [Giới thiệu](#giới-thiệu)
- [Bắt đầu từ đâu](#bắt-đầu-từ-đâu)
- [Bản đồ tài liệu](#bản-đồ-tài-liệu)
- [Quy ước viết tài liệu](#quy-ước-viết-tài-liệu)
- [Đóng góp tài liệu](#đóng-góp-tài-liệu)

---

## Giới thiệu

BabyDra là một **môi trường desktop Linux nhẹ** viết bằng Rust + GTK4 trên compositor labwc (Wayland). Tài liệu gồm 10 trang đánh số, mỗi trang một chủ đề lớn — đọc theo thứ tự từ 01 đến 10 là hiểu cả hệ thống.

---

## Bắt đầu từ đâu

```text
01 Tổng quan ─▶ 02 Kiến trúc ─▶ 03 Cài đặt & build
      │
      ▼
04 Cấu trúc & quy chuẩn ─▶ 06 Luồng hoạt động ─▶ 07 Dynamic Island
      │
      ▼
05 Themes & Variants ─▶ 08 API ─▶ 09 Design ─▶ 10 Components
```

| # | Tài liệu | Bạn cần khi… |
| :--- | :--- | :--- |
| 01 | [Tổng quan dự án](./01-overview.md) | Muốn biết BabyDra gồm những gì, mô hình phân nhánh |
| 02 | [Kiến trúc](./02-architecture.md) | Muốn hiểu pattern thiết kế, daemon-client, sơ đồ tổng thể |
| 03 | [Cài đặt & build](./03-setup.md) | Muốn cài đặt, build, chạy thử |
| 04 | [Cấu trúc & quy chuẩn](./04-structure.md) | Muốn biết code nằm ở đâu, viết code mới thế nào |
| 05 | [Themes & Variants](./05-themes-variants.md) | Muốn tạo theme/variant mới (không cần sửa code) |
| 06 | [Luồng hoạt động hệ thống](./06-system-flows.md) | Muốn hiểu ai gọi ai, luồng từng crate |
| 07 | [Dynamic Island](./07-dynamic-island.md) | Muốn dùng/mở rộng island, tạo feature mới |
| 08 | [API Reference](./08-apis.md) | Muốn tra cứu API core, ui-kit, explore |
| 09 | [Ngôn ngữ thiết kế](./09-design.md) | Muốn làm UI đúng phong cách (tokens, motion, states…) |
| 10 | [Component Library](./10-components.md) | Muốn dùng đúng component chuẩn (button, modal, switch…) |

---

## Bản đồ tài liệu

```text
docs/
├── README.md            ← Trang này — mục lục & điểm bắt đầu
├── 01-overview.md       ← Giới thiệu, thành phần, phân nhánh
├── 02-architecture.md   ← Pattern, daemon-client, sơ đồ hệ thống
├── 03-setup.md          ← Cài đặt, build, chạy
├── 04-structure.md      ← Cây thư mục, trách nhiệm module, quy chuẩn code
├── 05-themes-variants.md← Theme packages & variants
├── 06-system-flows.md   ← Luồng hoạt động từng crate/lib
├── 07-dynamic-island.md ← Dynamic Island: dùng + mở rộng
├── 08-apis.md           ← API core, ui-kit, explore
├── 09-design.md         ← Ngôn ngữ thiết kế (tokens, motion, states)
└── 10-components.md     ← Component library (button, modal, switch…)
```

Nguyên tắc tổ chức: **một chủ đề, một trang** — thông tin định nghĩa một lần, nơi khác chỉ link tới (không copy-paste).

---

## Quy ước viết tài liệu

| Quy ước | Chi tiết |
| :--- | :--- |
| Ngôn ngữ | Tiếng Việt; thuật ngữ kỹ thuật giữ tiếng Anh (`struct`, `trait`, `Rc<RefCell<T>>`) |
| Format | Chỉ `README.md` dùng header trình bày (logo/badges); mọi trang khác là tài liệu thuần |
| Diagram | Mermaid cho sơ đồ kiến trúc/luồng lớn; ASCII cho luồng terminal (installer, khởi động) |
| Bảng | Ưu tiên bảng hơn đoạn văn dài — 1 hàng = 1 ý |
| Callout | `> [!NOTE]`, `> [!IMPORTANT]`, `> [!TIP]`, `> [!WARNING]` |
| API docs | Bảng tham số (Tên / Kiểu / Mô tả) + ví dụ code |
| Mỗi file | Bắt đầu bằng metadata: **Phạm vi** / **Phiên bản** / **Cập nhật lần cuối** |

---

## Đóng góp tài liệu

- Sửa tài liệu cùng commit với code — nếu đổi API, đổi luôn docs.
- Thêm nội dung mới: ưu tiên bổ sung vào trang chủ đề tương ứng; chỉ tạo trang mới khi chủ đề thực sự tách biệt.
- Kiểm tra liên kết chéo sau khi đổi tên/di chuyển file.

Xem thêm: [CONTRIBUTING.md](../CONTRIBUTING.md) — quy trình đóng góp.
