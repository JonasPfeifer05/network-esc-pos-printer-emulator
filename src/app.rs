use crate::cli_args::Arguments;
use clap::Parser;
use log::{error, info, LevelFilter, warn};

const DEFAULT_PORT: u64 = 9100;
const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Info;

pub struct App {
    port: u64,
}

impl App {
    pub fn initialized() -> Self {
        let arguments: Arguments = Arguments::parse();
        
        let (is_fallback, level_filter) = if arguments.verbose.is_present() {
            (false, arguments.verbose.log_level_filter())
        } else {
            (true, DEFAULT_LOG_LEVEL)
        };
        env_logger::builder().filter_level(level_filter).init();
        if is_fallback {
            warn!("No verbosity provided therefore using default verbosity: {DEFAULT_LOG_LEVEL}");
        }
        
        let port = arguments.port.unwrap_or_else(|| {
            warn!("No --port argument provided therefore using default port: {DEFAULT_PORT}");
            DEFAULT_PORT
        });
        info!("Starting esc/pos emulator on port {port}");

        Self { port }
    }

    pub fn run(&self) {
        println!("Running")
    }
}
