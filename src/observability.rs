use core::fmt;
use std::{fs::File, sync::Mutex};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{
    filter::LevelFilter, layer::SubscriberExt, reload, util::SubscriberInitExt, EnvFilter, Registry,
};

/// Log filter level for the node.
#[derive(Default, Debug, Copy, Clone, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LogLevel::Trace => f.pad("TRACE"),
            LogLevel::Debug => f.pad("DEBUG"),
            LogLevel::Info => f.pad("INFO"),
            LogLevel::Warn => f.pad("WARN"),
            LogLevel::Error => f.pad("ERROR"),
        }
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => LevelFilter::TRACE,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}

/// A sharable reference to the observability stack.
#[derive(Debug, Default, Clone)]
pub struct Observability {
    binary_names: Vec<String>,
    reload_handle: Option<reload::Handle<EnvFilter, Registry>>,
}

impl Observability {
    /// Initialize the tracing subscriber.
    pub fn init(
        binary_names: Vec<String>,
        log_level_filter: LevelFilter,
        log_file: File,
    ) -> Result<Self, anyhow::Error> {
        let joined_filter = binary_names
            .iter()
            .map(|x| format!("{}={}", x, log_level_filter.to_string().to_lowercase()))
            .collect::<Vec<String>>()
            .join(",");
        let filter = Self::parse_filter(&joined_filter)?;
        let (filter, reload_handle) = reload::Layer::new(filter);

        let timer_format =
            time::format_description::parse("[hour]:[minute]:[second]").expect("Cataplum");
        let time_offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
        let timer = tracing_subscriber::fmt::time::OffsetTime::new(time_offset, timer_format);

        tracing_subscriber::registry()
            .with(filter)
            .with(
                tracing_subscriber::fmt::layer().event_format(
                    tracing_subscriber::fmt::format()
                        .compact()
                        .with_timer(timer.clone())
                        .with_target(false),
                ),
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .event_format(
                        tracing_subscriber::fmt::format()
                            .compact()
                            .with_timer(timer.clone())
                            .with_target(false),
                    )
                    .with_writer(Mutex::new(log_file))
                    .with_ansi(false),
            )
            .init();

        Ok(Self {
            binary_names,
            reload_handle: Some(reload_handle),
        })
    }

    /// Set the log level for the binary.
    pub fn set_log_level(&self, level: LogLevel) -> Result<(), anyhow::Error> {
        let level = LevelFilter::from(level);
        let new_filter = Self::parse_filter(
            &self
                .binary_names
                .join(format!("={},", level.to_string().to_lowercase()).as_str()),
        )?;

        if let Some(handle) = &self.reload_handle {
            handle.modify(|filter| *filter = new_filter)?;
        }

        Ok(())
    }

    /// Sets advanced logging directive.
    /// Example:
    ///     * "my_crate=debug"
    ///     * "my_crate::module=trace"
    ///     * "my_crate=debug,other_crate=warn"
    pub fn set_logging(&self, directive: &str) -> Result<(), anyhow::Error> {
        let new_filter = Self::parse_filter(directive)?;

        if let Some(handle) = &self.reload_handle {
            handle.modify(|filter| *filter = new_filter)?;
        }

        Ok(())
    }

    /// Parses a directive and builds an [EnvFilter] from it.
    /// Example:
    ///     * "my_crate=debug"
    ///     * "my_crate::module=trace"
    ///     * "my_crate=debug,other_crate=warn"
    fn parse_filter(directive: &str) -> Result<EnvFilter, anyhow::Error> {
        let mut filter = EnvFilter::from_default_env();
        for directive in directive.split(',') {
            filter = filter.add_directive(directive.parse()?);
        }

        Ok(filter)
    }
}
