pub mod shor;
pub mod blockchain;
pub mod quantum;
pub mod audit;
pub mod cli;

pub use shor::*;
pub use blockchain::*;
pub use quantum::*;
pub use audit::*;

use log::info;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    let args = cli::CliArgs::parse();
    cli::run_with_args(args)
}