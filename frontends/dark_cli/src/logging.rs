use std::fs::OpenOptions;
use std::io::{IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

const DEFAULT_LOG_LEVEL: &str = "info";

pub(crate) fn init() -> Result<PathBuf> {
    let log_path = resolve_log_path()?;
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .with_context(|| {
            format!(
                "Dark CLI // Logging // Failed to open log file ({})",
                log_path.display()
            )
        })?;
    let file = Arc::new(Mutex::new(file));

    let filter_value = std::env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_LOG_LEVEL.to_string());
    let env_filter = EnvFilter::try_new(filter_value)
        .or_else(|_| EnvFilter::try_new(DEFAULT_LOG_LEVEL))
        .context("Dark CLI // Logging // Failed to build env filter")?;

    let use_ansi = std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none();
    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_ansi(use_ansi)
        .without_time();

    let file_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_ansi(false)
        .without_time()
        .with_writer(BoxMakeWriter::new({
            let file = Arc::clone(&file);
            move || SharedFileWriter::new(Arc::clone(&file))
        }));

    Registry::default()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .try_init()
        .context("Dark CLI // Logging // Failed to initialize tracing subscriber")?;

    Ok(log_path)
}

fn resolve_log_path() -> Result<PathBuf> {
    if let Ok(override_path) = std::env::var("DARK_CLI_LOG") {
        let trimmed = override_path.trim();
        if !trimmed.is_empty() {
            let path = PathBuf::from(trimmed);
            ensure_parent_dir(&path)?;
            return Ok(path);
        }
    }

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let preferred = cwd
        .join(".darkfactory")
        .join("logs")
        .join(format!("dark_cli-{}.log", unix_timestamp_millis()));

    if ensure_parent_dir(&preferred).is_ok() {
        return Ok(preferred);
    }

    let fallback = std::env::temp_dir().join(format!("dark_cli-{}.log", unix_timestamp_millis()));
    ensure_parent_dir(&fallback)?;
    Ok(fallback)
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!(
                "Dark CLI // Logging // Failed creating log directory ({})",
                parent.display()
            )
        })?;
    }
    Ok(())
}

fn unix_timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_millis())
}

struct SharedFileWriter {
    file: Arc<Mutex<std::fs::File>>,
}

impl SharedFileWriter {
    fn new(file: Arc<Mutex<std::fs::File>>) -> Self {
        Self { file }
    }
}

impl Write for SharedFileWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut file = self
            .file
            .lock()
            .map_err(|_| std::io::Error::other("dark_cli log writer lock poisoned"))?;
        file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut file = self
            .file
            .lock()
            .map_err(|_| std::io::Error::other("dark_cli log writer lock poisoned"))?;
        file.flush()
    }
}
