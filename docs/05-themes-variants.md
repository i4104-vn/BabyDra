# 05 — Theme và variant

## Phạm vi

Trang này mô tả dữ liệu giao diện runtime và cách variant liên kết theme với cấu hình cài đặt.

## Khái niệm

| Khái niệm | Định nghĩa |
| :--- | :--- |
| Theme package | Một thư mục dưới `themes/<theme-id>` chứa token, font và CSS của giao diện. |
| Variant | Một thư mục dưới `variants/<variant-id>` mô tả theme và lựa chọn ứng dụng cho một profile. |
| Theme selection | Giá trị được ghi vào `~/.babydra/babydra.conf` để app biết theme đang dùng. |

Theme là dữ liệu dùng chung. Variant là lớp lựa chọn. Không đặt logic Rust vào hai thư mục này.

## Cấu trúc theme package

```text
themes/<theme-id>/
├── tokens.json
├── fonts.json
└── css/
    ├── dark.css
    ├── light.css
    └── theme.css
```

`tokens.json` mô tả màu, surface, border, font và radius. `fonts.json` mô tả font family và fallback. `dark.css` và `light.css` chứa lớp màu tương ứng; `theme.css` là lớp override nạp cuối nếu theme cần điều chỉnh riêng.

Ví dụ token tối giản:

```json
{
  "name": "example-dark",
  "base": null,
  "dark": {
    "surface": "rgba(14,14,18,0.96)",
    "border": "rgba(255,255,255,0.14)",
    "accent": "#3b82f6"
  },
  "light": {
    "surface": "rgba(255,255,255,0.98)",
    "border": "rgba(0,0,0,0.08)",
    "accent": "#3b82f6"
  }
}
```

Tên trong `tokens.json` phải khớp `theme-id`. Nếu dùng `base`, theme con chỉ ghi phần khác biệt và engine hợp nhất nó với theme cha.

## Thứ tự resolve theme

```text
app start
  → đọc theme selection
  → tìm theme root theo thứ tự:
      BABYDRA_THEMES_DIR
      ~/.babydra/themes
      /usr/share/babydra/themes
      workspace/themes
  → đọc tokens và CSS
  → resolve base theme
  → tạo CSS theo thứ tự shared → dark/light → theme.css
  → áp dụng GtkCssProvider
```

CSS layout dùng chung được đóng gói trong `babydra-ui-kit`; CSS màu được đọc từ theme package. Không copy CSS màu vào application crate.

## Tạo theme mới

1. Copy một theme đang hoạt động thành `themes/<theme-id>`.
2. Đổi `name` trong `tokens.json` để khớp tên thư mục.
3. Cập nhật cả `dark.css` và `light.css` khi thay đổi màu hoặc state.
4. Kiểm tra font trong `fonts.json` tồn tại trên hệ thống hoặc được khai báo trong package cài đặt.
5. Build và chạy một app GTK để kiểm tra widget phổ biến, modal, popover và disabled state.

Không thêm màu mới chỉ vì một component cần một trạng thái. Trước tiên kiểm tra token hiện có và xác định màu đó có phải semantic token dùng chung hay không.

## Cấu trúc variant

```text
variants/<variant-id>/
└── variant.toml
```

Ví dụ:

```toml
name = "work"
theme = "example-dark"
apps = ["babydra-settings", "babydra-explore"]
```

Installer đọc các file `variant.toml`, hiển thị variant trong wizard, deploy theme đã chọn và ghi lựa chọn vào config. Tên variant và theme được lấy từ dữ liệu; không thêm branch-specific variant vào installer.

## Kiểm tra theme và variant

- `name` trong token khớp thư mục.
- Theme có đủ file cần thiết hoặc có base theme cung cấp file còn thiếu.
- Variant trỏ tới theme tồn tại.
- CSS dark/light không dùng class chỉ tồn tại ở một mode.
- Installer có thể deploy theme khi branch được checkout trong worktree.
