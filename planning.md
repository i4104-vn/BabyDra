# Planning: Khắc phục tính đồng nhất & Thiết kế Kiến trúc Linh hoạt cho BabyDra

**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-14
**Phạm vi:** Kế hoạch (a) khắc phục các vấn đề đồng nhất giữa component đã phát hiện, (b) thiết kế lại cấu trúc code trên nhánh `release` để hỗ trợ **nhiều phiên bản (variants) bởi nhiều người dùng khác nhau** với **xung đột tối thiểu**.

---

## Mục lục

- [1. Bối cảnh và Mục tiêu](#1-bối-cảnh-và-mục-tiêu)
- [2. Chẩn đoán hiện trạng (có bằng chứng)](#2-chẩn-đoán-hiện-trạng-có-bằng-chứng)
- [3. Nguyên tắc thiết kế](#3-nguyên-tắc-thiết-kế)
- [4. Kiến trúc mục tiêu (release)](#4-kiến-trúc-mục-tiêu-release)
- [5. Cơ chế hỗ trợ đa phiên bản / đa người dùng](#5-cơ-chế-hỗ-trợ-đa-phiên-bản--đa-người-dùng)
- [6. Ma trận sở hữu file — chống conflict](#6-ma-trận-sở-hữu-file--chống-conflict)
- [7. Kế hoạch triển khai theo Phase](#7-kế-hoạch-triển-khai-theo-phase)
- [8. Quy tắc phát triển & chống conflict](#8-quy-tắc-phát-triển--chống-conflict)
- [9. Rủi ro và Giảm thiểu](#9-rủi-ro-và-giảm-thiểu)
- [10. Timeline ước lượng](#10-timeline-ước-lượng)

---

## 1. Bối cảnh và Mục tiêu

### 1.1. Bối cảnh

Đã tiến hành đánh giá toàn diện (docs ↔ code ↔ CSS ↔ design tokens) và phát hiện 2 nhóm vấn đề:

**Nhóm A — Thiếu đồng nhất giữa các component (code + docs):**
- 3 bản triển khai "row builder" song song (`card::create_item_row`, `list_group::create_list_row`, `explore::items::list_row`).
- 2 hệ switch song song (`CustomSwitch` vẽ Cairo đang dùng + 86 dòng CSS `switch.baby-switch` **không được dùng** — dead CSS).
- ~1/3 component là code chết (chưa crate nào dùng): `alerts`, `spinners`, `progress`, `navbar`, `close_button`, `create_item_row`, `create_status_badge`, `create_dialog_box`.
- 9 "ghost class" — component gắn class nhưng **không tồn tại rule CSS** (`success-text`, `settings-desc`, `settings-title`, `settings-subtitle`, `settings-label`, `settings-item-row`, `sidebar-icon-badge`, `badge-slate`, `baby-button`).
- Class CSS định nghĩa trùng ở 4–5 file (`.settings-card` ở 4 file; `.settings-row-title` ở 5 file).
- 2 phong cách styling khác nhau: component dùng CSS class vs `switch`/`slider` hardcode màu `#3b82f6` trong Cairo.
- Xung đột font: docs nói "Inter duy nhất" nhưng config triển khai dùng **Segoe UI Variable**.
- `tokens.md` mục Badge mô tả kiểu cũ, không khớp component hiện tại.
- **0 test** trong toàn repo.

**Nhóm B — Cấu trúc chưa hỗ trợ đa phiên bản:**
- Không có cơ chế theme/variant — `ThemeConfig` chỉ có 4 field (blur, opacity, border_color, border_width), CSS màu cứng trong source.
- Không có cargo features, không có profile config.
- Mọi thứ (CSS, config mẫu, keybinds) nằm chung một nơi → người làm variant phải sửa thẳng file core → **conflict chắc chắn xảy ra** khi nhiều người cùng phát triển.

### 1.2. Mục tiêu

1. Khắc phục triệt để nhóm A (đồng nhất component).
2. Thiết kế cấu trúc cho phép **nhiều phiên bản** (bản chính thức, bản cộng đồng, bản cá nhân) phát triển **song song** với **conflict tối thiểu**.
3. Mỗi phase đều **giữ cho code build & chạy được** (migration an toàn từng bước).

---

## 2. Chẩn đoán hiện trạng (có bằng chứng)

### 2.1. Sơ đồ kiến trúc hiện tại (`release`)

```
Cargo.toml  (workspace: 4 libs + 8 crates + install)
├── libs/
│   ├── babydra-common/   services, models, i18n, config (babydra.conf)   ← KHÔNG GTK ✓
│   ├── babydra-utils/    components + explore/ (dialogs, context_menu) + ui/ + styles/ (85 CSS)
│   ├── babydra-island/   player, widgets
│   └── babydra-launcher/ search, file_search
├── crates/               panel, switcher, screenshot, lock, greeter, settings, preview, explore
├── configs/              labwc, kitty, nvim, fastfetch, themes
├── scripts (start.sh, update.sh, install.sh)
└── install/              TUI installer
```

### 2.2. Vấn đề cụ thể đã xác minh

| # | Vấn đề | Bằng chứng | Mức |
|---|---|---|---|
| 1 | Row builder trùng lặp ×3 | `create_item_row` (chỉ re-export, không dùng), `create_list_row` (explore dùng), `explore::items::list_row` | 🔴 |
| 2 | CSS `switch.baby-switch` chết | `git grep 'baby-switch' crates/` = rỗng | 🔴 |
| 3 | 5 component chết + 3 helper chết | không có lời gọi trong `crates/` | 🔴 |
| 4 | 9 ghost class | class dùng trong code nhưng 0 occurrence trong `styles/` | 🔴 |
| 5 | `.settings-card` định nghĩa ở 4 file; `.settings-row-title` ở 5 file | `git grep` | 🟡 |
| 6 | Màu hardcode trong Cairo (`switch`, `slider`) | `cr.set_source_rgba(0.23, 0.51, 0.96, 1.0)` | 🟡 |
| 7 | Font không nhất quán | CSS: Inter (5 file) + Segoe UI (3 file); configs: Segoe UI (7 chỗ) | 🟡 |
| 8 | `tokens.md` mục Badge cũ | `#fce7f3` chỉ xuất hiện trong `preview.css` | 🟡 |
| 9 | 0 test | `git ls-tree ... | grep -c tests` = 0 | 🔴 |
| 10 | Không có theme/variant system | `ThemeConfig` 4 field; không features; config tối thiểu | 🔴 |

---

## 3. Nguyên tắc thiết kế

1. **Phân tầng đơn hướng** (Layered): mỗi tầng chỉ được phụ thuộc tầng dưới. `core < ui-kit < theme < apps < variants`.
2. **Mọi thứ người dùng thấy là config/theme-driven**: màu, font, keybind, nhãn, component tùy chọn → đều qua tokens/config, không hardcode.
3. **Một class = một file = một chủ sở hữu** (Single-writer): xóa định nghĩa trùng.
4. **API ổn định làm hợp đồng**: `babydra-core` expose API; đổi API = breaking change (bump version).
5. **Cô lập theo feature**: mỗi app 1 crate, mỗi variant 1 thư mục riêng — người khác không đụng file của nhau.
6. **Migration từng bước**: mỗi phase kết thúc bằng `cargo check --workspace` + app chạy được.
7. **Test là lưới an toàn**: thêm test cho core trước khi refactor.

---

## 4. Kiến trúc mục tiêu (release)

```
BabyDra/  (workspace)
├── Cargo.toml
│
├── libs/
│   ├── babydra-core/        ← (từ babydra-common) — logic thuần, KHÔNG GTK, không CSS.
│   │                          services, models, i18n, config, logger. API = hợp đồng.
│   ├── babydra-ui-kit/      ← (từ babydra-utils, tách explore/) — component dùng chung + CSS CORE.
│   │   ├── components/      ← chỉ builder thuần; bỏ component chết; 1 row builder duy nhất
│   │   ├── ui/              ← theme loader, icon resolver, animation, window helpers
│   │   └── styles/core/     ← CSS cấu trúc (shared) — màu dark/light thuộc themes/ (xem T3.3)
│   ├── babydra-theme/       ← MỚI: engine đọc theme package (tokens.json + CSS layer + fonts)
│   ├── babydra-explore-kit/ ← (từ utils/explore/) — dialogs & context_menu riêng của Explore
│   ├── babydra-island/      ← giữ nguyên
│   └── babydra-launcher/    ← giữ nguyên
│
├── crates/                  ← ứng dụng: chỉ chứa UI + gọi API core/ui-kit/theme
│   └── babydra-<app>/       ← 1 app = 1 crate = 1 owner
│
├── themes/                  ← MỚI: theme packages (điểm mở rộng chính, KHÔNG đụng code)
│   └── <theme-id>/
│       ├── tokens.json      ← design tokens (màu, radius, spacing, font, motion)
│       ├── theme.css        ← lớp màu theme (nạp lên core CSS)
│       ├── fonts.json
│       └── configs/         ← rc.xml, autostart, settings.ini mẫu theo theme
│
├── variants/                ← MỖI variant = 1 thư mục riêng biệt (không đụng nhau)
│   ├── default/             ← variant chính thức
│   └── <user>-<name>/       ← variant cá nhân/cộng đồng
│       ├── variant.toml     ← theme ref, app list, config overrides, keybinds
│       └── overrides/       ← override config/labwc, configs/..., assets
│
├── configs/                 ← config hệ thống "seed" (bản mặc định, giữ)
├── scripts/                 ← install.sh, start.sh, update.sh (gom lại)
├── installer/               ← install/ — TUI installer (mở rộng: chọn variant)
├── docs/
└── planning.md              ← tài liệu này
```

### 4.1. Quy tắc phụ thuộc (chỉ phụ thuộc tầng dưới)

```
variants  ──▶ themes ──▶ ui-kit ──▶ core
                ▲          ▲
crates ─────────┴──────────┴───────┘   (apps phụ thuộc core + ui-kit + theme)
installer ──▶ đọc variants/ + themes/ (không import code apps)
```

- `babydra-core` **không bao giờ** import `gtk4` hay biết đến CSS.
- `ui-kit` import GTK4 + core; **không** biết theme cụ thể (chỉ dùng tokens từ theme engine).
- `theme` đọc tokens → sinh lớp CSS; **không** import component.
- `crates/*` chỉ gọi API public của các libs; không tự dựng component tay.

---

## 5. Cơ chế hỗ trợ đa phiên bản / đa người dùng

### 5.1. Theme packages — điểm mở rộng chính

Mỗi phiên bản khác nhau (giao diện khác nhau) = **1 theme package** trong `themes/`. Người dùng tạo theme mới **không cần sửa 1 dòng code core**:

```jsonc
// themes/babydra-default/tokens.json
{
  "name": "babydra-default",
  "base": null,                  // kế thừa theme khác (vd: "segoe-light")
  "dark": {
    "surface": "rgba(14, 14, 18, 0.96)",
    "border": "rgba(255,255,255,0.14)",
    "accent": "#3b82f6",
    "radius": { "pill": 9999, "lg": 20, "md": 16 },
    "font": "Segoe UI Variable Static Text"
  },
  "light": { ... }
}
```

### 5.2. Variants — cấp độ phiên bản hoàn chỉnh

Một **variant** = một bộ sưu tập quyết định (theme nào, app nào, keybind nào, config nào) — nằm gọn trong 1 thư mục:

```toml
# variants/nguyenvana-dark/variant.toml
name = "nguyenvana-dark"
theme = "babydra-gruvbox"        # theme ref
apps = ["panel", "explore", "settings", "switcher"]   # danh sách app
keybinds = { "A-Tab" = "babydra-switcher", "W-q" = "babydra-launcher" }
config_overrides = { "labwc.rc.margin.gap" = 12 }
```

**Merge thứ tự (override từ phải sang trái):**
```
system defaults < configs/ seed < theme package < variant < ~/.babydra/ (user)
```

### 5.3. Cargo features (optional components)

Thêm features cho các module optional trong `ui-kit`/`explore-kit` để bản nhẹ không compile phần không dùng:

```toml
[features]
default = ["full"]
full = ["explore-kit", "island", "launcher"]
minimal = []          # chỉ panel core
```

> [!NOTE]
> Các app là binary crate riêng — danh sách app cài đặt đã được installer quản lý (bước Binaries trong TUI). Features dùng cho **thành phần trong libs**, không phải cho app.

### 5.4. Mô hình branch cho variant (bổ sung WORKFLOW.md)

```
release  ──(official)──▶  variants/default  (chính thức)
   │
   └── develop ──▶ variant/<user>-<name>   (mỗi user 1 branch variant riêng)
                      │
                      └── merge ngược: chỉ gửi PR phần dùng chung (fix core/ui-kit)
```

- Mỗi user chỉ làm việc trong `variants/<user>-<name>/` + branch riêng → **file không bao giờ chạm nhau**.
- **Chính sách merge variant**: chỉ **variant chính thức** (được tác giả duyệt) được merge vào `release`; variant cộng đồng **giữ trên branch riêng** — không merge vào mainline để tránh tích lũy file lạ.
- Khi user sửa lỗi ở core/ui-kit → gửi **PR riêng** (tách khỏi variant), review bởi tác giả core.

---

## 6. Ma trận sở hữu file — chống conflict

| Thư mục | Chủ sở hữu | Người khác được sửa? | Quy trình |
|---|---|---|---|
| `libs/babydra-core/` | Tác giả core | ❌ Chỉ qua PR + review | Breaking change → bump version |
| `libs/babydra-ui-kit/` | Maintainer UI kit | ❌ Chỉ qua PR + review | Thêm component mới phải có docs + test |
| `libs/babydra-theme/` | Maintainer theme engine | ❌ Chỉ qua PR | API ổn định |
| `crates/babydra-<app>/` | Owner từng app | PR cho app đó | Không đụng app khác |
| `themes/<theme-id>/` | Owner theme đó | PR cho theme đó | Không sửa theme của người khác |
| `variants/<user>-<name>/` | **Chỉ user đó** | ❌ Không ai khác | Mỗi người 1 thư mục = 0 conflict |
| `configs/` (seed) | Tác giả | PR | Thay đổi cấu trúc = breaking |
| `docs/`, `planning.md` | Tác giả | PR | — |
| `Cargo.toml` (workspace) | Tác giả | ❌ | Chỉ đổi khi thêm/xóa crate (hiếm) |
| `installer/` | Tác giả | PR | Mở rộng: chọn variant |

**Quy tắc vàng:** *Không ai được sửa file của người khác trực tiếp; mọi thay đổi qua PR và owner quyết định.*

---

## 7. Kế hoạch triển khai theo Phase

> Mỗi phase kết thúc bằng: `cargo check --workspace` + `cargo fmt --check` + app demo chạy được.

### Phase 0 — Lưới an toàn (2–3 ngày)

**Mục tiêu:** Trước khi refactor, tạo test + CI để phát hiện hồi quy.

- [ ] T0.1 Thêm `#[cfg(test)]` cho các service thuần trong `babydra-common` (volume parse, vpn config parse, wifi sort, config load/save, i18n lookup, `format_size`).
- [ ] T0.2 Thêm script `scripts/check.sh`: `cargo check --workspace && cargo fmt --check && cargo clippy --workspace -- -D warnings`.
- [ ] T0.3 Thêm `.github/workflows/ci.yml` chạy `check.sh` trên push/PR (các nhánh release/develop).
- [ ] T0.4 Chụp screenshot baseline từng app (panel, settings, explore) để đối chiếu visual sau refactor.
- [ ] T0.5 Rà soát `.gitignore` (thêm `target/`, `*.log` nếu thiếu).

**Định nghĩa hoàn thành:** `cargo test --workspace` xanh; CI chạy được.

---

### Phase 1 — CSS & Tokens đồng nhất (3–5 ngày)

**Mục tiêu:** Xóa ghost class, xóa định nghĩa trùng, thống nhất font, đưa màu Cairo về tokens.

- [ ] T1.1 Định nghĩa đủ **9 ghost class** đúng file chủ quyền (chủ yếu `styles/shared/apps/settings.css`):
  `success-text`, `settings-desc`, `settings-title`, `settings-subtitle`, `settings-label`, `settings-item-row`, `sidebar-icon-badge`, `badge-slate`, `baby-button` (+ dark/light nếu là lớp màu).
- [ ] T1.2 **Hợp nhất định nghĩa trùng**: giữ 1 định nghĩa mỗi class mỗi tầng. `.settings-card`, `.settings-row-title`, `.settings-row-desc`, `.settings-card-row` → gom vào `settings.css`, **xóa bản trùng trong `explore/dialogs.css`** (kiểm tra không lệch giá trị trước khi xóa).
- [ ] T1.3 Quyết định font: **Segoe UI Variable làm chuẩn** (đang là deployment thực tế). Cập nhật `typography.md`, CSS dùng Inter → Segoe UI (giữ Inter làm `font-family` fallback). Đồng bộ `tokens.md`.
- [ ] T1.4 Đưa màu hardcode trong `switch`/`slider` (Cairo) về hằng số tokens: thêm module `ui-kit/src/ui/theme/colors.rs` (hằng `ACCENT_RGB`, `SURFACE_TRACK_DARK/LIGHT`...) và dùng chung cho cả Cairo lẫn CSS generation.
- [ ] T1.5 Cập nhật `tokens.md` mục Badge + bảng surface cho khớp component thực tế; sửa 2 ví dụ sai trong `tooltips.md`, `popovers.md` (signature `create_icon_button` 5 tham số; class `status-good` → class thật).

**Định nghĩa hoàn thành:** `grep -c` mỗi ghost class ≥ 1 trong CSS; `.settings-card` chỉ còn 1 file mỗi tầng; app render đúng (đối chiếu screenshot baseline).

---

### Phase 2 — Component Cleanup (3–5 ngày)

**Mục tiêu:** Hết trùng lặp, hết dead code; mỗi khái niệm 1 component.

- [ ] T2.1 **Row builder duy nhất**: giữ `list_group::create_list_row` (đang được dùng). **Đánh dấu `#[deprecated]`** `card::create_item_row` (nhất quán với chính sách T2.3 — chuyển caller nếu có sang `create_list_row`, xóa hẳn ở Phase 3). Xóa hoặc gộp `explore::items::list_row` nếu trùng chức năng (nếu có hành vi riêng → tách class riêng, không trùng API).
- [ ] T2.2 **Một hệ switch**: giữ `CustomSwitch` (đang dùng). Xóa CSS `switch.baby-switch` chết (86 dòng) hoặc đánh dấu `@deprecated` rồi xóa ở phase sau.
- [ ] T2.3 **Xử lý dead components** (chọn 1 trong 2):
  - (a) Xóa: `alerts`, `spinners`, `progress`, `navbar`, `close_button`, `create_status_badge`, `create_dialog_box` — nếu chắc chắn không dùng.
  - (b) Giữ + đánh dấu `#[deprecated(note = "unused, remove in v2")]` để quyết định sau.
  - **Khuyến nghị (b)** — tránh phá API đang có người dùng; dọn dứt điểm ở Phase 3 cùng lúc tách libs.
- [ ] T2.4 **Gộp `alerts` vào `placeholder`**: chuyển `create_placeholder_message` sang module placeholder (hoặc xóa nếu trùng `create_placeholder_row`), xóa module `alerts`.
- [ ] T2.5 Thêm test component nhẹ: unit test cho `CustomSwitch` state machine, `CustomSlider` clamp/step, `parse_vpn_config_file` (nếu chưa có).

**Định nghĩa hoàn thành:** `cargo test` xanh; grep không còn `create_item_row`/`baby-switch`; docs component phản ánh đúng số component sống.

---

### Phase 3 — Tái cấu trúc libs + Theme/Variant engine (2–3 tuần — phase lớn nhất)

**Mục tiêu:** Đưa code về kiến trúc mục tiêu (mục 4).

- [ ] T3.1 **Tách `babydra-utils`** thành `ui-kit` + `explore-kit`:
  - Di chuyển `src/explore/` → lib mới `babydra-explore-kit` (dialogs, context_menu).
  - `components/`, `ui/`, `styles/` → `babydra-ui-kit`.
  - Cập nhật `Cargo.toml` workspace + dependency của `babydra-explore` crate.
- [ ] T3.2 **Tạo `babydra-theme`**:
  - `ThemePackage` load: đọc `themes/<id>/tokens.json` + `theme.css` + `fonts.json`.
  - `resolve_theme(id)` → `ThemeValue { tokens, css_layers, fonts }`.
  - Thay thế `init_theme()` cứng: `init_theme_with(theme_id)`; giữ `init_theme()` = theme mặc định (backward compatible).
  - `ThemeConfig` trong core: mở rộng thành `ThemeSelection { id, dark: bool }` — bỏ field màu cứng hoặc deprecate. **Mọi field mới phải có `#[serde(default)]`** để file `~/.babydra/babydra.conf` cũ vẫn load được bình thường.
- [ ] T3.3 **Tạo cây `themes/`**:
  - Di chuyển `libs/babydra-utils/src/styles/dark|light/` → `themes/babydra-default/{theme.css dark|light}` (giữ `styles/core/` = shared).
  - Thêm `tokens.json` khởi đầu cho `babydra-default` (lấy giá trị từ tokens.md + CSS hiện tại).
  - Tạo 1 theme mẫu thứ hai (vd `babydra-blue`) để chứng minh cơ chế hoạt động (đổi accent + radius) — **test sống** cho hệ thống.
- [ ] T3.4 **Tạo cây `variants/`**:
  - `variants/default/` (variant hiện tại: theme default, đủ 8 app).
  - Thêm module `babydra-core/src/config/variant.rs`: `load_variant(name)` merge theo thứ tự mục 5.2.
  - Installer: thêm bước chọn variant (đọc `variants/*/variant.toml`) → thay đổi nhỏ ở `installer`.
  - **Lưu ý đồng bộ nhánh `main`**: `install/` tồn tại trên cả `release` lẫn `main` (main là kênh phân phối). Mọi thay đổi installer ở Phase 3 phải **mirror sang `main`** cùng phiên bản, nếu không hub sẽ phân phối installer cũ.
- [ ] T3.5 **Cargo features** cho ui-kit/explore-kit (mục 5.3).
- [ ] T3.6 **Gom scripts**: `start.sh`, `update.sh`, `install.sh` → `scripts/` (giữ file gốc 1 phase để không phá autostart tham chiếu cũ).

**Định nghĩa hoàn thành:** `cargo build --release --workspace` xanh; panel/settings render bằng theme engine; đổi theme bằng 1 dòng config; installer chọn được variant.

---

### Phase 4 — Workflow & Quy tắc chống conflict (1 tuần)

- [ ] T4.1 Cập nhật `WORKFLOW.md`: mô hình branch mở rộng (mục 5.4), ma trận sở hữu (mục 6), quy trình PR cho variant.
- [ ] T4.2 Thêm `CHANGELOG.md` + chính sách version (semver: core/ui-kit là API public).
- [ ] T4.3 Template PR (`CONTRIBUTING.md`): checklist — chạy `scripts/check.sh`, không sửa file ngoài ownership, thêm test khi đổi core.
- [ ] T4.4 Bảo vệ nhánh: `release`, `develop` bắt buộc CI + review (nếu dùng GitHub).

---

### Phase 5 — Docs & Đóng gói (2–3 ngày)

- [ ] T5.1 Cập nhật `docs/03-project-structure.md` theo cấu trúc mới.
- [ ] T5.2 Viết `docs/05-themes-variants.md`: cách tạo theme mới, cách tạo variant mới (hướng dẫn từng bước cho người dùng thứ 3).
- [ ] T5.3 Cập nhật docs design: `theming.md` mô tả cơ chế theme package; `tokens.md` làm schema mẫu cho `tokens.json`.
- [ ] T5.4 Đồng bộ `planning.md` → ghi chú phase đã hoàn thành.

---

## 8. Quy tắc phát triển & chống conflict

1. **Luôn làm việc trong file thuộc ownership của mình** (mục 6).
2. **Không sửa workspace `Cargo.toml`** trừ khi thêm/xóa crate — đây là file dễ conflict nhất.
3. **Mọi thay đổi `core`/`ui-kit` qua PR** + CI xanh + review bởi owner.
4. **Thêm component mới phải kèm**: docs (`docs/design/components/<name>.md`), test cơ bản, CSS đúng file chủ quyền — không ghost class.
5. **Không hardcode màu/font/chuỗi** trong app — đi qua tokens/config/i18n.
6. **CSS: 1 class = 1 file mỗi tầng** — trước khi thêm class mới, grep xem đã tồn tại chưa.
7. **Rebase thay vì merge** cho branch cá nhân (giữ lịch sử tuyến tính — đã có trong WORKFLOW).
8. **Mỗi phase commit riêng, gọn, message chuẩn** — dễ revert khi cần.

---

## 9. Rủi ro và Giảm thiểu

| Rủi ro | Mức | Giảm thiểu |
|---|---|---|
| Refactor lớn (Phase 3) gây hồi quy visual | Cao | Baseline screenshot (T0.4); từng bước nhỏ; đối chiếu sau mỗi T3.x |
| Đổi API `babydra-common`/`utils` làm vỡ app | Cao | Giữ `init_theme()` backward-compatible; deprecate thay vì xóa đột ngột; CI |
| Xóa nhầm component đang dùng | TB | Kiểm tra `git grep` trước khi xóa; Phase 2 dùng `#[deprecated]` trước |
| Nhiều người cùng sửa `styles/` trong quá trình chuyển | TB | Chuyển dứt điểm theo phase; tạm "đóng băng" styles trong Phase 3 |
| Theme engine phức tạp quá mức | TB | Bắt đầu đơn giản: tokens.json + 1 lớp CSS; mở rộng sau |
| Thiếu test cho GTK code (khó test UI) | TB | Tập trung test logic thuần (core); UI test thủ công + screenshot |

---

## 10. Timeline ước lượng

| Phase | Nội dung | Thời gian | Dependency |
|---|---|---|---|
| 0 | Lưới an toàn (test + CI + baseline) | 2–3 ngày | — |
| 1 | CSS & tokens đồng nhất | 3–5 ngày | Phase 0 |
| 2 | Component cleanup (dedup + dead code) | 3–5 ngày | Phase 0, 1 |
| 3 | Tái cấu trúc libs + Theme/Variant engine | 2–3 tuần | Phase 1, 2 |
| 4 | Workflow & quy tắc chống conflict | 1 tuần | Phase 3 |
| 5 | Docs & đóng gói | 2–3 ngày | Phase 3, 4 |
| **Tổng** | | **~4–5 tuần** | |

**Gợi ý triển khai:** Bắt đầu Phase 0 + 1 ngay (an toàn, lợi ích tức thì). Phase 3 nên làm sau khi Phase 1–2 ổn định — tránh vừa đổi cấu trúc vừa dọn nợ kỹ thuật cùng lúc.
