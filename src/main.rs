mod quantum_shor;

fn main() {
    if let Err(e) = quantum_shor::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}