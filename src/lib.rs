use log::info;

pub mod shor;
pub mod blockchain;
pub mod quantum;
pub mod audit;
pub mod cli;

pub use shor::*;
pub use blockchain::*;
pub use quantum::*;
pub use audit::*;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::CliArgs::parse();
    cli::run_with_args(args)
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn info() {
    println!("Quantum Shor Algorithm - Blockchain Security Audit");
    println!("Versión: {}", version());
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
}