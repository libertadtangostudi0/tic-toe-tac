use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Resource)]
pub struct LogConfig {
    /// Directory where log files should be stored.
    pub directory: String,
    /// Log file name inside the directory.
    pub file_name: String,
    /// Whether logging is enabled at all.
    pub enabled: bool,
    /// Minimal level to log (Trace < Debug < Info < Warn < Error).
    pub min_level: LogLevel,
    /// Debug mode flag (usually true in debug builds).
    pub debug_mode: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        let debug_mode = cfg!(debug_assertions);

        Self {
            directory: "logs".to_string(),
            file_name: "tictactoe.log".to_string(),
            enabled: true,
            // In debug mode we want more verbose logs.
            min_level: if debug_mode {
                LogLevel::Debug
            } else {
                LogLevel::Info
            },
            debug_mode,
        }
    }
}

/// Runtime state for logging (file handles, buffers, flags, etc.).
#[derive(Resource)]
pub struct LogRuntimeState {
    pub initialized: bool,
    // You can add internal buffers or counters later.
}

impl Default for LogRuntimeState {
    fn default() -> Self {
        Self {
            initialized: false,
        }
    }
}

/// Event that other systems can send to request a log write.
#[derive(Event, Message, Debug, Clone)]
pub struct LogEvent {
    pub level: LogLevel,
    pub message: String,
}
