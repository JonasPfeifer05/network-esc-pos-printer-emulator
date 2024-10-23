use clap::Parser;
use log::{info, LevelFilter};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    port: Option<u64>
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Debug).init();

    let arguments: Arguments = Arguments::parse();

    let port;
    match arguments.port {
        None => {
            info!("No port was provided");
            info!("Therefore using default port 9100");
            port = 9100;
        }
        Some(user_port) => {
            port = user_port;
        }
    }
    info!("Starting esc/pos emulator on port {port}")
}
