# 03 — Cài đặt & build

**Phạm vi:** yêu cầu hệ thống, cài đặt qua installer/script, build từ nguồn.
**Phiên bản:** 2.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## 1. Yêu cầu hệ thống

| Yêu cầu | Giá trị |
| :--- | :--- |
| Hệ điều hành | Arch Linux (hoặc bản tương thích) |
| Trình quản lý gói | `pacman` + `yay` (hoặc trợ giúp AUR tương đương) |
| Compositor | `labwc` (Wayland) |
| Rust toolchain | 1.80.0 trở lên |
| Thư viện GTK | `gtk4` + `gtk4-layer-shell` |

---

## 2. Cài đặt qua bộ cài đặt TUI (khuyến nghị)

### Chạy

Tài liệu sống trên nhánh `main` — nơi bộ cài đặt là **crate độc lập** trong `install/` (không có workspace root). Chạy:

```bash
cd install
cargo run --release
```

Hoặc dùng script kèm theo:

```bash
./install/run.sh
```

### Luồng wizard 10 bước

```text
 1. Welcome & profile ─▶ 2. Chọn branch (nguồn) ─▶ 3. Packages
        │                    │ (release/develop/nhánh đóng góp)   │
        │                    ▼                                   ▼
 6. Configs & Themes ◀── 5. /var/lib bundle ◀── 4. Binaries
        │                                                      
 7. Variant ─▶ 8. Display manager ─▶ 9. Execute ─▶ 10. Summary
                                        │
                                        ├─ nhập mật khẩu sudo (modal che ký tự)
                                        ├─ checkout branch → git pull
                                        ├─ cargo build --release
                                        └─ copy binaries + configs + themes
```

Điểm đáng chú ý:

- **Mật khẩu sudo được hỏi trước**, xác thực 1 lần trước khi thay đổi bất cứ thứ gì — sai quá 3 lần sẽ dừng để tránh khóa tài khoản.
- Chọn **branch** ở bước 2 → installer checkout, pull, build và cài đúng mã của nhánh đó.
- Binaries → `~/.local/bin` (riêng `babydra-greeter` → `/usr/bin`), staging → `/var/lib/babydra`, theme packages → `~/.babydra/themes`.

---

## 3. Cài đặt qua script tự động (nhánh nguồn)

Script không tương tác `scripts/install.sh` nằm ở **nhánh `release`/`develop`** (nơi có mã nguồn) — nhánh `main` chỉ có bộ cài đặt TUI.

```bash
chmod +x ./scripts/install.sh
./scripts/install.sh
```

Script thực hiện toàn bộ: pacman + yay (deps, fonts, AUR tools) → `cargo build --release` → kill tiến trình cũ → copy binaries → config labwc/GTK/kitty/nvim/fastfetch → greetd → .desktop entries → font cache.

---

## 4. Build từ mã nguồn (nhánh `release`/`develop`)

> [!IMPORTANT]
> Mục này yêu cầu **mã nguồn** — clone nhánh `release` (hoặc `develop`). Nhánh `main` không chứa mã nguồn.

```bash
# Release (khuyến nghị — binary nhỏ, chạy nhanh)
cargo build --release --workspace

# Debug (build nhanh, phục vụ dev)
cargo build

# Chỉ build 1 crate
cargo build -p babydra-panel

# Chỉ kiểm tra compile (không sinh binary)
cargo check --workspace

# Format chuẩn
cargo fmt
```

### Chạy từng thành phần

| Thành phần | Lệnh chạy |
| :--- | :--- |
| Panel + Island (nền) | `~/.local/bin/babydra-panel` hoặc trong autostart của labwc |
| Settings | `cargo run -p babydra-settings` |
| Explore | `cargo run -p babydra-explore` |
| Switcher | `cargo run -p babydra-switcher` |
| Screenshot | `cargo run -p babydra-screenshot` |
| Lock | `cargo run -p babydra-lock` |
| Launcher | `cargo run -p babydra-launcher` |
| Preview | `cargo run -p babydra-preview <ảnh>` |
| Greeter | `cargo run -p babydra-greeter` (chạy trong cage bởi greetd) |

> [!TIP]
> Script `scripts/start.sh` cấu hình labwc (autostart, rc.xml, theme, .desktop entries) rồi chạy `labwc` — dùng cho máy đã cài xong.

---

## 5. Kiểm tra an toàn (dành cho developer)

```bash
./scripts/check.sh              # cargo check + fmt --check + clippy -D warnings + test
cargo test --workspace          # toàn bộ test
cargo test -p babydra-tests     # chỉ integration suite (tests/)
```

---

## 6. Vị trí dữ liệu sau khi cài

| Dữ liệu | Vị trí |
| :--- | :--- |
| Binaries người dùng | `~/.local/bin/` |
| Greeter (system) | `/usr/bin/babydra-greeter` |
| Staging hệ thống | `/var/lib/babydra/` (bin, wallpaper, logo) |
| Theme packages | `~/.babydra/themes/` |
| Cấu hình | `~/.babydra/babydra.conf` |
| Config labwc | `~/.config/labwc/` |
| Log panel | `~/.cache/babydra/panel.log` |
