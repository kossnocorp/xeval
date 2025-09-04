mod cli;
mod command;
mod prelude;
use std::process::exit;

use prelude::*;

#[tokio::main]
async fn main() {
    match Cli::run().await {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {:#}", err);
            exit(1);
        }
    }
}
