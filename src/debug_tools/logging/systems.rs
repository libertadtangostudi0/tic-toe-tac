use bevy::prelude::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use super::resources::{LogConfig, LogRuntimeState, LogEvent, LogLevel};

fn log_level_priority(level: LogLevel) -> u8 {
    match level {
        LogLevel::Trace => 0,
        LogLevel::Debug => 1,
        LogLevel::Info  => 2,
        LogLevel::Warn  => 3,
        LogLevel::Error => 4,
    }
}

fn should_log(level: LogLevel, config: &LogConfig) -> bool {
    log_level_priority(level) >= log_level_priority(config.min_level)
}

fn level_tag(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Trace => "TRACE",
        LogLevel::Debug => "DEBUG",
        LogLevel::Info  => "INFO",
        LogLevel::Warn  => "WARN",
        LogLevel::Error => "ERROR",
    }
}

/// Initialize log directory. This is called once on startup.
pub fn init_log_directory(
    mut state: ResMut<LogRuntimeState>,
    config: Res<LogConfig>,
) {
    if state.initialized {
        return;
    }

    let dir = PathBuf::from(&config.directory);

    if let Err(err) = fs::create_dir_all(&dir) {     // We do not panic here; just print to stderr.
        eprintln!(
            "Failed to create log directory '{}': {}",
            dir.display(),
            err
        );
        // Logging will remain disabled effectively, because no file can be created.
        // But we still mark as initialized to avoid spamming this error.
        state.initialized = true;
        return;
    }

    state.initialized = true;
}

/// Flush log events to the configured log file.
pub fn flush_log_buffer(
    config: Res<LogConfig>,
    state: Res<LogRuntimeState>,
    mut events: EventReader<LogEvent>,
) {
    if !config.enabled || !state.initialized {
        for _ in events.read() {
            // Logging is disabled or not ready.
            // Still need to drain events to avoid keeping them forever.
            // Just consume them.
        }
        return;
    }

    // Lazily open the file only if we actually have something to write.
    let mut file: Option<std::fs::File> = None;

    for event in events.read() {
        if !should_log(event.level, &config) {
            continue;
        }

        // Open the file on first eligible event.
        if file.is_none() {
            let mut path = PathBuf::from(&config.directory);
            path.push(&config.file_name);

            match OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
            {
                Ok(f) => {
                    file = Some(f);
                }
                Err(err) => {
                    eprintln!(
                        "Failed to open log file '{}': {}",
                        path.display(),
                        err
                    );
                    // If we cannot open the file, there is no point in trying further.
                    // We still consume remaining events to keep the queue clean.
                    return;
                }
            }
        }

        if let Some(f) = file.as_mut() {
            let tag = level_tag(event.level);

            // In debug mode we can optionally also print to stdout.
            if config.debug_mode && event.level != LogLevel::Trace {
                println!("[{}] {}", tag, event.message);
            }

            if let Err(err) = writeln!(f, "[{}] {}", tag, event.message) {
                eprintln!("Failed to write to log file: {}", err);
                // If writing fails, we stop further attempts in this frame.
                break;
            }
        }
    }
}
