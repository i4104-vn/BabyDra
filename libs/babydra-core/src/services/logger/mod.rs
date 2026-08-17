use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use tracing::field::Visit;
use tracing::{Event, Level, Metadata, Subscriber};

/// Resolves the global log directory path `~/.babydra/logs`
pub fn get_log_dir() -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        return home.join(".babydra").join("logs");
    }
    PathBuf::from(".babydra/logs")
}

/// Resolves the full log file path `~/.babydra/logs/<filename>`
pub fn get_log_path(filename: &str) -> PathBuf {
    get_log_dir().join(filename)
}

pub struct BabyDraLogger {
    file: Mutex<Option<File>>,
    log_path: PathBuf,
}

impl BabyDraLogger {
    pub fn new(filename: &str) -> Self {
        let log_dir = get_log_dir();
        if let Err(e) = std::fs::create_dir_all(&log_dir) {
            eprintln!(
                "[LOGGER ERROR] Could not create log directory {:?}: {}",
                log_dir, e
            );
        }

        let log_path = get_log_path(filename);
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .map_err(|e| {
                eprintln!(
                    "[LOGGER ERROR] Could not open log file {:?}: {}",
                    log_path, e
                );
                e
            })
            .ok();

        Self {
            file: Mutex::new(file),
            log_path,
        }
    }

    pub fn write_entry(&self, level: &str, target: &str, msg: &str) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let formatted = format!("[{}] [{}] [{}] {}\n", timestamp, level, target, msg);

        if level == "ERROR" {
            eprint!("{}", formatted);
        } else {
            print!("{}", formatted);
        }

        if let Ok(mut guard) = self.file.lock() {
            if let Some(ref mut f) = *guard {
                let _ = f.write_all(formatted.as_bytes());
                let _ = f.flush();
            }
        }
    }
}

struct MessageVisitor {
    message: String,
    extra_fields: Vec<String>,
}

impl Visit for MessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        let name = field.name();
        let val_str = format!("{:?}", value);
        if name == "message" {
            if val_str.starts_with('"') && val_str.ends_with('"') && val_str.len() >= 2 {
                self.message = val_str[1..val_str.len() - 1]
                    .replace("\\\"", "\"")
                    .replace("\\n", "\n");
            } else {
                self.message = val_str;
            }
        } else {
            self.extra_fields.push(format!("{}={}", name, val_str));
        }
    }
}

impl Subscriber for BabyDraLogger {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _span: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(1)
    }

    fn record(&self, _span: &tracing::span::Id, _values: &tracing::span::Record<'_>) {}

    fn record_follows_from(&self, _span: &tracing::span::Id, _follows: &tracing::span::Id) {}

    fn event(&self, event: &Event<'_>) {
        let metadata = event.metadata();
        let level_str = match *metadata.level() {
            Level::ERROR => "ERROR",
            Level::WARN => "WARN ",
            Level::INFO => "INFO ",
            Level::DEBUG => "DEBUG",
            Level::TRACE => "TRACE",
        };

        let mut visitor = MessageVisitor {
            message: String::new(),
            extra_fields: Vec::new(),
        };
        event.record(&mut visitor);

        let mut full_msg = visitor.message;
        if !visitor.extra_fields.is_empty() {
            if !full_msg.is_empty() {
                full_msg.push(' ');
            }
            full_msg.push_str(&format!("({})", visitor.extra_fields.join(", ")));
        }

        self.write_entry(level_str, metadata.target(), &full_msg);
    }

    fn enter(&self, _span: &tracing::span::Id) {}

    fn exit(&self, _span: &tracing::span::Id) {}
}

/// Initializes global logger for any BabyDra desktop environment component
pub fn init_logger(app_name: &str, filename: &str) -> PathBuf {
    let logger = BabyDraLogger::new(filename);
    let log_path = logger.log_path.clone();

    logger.write_entry(
        "INFO ",
        app_name,
        "==================================================",
    );
    logger.write_entry("INFO ", app_name, &format!("{} session started", app_name));
    logger.write_entry(
        "INFO ",
        app_name,
        &format!("Log file initialized at: {}", log_path.display()),
    );
    logger.write_entry(
        "INFO ",
        app_name,
        "==================================================",
    );

    if let Err(e) = tracing::subscriber::set_global_default(logger) {
        eprintln!(
            "[LOGGER ERROR] Failed to set global tracing subscriber for {}: {}",
            app_name, e
        );
    }

    log_path
}
