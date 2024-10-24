mod app;
mod cli_args;

use crate::app::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    App::initialized().run().await
}
