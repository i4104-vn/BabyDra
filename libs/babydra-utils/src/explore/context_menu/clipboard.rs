use std::path::PathBuf;
use std::rc::Rc;
use crate::explore::context_menu::CLIPBOARD;

/// Executes paste (copy or cut) operation asynchronously and triggers navigation refresh.
pub fn execute_paste(
    sources: Vec<PathBuf>,
    dest_dir: PathBuf,
    is_cut: bool,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    glib::spawn_future_local(async move {
        let mut all_success = true;
        for src in sources {
            if let Some(filename) = src.file_name() {
                let dest = dest_dir.join(filename);
                if is_cut {
                    if let Err(e) = babydra_common::move_path(src, dest).await {
                        eprintln!("Failed to move file: {}", e);
                        all_success = false;
                    }
                } else {
                    if let Err(e) = babydra_common::copy_path(src, dest).await {
                        eprintln!("Failed to copy file: {}", e);
                        all_success = false;
                    }
                }
            }
        }
        if is_cut && all_success {
            CLIPBOARD.with(|cb| cb.replace(None));
        }
        nav_callback(current_path);
    });
}
