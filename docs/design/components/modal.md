# Component: Modal (Dialog)

Tài liệu quy định cách sử dụng, kiểu dáng, và mã nguồn chuẩn cho **Modal / Dialog** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `libs/babydra-utils/src/components/modal/`

---

## 1. Tổng quan

Modal là cửa sổ nổi (overlay) căn giữa màn hình, dùng cho các tương tác yêu cầu nhập liệu hoặc xác nhận. Mọi modal đều:

- Dùng class `auth-dialog-card` (surface nổi cấp 2, xem [surfaces.md](../surfaces.md)).
- Ẩn sẵn (`set_visible(false)`) và được bật qua `show_for_*()`.
- Nút hành động chính dùng `suggested-action` (xanh), phụ dùng `connect-pill-btn` (xem [buttons.md](./buttons.md)).

| Dialog | File | Công dụng |
| :--- | :--- | :--- |
| `PasswordDialog` | `password_dialog.rs` | Nhập mật khẩu xác thực (PAM...) |
| `WifiPasswordDialog` | `wifi_password_dialog.rs` | Nhập mật khẩu / username kết nối Wi-Fi (hỗ trợ 802.1X) |
| `WifiInfoDialog` | `wifi_info_dialog.rs` | Chi tiết mạng Wi-Fi đã lưu (security, signal, IPv4...) |
| `WifiConfigDialog` | `wifi_config_dialog.rs` | Cấu hình IP (DHCP / Static + DNS) |
| `VpnConfigDialog` | `vpn_config_dialog.rs` | Thêm / sửa / xóa kết nối VPN (nhập file .ovpn, type, gateway...) |
| `VpnLogDialog` | `vpn_log_dialog.rs` | Xem log kết nối VPN (TextView monospace, tô màu log) |

Ngoài ra có helper `create_dialog_box(title, content)` tạo overlay box đơn giản (class `cheatsheet-overlay`).

---

## 2. Mẫu cấu trúc chung

Mọi dialog tuân theo bố cục:

```
┌───────────────────────────────────────┐
│  [icon]  Tiêu đề        (auth-dialog-card)
│          Mô tả phụ                     │
│  ─────────────────────────────────────  │
│  Nhóm nhập liệu (Entry / DropDown)     │
│  ...                                   │
│  ─────────────────────────────────────  │
│  [🗑]               [Cancel] [Confirm] │   ← nút xóa chỉ ở dialog có dữ liệu
└───────────────────────────────────────┘     (WifiInfoDialog, VpnConfigDialog)
```

- Header: icon + `settings-row-title` (13px, 500) + `settings-row-desc` (12px).
- Label trường: `wifi-info-label`; Entry: `sidebar-search-entry`.
- Footer: nút hủy `connect-pill-btn` + nút chính `suggested-action`. Nút xóa (icon tròn `icon-btn circular delete-btn`) chỉ xuất hiện ở các dialog có dữ liệu có thể xóa — `WifiInfoDialog` (Forget) và `VpnConfigDialog` (Delete).

---

## 3. API chính

### 3.1. PasswordDialog

```rust
pub fn new(title: &str, subtitle: &str) -> Self
pub fn show_for(&self, prompt_title: &str, prompt_sub: &str)
pub fn hide(&self)
pub fn connect_submit<F: Fn(Option<String>) + 'static>(&self, callback: F)  // None nếu rỗng
```

### 3.2. WifiPasswordDialog

```rust
pub fn new() -> Self
pub fn show_for(&self, ssid: &str, security: &str)   // "8021x" hiện ô username
pub fn set_error(&self, msg: Option<&str>)
pub fn connect_submit<F: Fn(String, Option<String>) + 'static>(&self, callback: F) // (pwd, user)
```

### 3.3. WifiInfoDialog

```rust
pub fn show_for(&self, net: &WifiNetwork, config: Option<&WifiConfig>)
pub fn connect_configure<F: Fn() + 'static>(&self, callback: F)
pub fn connect_forget<F: Fn() + 'static>(&self, callback: F)
```

- Hiển thị badge trạng thái (`wifi-status-badge` + chấm `wifi-saved-dot` / `wifi-connected-dot`).
- Body chia section: "Wireless Connection" và "IPv4 Configuration" (Grid key–value).

### 3.4. WifiConfigDialog

```rust
pub fn show_for(&self, ssid: &str, cfg: &WifiConfig)
pub fn connect_save<F: Fn(String, WifiConfig) + 'static>(&self, callback: F)
```

- **Segmented control** DHCP/Static: hai nút `seg-btn` / `seg-btn-active` trong container `segmented-control`.

### 3.5. VpnConfigDialog

```rust
pub fn apply_config_file(&self, path: &str)      // parse .ovpn/.conf và tự điền
pub fn show_for_new(&self)
pub fn show_for_edit(&self, details: &VpnConnDetails)
pub fn connect_save<F: Fn(VpnConnDetails) + 'static>(&self, callback: F)
pub fn connect_delete<F: Fn(String) + 'static>(&self, callback: F)
```

- DropDown loại VPN: openvpn, wireguard, l2tp, pptp, openconnect, fortisslvpn, strongswan.
- Có nút Browse để chọn file config / CA cert qua `gtk4::FileDialog`.

### 3.6. VpnLogDialog

```rust
pub fn show_for_vpn(&self, vpn_name: &str)
```

- `TextView` monospace trong `ScrolledWindow` (class `console-log-panel`).
- Tô màu log qua text tags: time `#9ca3af`, WARN `#f59e0b`, ERROR `#ef4444`, INFO `#60a5fa`, LOG `#34d399`.
- Fetch log chạy background thread + poll 100ms.

---

## 4. Style

```css
/* Dark theme — surface dialog */
.auth-dialog-card {
    background-color: rgba(18, 18, 28, 0.98);
    border: 1px solid rgba(255, 255, 255, 0.14);
    border-top: 1px solid rgba(255, 255, 255, 0.22);
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.7);
}
```

> [!IMPORTANT]
> Modal là surface **nổi cấp 2** — shadow lớn hơn card thường và luôn căn giữa màn hình (`Align::Center` cả hai trục).

---

## 5. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Mọi dialog mới phải dùng class `auth-dialog-card` và ẩn sẵn khi khởi tạo |
| DO | Nút chính `suggested-action`, nút phụ `connect-pill-btn` (xem buttons.md) |
| DO | Nhãn trường dùng `wifi-info-label`, entry dùng `sidebar-search-entry` |
| DO | Dialog yêu cầu nhập liệu phải có cả xác nhận lẫn hủy bỏ |
| DO NOT | Không dùng `gtk4::Dialog`/`gtk4::Window` modal riêng — dùng overlay box |
| DO | Chuỗi mới nên đi qua i18n (`babydra_common::i18n::t`) — lưu ý một số dialog hiện tại vẫn hardcode tiếng Anh (Cancel, Confirm...), nên khi sửa hãy chuyển dần sang i18n |
