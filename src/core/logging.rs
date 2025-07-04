use crate::core::config::EddaConfig;
use tracing::Level;
use tracing_subscriber::{
    FmtSubscriber,
    fmt::{format::FmtSpan, time::UtcTime},
};

/// Initialize logging with the specified configuration
pub fn init_logging(config: &EddaConfig) -> Result<(), Box<dyn std::error::Error>> {
    let log_level = parse_log_level(&config.log_level)?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_timer(UtcTime::rfc_3339())
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(atty::is(atty::Stream::Stdout))
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Logging initialized with level: {}", config.log_level);
    Ok(())
}

/// Parse log level from string
fn parse_log_level(level: &str) -> Result<Level, Box<dyn std::error::Error>> {
    match level.to_lowercase().as_str() {
        "trace" => Ok(Level::TRACE),
        "debug" => Ok(Level::DEBUG),
        "info" => Ok(Level::INFO),
        "warn" => Ok(Level::WARN),
        "error" => Ok(Level::ERROR),
        _ => Err(format!("Invalid log level: {}", level).into()),
    }
}

/// Create a logger for testing
#[cfg(test)]
pub fn init_test_logging() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_test_writer()
        .try_init();
}
