pub mod dialog;
pub mod log_dialog;

pub use dialog::show_password_dialog;
pub use log_dialog::show_decompress_log_dialog;

use std::path::PathBuf;
use std::rc::Rc;
use babydra_common::services::explore::is_zip_encrypted;

pub fn perform_decompress_async(
    archive_path: PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    let nav_c = nav_callback.clone();
    let cp_c = current_path.clone();
    let archive_path_c = archive_path.clone();
    
    glib::spawn_future_local(async move {
        let name = archive_path_c.to_string_lossy().to_lowercase();
        let is_zip = name.ends_with(".zip");
            
        if is_zip && is_zip_encrypted(&archive_path_c).await {
            show_password_dialog(archive_path_c, cp_c, nav_c);
        } else {
            show_decompress_log_dialog(archive_path_c, cp_c, nav_c, None);
        }
    });
}
