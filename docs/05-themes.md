# 05 — Theme

## Phạm vi

Trang này mô tả cấu trúc theme package, cơ chế nạp CSS runtime và cách cấu hình theme trong BabyDra.

## Khái niệm

| Khái niệm | Định nghĩa |
| :--- | :--- |
| Theme package | Một thư mục dưới `assets/themes/<theme-id>` chứa token, font và CSS của giao diện. |
| Theme selection | Khóa cấu hình được lưu trong `~/.babydra/babydra.conf` để ứng dụng xác định theme đang hoạt động. |

Theme là dữ liệu giao diện dùng chung. Không đặt logic Rust vào thư mục theme.

## Cấu trúc theme package

```text
assets/themes/<theme-id>/
├── tokens.json
├── fonts.json
└── css/
    ├── dark.css
    ├── light.css
    └── theme.css
```

`tokens.json` mô tả màu, surface, border, font và radius. `fonts.json` mô tả font family và fallback. `dark.css` và `light.css` chứa lớp màu tương ứng; `theme.css` là lớp override nạp cuối cùng khi theme cần điều chỉnh riêng.

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

Tên trong `tokens.json` phải khớp `theme-id`. Khi sử dụng `base`, theme con chỉ định nghĩa phần khác biệt và engine sẽ hợp nhất với theme cha.

## Thứ tự resolve theme

```text
app start
  → đọc theme selection từ ~/.babydra/babydra.conf
  → tìm theme root theo thứ tự:
      BABYDRA_THEMES_DIR
      ~/.babydra/themes
      /usr/share/babydra/themes
      workspace/assets/themes
      workspace/themes
  → đọc tokens và CSS
  → resolve base theme
  → tạo CSS theo thứ tự shared → dark/light → theme.css
  → áp dụng GtkCssProvider
```

CSS layout dùng chung được đóng gói trong `babydra-ui-kit`; CSS màu và token trực quan được nạp từ theme package. Không sao chép CSS màu vào các crate ứng dụng.

## Tạo theme mới

1. Sao chép một theme đang hoạt động vào `assets/themes/<theme-id>`.
2. Đổi trường `name` trong `tokens.json` khớp tên thư mục.
3. Cập nhật cả `dark.css` và `light.css` khi thay đổi màu hoặc trạng thái widget.
4. Kiểm tra font trong `fonts.json` đã tồn tại trên hệ thống hoặc được khai báo cài đặt trong `workspace.toml`.
5. Build và chạy ứng dụng GTK để kiểm tra widget, modal, popover và trạng thái disabled.

Không thêm màu mới chỉ cho một widget đơn lẻ. Hãy sử dụng semantic token dùng chung từ thiết kế hệ thống.

## Cấu hình theme

Lựa chọn theme được lưu trong `~/.babydra/babydra.conf`:

```toml
[theme.selection]
id = "babydra-default"
```

Khi người dùng chuyển đổi theme trong `babydra-settings`, tệp cấu hình này được cập nhật và các daemon lắng nghe thay đổi để áp dụng lại CSS runtime.

## Kiểm tra theme

- Trường `name` trong token khớp với tên thư mục package.
- Theme có đầy đủ tệp cần thiết hoặc có base theme hợp lệ cung cấp tệp còn thiếu.
- CSS dark và light đồng bộ, không dùng class chỉ tồn tại ở một chế độ màu.
- Bộ cài đặt có thể triển khai theme vào thư mục hệ thống khi chạy cài đặt.
