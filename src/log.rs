use std::env;
use std::error::Error;
use std::io::{stdout, IsTerminal};
use std::str::FromStr;
use tracing_log::LogTracer;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, Layer};

#[must_use = "this may hold resources used for logging purposes until dropped."]
pub struct LogHandle(());

pub fn setup() -> Result<LogHandle, Box<dyn Error>> {
    let targets = match env::var_os("RUST_LOG") {
        None => Targets::default(),
        Some(v) => Targets::from_str(v.to_string_lossy().as_ref())?,
    };

    let console = fmt::layer().pretty().with_ansi(stdout().is_terminal());

    tracing_subscriber::registry()
        .with(targets.and_then(console))
        .try_init()?;

    LogTracer::init()?;
    Ok(LogHandle(()))
}
