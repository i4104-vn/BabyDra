# Component: Wi-Fi Signal Icon

Tài liệu quy định cách sử dụng và mã nguồn chuẩn cho **icon cường độ tín hiệu Wi-Fi** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `kits/babydra-ui-kit/src/components/wifi/mod.rs`

---

## 1. Tổng quan

Component này render icon Wi-Fi dạng **SVG động** với 0–4 vạch sóng, thay đổi theo cường độ tín hiệu và trạng thái kết nối. Màu sắc tự động theo trạng thái:

| Trạng thái | Màu |
| :--- | :--- |
| Tắt (`!is_enabled`) | `#6B7280` (xám) |
| Bật nhưng chưa kết nối | `#9CA3AF` (xám nhạt) |
| Đã kết nối | `#3B82F6` (accent xanh) |

**Số vạch sóng theo cường độ:**

| Signal | Vạch sáng |
| :--- | :--- |
| `≤ 25%` | 1 vạch |
| `≤ 50%` | 2 vạch |
| `≤ 75%` | 3 vạch |
| `> 75%` | 4 vạch |
| Tắt / chưa kết nối | 0 vạch (opacity 0.2) |

---

## 2. API

```rust
pub fn render_wifi_signal_svg(strength_pct: u32, is_enabled: bool, is_connected: bool, size: i32, custom_color: Option<&str>) -> String
pub fn create_wifi_signal_icon_from_strength(strength_pct: u32, is_enabled: bool, is_connected: bool, size: i32, custom_color: Option<&str>) -> gtk4::Widget
pub fn create_wifi_signal_icon_for_network(signal_pct: u32, is_connected: bool, size: i32, custom_color: Option<&str>) -> gtk4::Widget
pub fn create_system_wifi_signal_icon(size: i32, custom_color: Option<&str>) -> gtk4::Widget
```

| Hàm | Mô tả |
| :--- | :--- |
| `render_wifi_signal_svg` | Trả về chuỗi SVG (dùng cho các mục đích không phải widget) |
| `create_wifi_signal_icon_from_strength` | Icon từ dữ liệu tường minh |
| `create_wifi_signal_icon_for_network` | Icon cho một mạng cụ thể (luôn bật) |
| `create_system_wifi_signal_icon` | Icon động — tự query trạng thái Wi-Fi hệ thống qua `babydra_core::services::system::wifi::get_wifi_signal_strength()` |

- `custom_color: Some(...)` → ghi đè màu mặc định.

---

## 3. Ví dụ sử dụng

```rust
// Icon động theo trạng thái hệ thống (panel status)
let icon = create_system_wifi_signal_icon(20, None);

// Icon cho một mạng trong danh sách
let net_icon = create_wifi_signal_icon_for_network(80, true, 20, None);
```

---

## 4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Mọi icon Wi-Fi phải dùng component này — không vẽ SVG thủ công |
| DO | Icon trạng thái hệ thống dùng `create_system_wifi_signal_icon` |
| DO | Icon mạng trong danh sách dùng `create_wifi_signal_icon_for_network` |
| DO NOT | Không tự đổi màu vạch ngoài 3 trạng thái chuẩn (trừ `custom_color` có lý do) |
