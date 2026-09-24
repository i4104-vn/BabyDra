use chrono::Local;
use std::fs::{self, File};
use std::io::{LineWriter, Write};
use std::path::{Path, PathBuf};

pub struct FileLogger {
    writer: LineWriter<File>,
    path: PathBuf,
    action_prefix: String,
    action_name: String,
}

impl FileLogger {
    /// Start a new file logger in `<repo_root>/logs/` with timestamped filename
    pub fn start(repo_root: &Path, action_prefix: &str, display_title: &str) -> std::io::Result<Self> {
        let logs_dir = repo_root.join("logs");
        fs::create_dir_all(&logs_dir)?;

        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
        let filename = format!("{}_{}.log", action_prefix, timestamp);
        let path = logs_dir.join(&filename);

        let file = File::create(&path)?;
        let mut writer = LineWriter::new(file);

        let header = format!(
            "================================================================================\n\
             BabyDra Updater - {}\n\
             Started at: {}\n\
             Repo root:  {}\n\
             Log file:   logs/{}\n\
             ================================================================================\n\n",
            display_title,
            now.format("%Y-%m-%d %H:%M:%S"),
            repo_root.display(),
            filename
        );
        writer.write_all(header.as_bytes())?;

        Ok(Self {
            writer,
            path,
            action_prefix: action_prefix.to_string(),
            action_name: display_title.to_string(),
        })
    }

    pub fn write_line(&mut self, line: &str) {
        let now = Local::now();
        let ts = now.format("%H:%M:%S");
        let _ = writeln!(self.writer, "[{}] {}", ts, line);
    }

    pub fn write_step(&mut self, step: &str) {
        let now = Local::now();
        let ts = now.format("%H:%M:%S");
        let _ = writeln!(self.writer, "\n[{}] [STEP] {}", ts, step);
    }

    pub fn write_success(&mut self, msg: &str) {
        let now = Local::now();
        let ts = now.format("%H:%M:%S");
        let _ = writeln!(self.writer, "[{}] [SUCCESS] {}", ts, msg);
    }

    pub fn write_error(&mut self, err: &str) {
        let now = Local::now();
        let ts = now.format("%H:%M:%S");
        let _ = writeln!(self.writer, "[{}] [ERROR] {}", ts, err);
    }

    pub fn finish(&mut self, code: i32) -> PathBuf {
        let now = Local::now();
        let status = if code == 0 { "SUCCESS" } else { "FAILED" };
        let footer = format!(
            "\n================================================================================\n\
             Action '{}' completed with exit code: {} ({})\n\
             Finished at: {}\n\
             ================================================================================\n",
            self.action_name,
            code,
            status,
            now.format("%Y-%m-%d %H:%M:%S")
        );
        let _ = self.writer.write_all(footer.as_bytes());
        let _ = self.writer.flush();

        // Also copy or update latest symlink/file for convenience (e.g. logs/install_latest.log)
        if let Some(parent) = self.path.parent() {
            let latest_name = format!("{}_latest.log", self.action_prefix);
            let latest_path = parent.join(latest_name);
            let _ = fs::copy(&self.path, latest_path);
        }

        self.path.clone()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
