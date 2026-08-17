# API Reference — `babydra_ui_kit::components::explore`

**Crate:** `libs/babydra-ui-kit/` (module `components/explore/`)
**Phạm vi:** Feature components cho Explore — dialogs, context menu, drag & drop, file items, rubberband selection.
**Dependency:** `babydra-core`, `gtk4`, `trash` (dùng chung deps của `babydra-ui-kit`).

---

## 1. Cách dùng nhanh

`babydra-ui-kit` đã bao gồm module `components::explore` — chỉ cần khai báo 1 dependency:

```toml
babydra-ui-kit = { workspace = true }
```

Import toàn bộ API feature qua `prelude` riêng:

```rust
use babydra_ui_kit::components::explore::prelude::*;

// Hiện menu ngữ cảnh cho file / thư mục / vùng trống
show_for_file(&widget, x, y, target_paths, current_path, nav_cb, &window);
show_for_empty(&widget, x, y, current_path, nav_cb);

// Dialog
show_new_folder_dialog(current_path.clone(), nav_cb.clone(), None);
show_rename_dialog(&path, current_path.clone(), nav_cb.clone(), None);

// Item builders
let card = create_grid_file_item(idx, &entry, selected_paths, on_right_click);
let row  = create_list_row(idx, &entry, ...);

// Drag & drop
let target = create_dir_drop_target(dest_path);
```

---

## 2. `components::explore::prelude` — re-export toàn bộ API feature

`babydra_ui_kit::components::explore::prelude::*` gộp mặt phẳng API của mọi module feature.
Các mục chính:

| Nhóm | Mục |
| :--- | :--- |
| **Context menu** | `show_for_file`, `show_for_empty`, `show_for_file_normal`, `show_for_file_trash`, `CLIPBOARD`, `UndoOperation`, `set_system_clipboard_files`, `execute_paste`, `execute_paste_from_system_clipboard`, `execute_undo`, `append_custom_context_items`, `apply_cut_dimming`, `apply_cut_dimming_global`, `create_menu_popover`, `create_menu_button`, `create_footer_icon_button`, `create_footer_container` |
| **Dialogs** | `show_alert_dialog`, `show_compress_dialog`, `show_compress_log_dialog`, `show_delete_confirm_dialog`, `show_conflict_dialog`, `show_password_dialog`, `show_decompress_log_dialog`, `perform_decompress_async`, `show_new_file_dialog`, `show_new_folder_dialog`, `show_properties_dialog`, `show_rename_dialog` |
| **Properties helpers** | `get_permissions_string`, `count_dir_contents_recursive`, `count_dialog_height`, `build_permission_matrix`, `apply_permissions`, `PermissionCheckboxes` |
| **Drag & drop** | `create_drag_source`, `create_dir_drop_target`, `create_dir_drop_target_with_nav`, `create_background_drop_target` |
| **Helpers** | `format_size`, `format_date`, `is_archive_file`, `is_in_trash`, `restore_from_trash`, `parse_target_dir`, `sanitize_path` |
| **Items** | `create_grid_file_item`, `create_list_row` |
| **Selection** | `wire_rubberband_grid`, `wire_rubberband_listbox` |
| **Widgets** | `update_new_folder_button` |

> [!NOTE]
> `components::explore::prelude` tách riêng khỏi `prelude` gốc vì một số tên trùng (vd
> `create_list_row`) — mỗi prelude giữ đúng không gian API của mình.

---

## 3. Context Menu

### 3.1. Hiển thị menu

| Hàm | Mô tả |
| :--- | :--- |
| `show_for_file(parent, x, y, target_paths, current_path, nav_cb, parent_window)` | Menu cho 1+ file/folder — tự route vào trash menu khi đang trong Thùng rác |
| `show_for_empty(parent, x, y, current_path, nav_cb)` | Menu cho vùng trống (New Folder, Paste…) |
| `show_for_file_normal(...)` / `show_for_file_trash(...)` | Menu tường minh (nếu cần kiểm soát thủ công) |

### 3.2. Clipboard (Cut / Copy / Paste / Undo)

```rust
set_system_clipboard_files(&paths, true /* is_cut */);
execute_paste(dest_path.clone(), nav_cb.clone(), None, true /* is_cut */);
execute_paste_from_system_clipboard(dest_path.clone(), nav_cb.clone());
execute_undo(nav_cb, current_path);
```

- `CLIPBOARD` — `thread_local` lưu `(paths, is_cut)` của thao tác cut/copy nội bộ.
- `UndoOperation` — struct mô tả thao tác undo.

### 3.3. Custom items & cut dimming

