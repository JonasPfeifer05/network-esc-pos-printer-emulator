use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long)]
    pub port: Option<u16>,

    #[command(flatten)]
    pub verbose: Verbosity,
}
