mod app;
mod cli_args;

use crate::app::App;

fn main() {
    App::initialized().run();
}