```rust
append_custom_context_items(&vbox, &custom_items);  // từ ExploreSettings
apply_cut_dimming(&root_widget, &cut_paths);
apply_cut_dimming_global(&cut_paths);
```

### 3.4. Widgets menu

```rust
let (popover, vbox) = create_menu_popover(&parent, x, y);
let btn = create_menu_button("Open", "folder");
let (footer_box, actions_box) = create_footer_container();
```

---

## 4. Dialogs

| Hàm | Dùng cho |
| :--- | :--- |
| `show_alert_dialog(title, message, parent)` | Thông báo lỗi / alert |
| `show_compress_dialog(targets, current_path, nav_cb, parent)` | Nén file/folder |
| `show_compress_log_dialog(...)` | Log quá trình nén |
| `show_delete_confirm_dialog(...)` | Xác nhận xóa |
| `show_conflict_dialog(name, on_override, parent)` | Trùng tên — Cancel / Override |
| `show_password_dialog(archive, current, nav_cb, parent)` | Mật khẩu archive zip |
| `show_decompress_log_dialog(...)` | Log giải nén |
| `perform_decompress_async(archive, current, nav_cb, parent)` | Giải nén async — tự phát hiện zip mật khẩu |
| `show_new_file_dialog(...)` | Tạo file mới |
| `show_new_folder_dialog(current_path, nav_cb, parent)` | Tạo thư mục mới |
| `show_properties_dialog(target_paths, parent)` | Thuộc tính file/folder |
| `show_rename_dialog(&path, current_path, nav_cb, parent)` | Đổi tên |

> [!NOTE]
> Tất cả hàm dialog nhận `parent: Option<&impl IsA<gtk4::Window>>` — truyền `None` để
> tự do, hoặc window cha để modal đúng ngữ cảnh.

### 4.1. Thuộc tính (Properties)

```rust
show_properties_dialog(vec![path], Some(&window));

// Tiện ích phụ
let perm = get_permissions_string(0o755);              // "rwxr-xr-x"
let (files, dirs) = count_dir_contents_recursive(&path);
let checkboxes = build_permission_matrix(&parent_vbox, 0o755);
apply_permissions(&path, &checkboxes);
```

---

## 5. Drag & Drop

```rust
// Đích thả vào thư mục
let target = create_dir_drop_target(dest_path);
let target = create_dir_drop_target_with_nav(dest_path, Some(nav_cb));

// Đích thả vào nền (dùng current_path mỗi lần thả)
let target = create_background_drop_target(Rc::new(RefCell::new(current_path)));

// Nguồn kéo
let source = create_drag_source(&path, &icon_name, selected_paths);
```

---

## 6. Helpers thuần

| Hàm | Kiểu trả về | Mô tả |
| :--- | :--- | :--- |
| `format_size(size: u64)` | `String` | "1.5 GB" / "2.5 TB" |
| `format_date(mtime: SystemTime)` | `String` | Ngày theo locale hiện tại |
| `is_archive_file(path)` | `bool` | `.zip/.tar/.gz/...` |
| `is_in_trash(path)` | `bool` | Đang nằm trong Thùng rác |
| `restore_from_trash(trash_file_path) -> Result<(), String>` | `async` | Khôi phục file/folder từ Thùng rác về vị trí cũ (đọc `.trashinfo`) |
| `parse_target_dir() -> PathBuf` | — | Thư mục đích khi paste (đọc global state) |
| `sanitize_path(path) -> PathBuf` | — | Vệ sinh tên file trùng |

---

## 7. Items & Selection

```rust
// Grid card / list row
let card: gtk4::FlowBoxChild = create_grid_file_item(idx, &entry, selected_paths, on_right_click);
let row: gtk4::ListBoxRow = create_list_row(idx, &entry, ...);

// Rubberband chọn nhiều
wire_rubberband_grid(&overlay, box_, fixed, rubberband_box);
wire_rubberband_listbox(&list_box, ...);
```

---

## 8. Quy tắc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Import qua `components::explore::prelude::*`; dùng module sâu khi cần (vd `components::explore::dialogs::properties::helpers`) |
| DO | Dialog async phải chạy trong `glib::spawn_future_local` — xem `perform_decompress_async` |
| DO | Thao tác file hệ thống đi qua `babydra-core` services; module chỉ dựng UI + gọi |
| DO NOT | Gọi trực tiếp `std::fs` xóa/nén trong render code — dùng helper của module hoặc core |
| DO NOT | Hardcode chuỗi UI — qua `babydra_core::i18n::t()` |

Xem thêm: [tổng hợp API kits](../06-kits-api.md), [explore docs](../02-architecture.md).
