# Changelog

Chính sách phiên bản: **SemVer** — `babydra-common`, `babydra-utils` (ui-kit),
`babydra-theme`, `babydra-explore-kit` là **API public**; thay đổi API không
tương thích = bump major.

Định dạng theo [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

### Added
- **Tests (TDD safety net)**: thư mục `tests/` cấp workspace (integration per
  area), unit test `#[cfg(test)]` cho pure logic trong `babydra-common`
  (volume profile parse, vpn config parse, wifi sort, config serde, i18n lookup,
  storage format) và `babydra-utils` (colors tokens, switch easing, slider
  clamp/step).
- `scripts/check.sh` — một lệnh chạy check + fmt + clippy + test.
- CI workflow `.github/workflows/ci.yml` chạy trên push/PR.
- **Theme engine**: crate `babydra-theme` đọc `themes/<id>/{tokens.json,
  theme.css, fonts.json}`, hỗ trợ kế thừa `base`, phát hiện vòng lặp kế thừa.
- **Themes tree**: `themes/babydra-default` + theme mẫu `themes/babydra-blue`
  (chứng minh cơ chế override accent/radius).
- **Variant system**: `variants/*/variant.toml` + module
  `babydra-common::config::variant` (`load_variant`, `list_variants`, keybinds,
  config overrides).
- Cargo features `full`/`minimal` cho `babydra-utils` và
  `babydra-explore-kit` (deprecated-components có thể tắt ở build nhẹ).
- Chia tách `babydra-explore-kit` khỏi `babydra-utils`.

### Changed
- **CSS/tokens**: định nghĩa 9 ghost class (`success-text`, `settings-title`,
  `settings-subtitle`, `settings-label`, `settings-desc`, `settings-item-row`,
  `sidebar-icon-badge`, `badge-slate`, `baby-button`) trong đúng file chủ quyền;
  dedupe `.settings-card`/`.settings-row-title`/`.settings-row-desc`/
  `.settings-card-row` (1 class = 1 file mỗi tầng, dialogs scoped
  `.explore-dialog`).
- Font chuẩn thống nhất **Segoe UI Variable Static Text** (Inter giữ làm
  fallback) — khớp deployment thực tế.
- Màu Cairo của `CustomSwitch`/`CustomSlider` về hằng số tokens
  `babydra_utils::ui::theme::colors` (thay hardcode `0.23, 0.51, 0.96`).
- Xóa CSS `switch.baby-switch` chết (86 dòng).
- Gộp module `alerts` vào `placeholder`; deprecate các builder chưa dùng
  (`create_item_row`, `create_status_badge`, `create_dialog_box`, `spinners`,
  `progress`, `navbar`, `close_button`).
- Scripts `install.sh`/`start.sh`/`update.sh` gom về `scripts/`.

### Deprecated
- `card::create_item_row` → dùng `list_group::create_list_row`.
- `create_placeholder_message` → dùng `create_placeholder_row` +
  `PlaceholderState`.

### Fixed
- Docs: `tokens.md` mục Badge khớp component thực tế; `tooltips.md` signature
  `create_icon_button` (5 tham số); `popovers.md` class `status-good` →
  `success-text`.

---

## [1.0.0] — 2026-08-14

Bản phát hành ban đầu (workspace 4 libs + 8 crates + install).
