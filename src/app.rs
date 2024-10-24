use crate::cli_args::Arguments;
use clap::Parser;
use log::{info, warn, LevelFilter};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

const DEFAULT_PORT: u16 = 9100;
const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Info;

pub struct App {
    port: u16,
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

    pub async fn run(&self) -> anyhow::Result<()> {
        let address = ("127.0.0.1", self.port);
        let server = TcpListener::bind(address).await?;
        info!("Emulator listening on port {}", self.port);

        let mut buffer = vec![0; 1024 * 10];
        info!("Waiting for connections");
        loop {
            let (mut peer_stream, peer_address) = server.accept().await?;
            info!("New peer connected: {peer_address}");

            info!("Waiting for peer input");
            let read_amount = peer_stream.read(&mut buffer).await?;
            info!("Peer sent: {:?}", &buffer[0..read_amount]);
        }
    }
}
