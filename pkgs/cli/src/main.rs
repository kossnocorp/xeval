mod cli;
mod command;
mod config;
mod openai;
mod prelude;
use prelude::*;

fn main() -> Result<()> {
    Cli::run()
}
