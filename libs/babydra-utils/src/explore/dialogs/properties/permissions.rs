use gtk4::prelude::*;
use gtk4::{Grid, Label, Align, CheckButton};
use std::path::Path;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use super::helpers::get_permissions_string;

pub struct PermissionCheckboxes {
    pub owner_read: CheckButton,
    pub owner_write: CheckButton,
    pub owner_exec: CheckButton,
    pub group_read: CheckButton,
    pub group_write: CheckButton,
    pub group_exec: CheckButton,
    pub others_read: CheckButton,
    pub others_write: CheckButton,
    pub others_exec: CheckButton,
}

pub fn build_permission_matrix(
    grid: &Grid,
    mode: u32,
    row_idx: &mut i32,
) -> PermissionCheckboxes {
    let lbl_key_perm = Label::builder().label("Permissions:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
    let perm_str = format!("{} ({:o})", get_permissions_string(mode), mode & 0o777);
    let lbl_val_perm = Label::builder().label(&perm_str).halign(Align::Start).selectable(false).build();
    grid.attach(&lbl_key_perm, 0, *row_idx, 1, 1);
    grid.attach(&lbl_val_perm, 1, *row_idx, 1, 1);
    *row_idx += 1;

    let lbl_key_edit_perm = Label::builder().label("Edit:").halign(Align::Start).css_classes(vec!["dim-label".to_string()]).build();
    grid.attach(&lbl_key_edit_perm, 0, *row_idx, 1, 1);

    let perm_grid = Grid::builder()
        .row_spacing(4)
        .column_spacing(10)
        .build();
    grid.attach(&perm_grid, 1, *row_idx, 1, 1);
    *row_idx += 1;

    // Labels
    let lbl_owner = Label::builder().label("Owner").halign(Align::Center).build();
    let lbl_group = Label::builder().label("Group").halign(Align::Center).build();
    let lbl_others = Label::builder().label("Others").halign(Align::Center).build();
    perm_grid.attach(&lbl_owner, 1, 0, 1, 1);
    perm_grid.attach(&lbl_group, 2, 0, 1, 1);
    perm_grid.attach(&lbl_others, 3, 0, 1, 1);

    let lbl_read = Label::builder().label("Read").halign(Align::Start).build();
    let lbl_write = Label::builder().label("Write").halign(Align::Start).build();
    let lbl_exec = Label::builder().label("Execute").halign(Align::Start).build();
    perm_grid.attach(&lbl_read, 0, 1, 1, 1);
    perm_grid.attach(&lbl_write, 0, 2, 1, 1);
    perm_grid.attach(&lbl_exec, 0, 3, 1, 1);

    // Checkboxes
    let c_or = CheckButton::builder().active(mode & 0o400 != 0).build();
    let c_ow = CheckButton::builder().active(mode & 0o200 != 0).build();
    let c_ox = CheckButton::builder().active(mode & 0o100 != 0).build();
    perm_grid.attach(&c_or, 1, 1, 1, 1);
    perm_grid.attach(&c_ow, 1, 2, 1, 1);
    perm_grid.attach(&c_ox, 1, 3, 1, 1);

    let c_gr = CheckButton::builder().active(mode & 0o040 != 0).build();
    let c_gw = CheckButton::builder().active(mode & 0o020 != 0).build();
    let c_gx = CheckButton::builder().active(mode & 0o010 != 0).build();
    perm_grid.attach(&c_gr, 2, 1, 1, 1);
    perm_grid.attach(&c_gw, 2, 2, 1, 1);
    perm_grid.attach(&c_gx, 2, 3, 1, 1);

    let c_tr = CheckButton::builder().active(mode & 0o004 != 0).build();
    let c_tw = CheckButton::builder().active(mode & 0o002 != 0).build();
    let c_tx = CheckButton::builder().active(mode & 0o001 != 0).build();
    perm_grid.attach(&c_tr, 3, 1, 1, 1);
    perm_grid.attach(&c_tw, 3, 2, 1, 1);
    perm_grid.attach(&c_tx, 3, 3, 1, 1);

    PermissionCheckboxes {
        owner_read: c_or,
        owner_write: c_ow,
        owner_exec: c_ox,
        group_read: c_gr,
        group_write: c_gw,
        group_exec: c_gx,
        others_read: c_tr,
        others_write: c_tw,
        others_exec: c_tx,
    }
}

pub fn apply_permissions(path: &Path, checkboxes: &PermissionCheckboxes) {
    let mut new_mode = 0;
    if checkboxes.owner_read.is_active() { new_mode |= 0o400; }
    if checkboxes.owner_write.is_active() { new_mode |= 0o200; }
    if checkboxes.owner_exec.is_active() { new_mode |= 0o100; }
    if checkboxes.group_read.is_active() { new_mode |= 0o040; }
    if checkboxes.group_write.is_active() { new_mode |= 0o020; }
    if checkboxes.group_exec.is_active() { new_mode |= 0o010; }
    if checkboxes.others_read.is_active() { new_mode |= 0o004; }
    if checkboxes.others_write.is_active() { new_mode |= 0o002; }
    if checkboxes.others_exec.is_active() { new_mode |= 0o001; }

    if let Ok(meta) = std::fs::metadata(path) {
        let original_mode = meta.mode();
        let final_mode = (original_mode & !0o777) | new_mode;
        let mut perms = meta.permissions();
        perms.set_mode(final_mode);
        if let Err(e) = std::fs::set_permissions(path, perms) {
            eprintln!("Failed to set permissions: {}", e);
        }
    }
}
