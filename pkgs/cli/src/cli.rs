use crate::prelude::*;

#[derive(Parser)]
#[command(name = "xeval")]
#[command(about = "LLM evaluation framework", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// Set custom config file.
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

impl Cli {
    pub fn run() -> Result<()> {
        let cli = Self::parse();
        println!("CLI config={:?}", cli.config);
        Command::run(&cli)
    }
}
