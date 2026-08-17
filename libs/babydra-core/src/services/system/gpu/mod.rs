//! GPU information retrieval service.

use std::process::Command;

/// Retrieves the GPU model string using lspci.
pub fn get_gpu_info() -> String {
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -i 'vga\\|3d' | cut -d: -f3 | head -n 1")
        .output()
    {
        let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !raw.is_empty() {
            return raw;
        }
    }
    "lspci lookup failed".to_string()
}
