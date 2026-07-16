use std::path::PathBuf;

/// Performs the cut/copy paste operation asynchronously and returns if it was completely successful.
pub async fn perform_paste(sources: Vec<PathBuf>, is_cut: bool, dest_dir: PathBuf) -> bool {
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
    all_success
}
