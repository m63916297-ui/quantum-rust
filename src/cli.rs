use std::env;
use std::process;

pub struct CliArgs {
    pub blocks: usize,
    pub difficulty: u32,
    pub audit_mode: AuditMode,
    pub output: Option<String>,
    pub verbose: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AuditMode {
    Quick,
    Full,
    Deep,
}

impl CliArgs {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().collect();
        
        let mut blocks = 3;
        let mut difficulty = 2;
        let mut audit_mode = AuditMode::Full;
        let mut output = None;
        let mut verbose = false;
        
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--blocks" | "-b" => {
                    if i + 1 < args.len() {
                        blocks = args[i + 1].parse().unwrap_or(3);
                        i += 1;
                    }
                }
                "--difficulty" | "-d" => {
                    if i + 1 < args.len() {
                        difficulty = args[i + 1].parse().unwrap_or(2);
                        i += 1;
                    }
                }
                "--audit-mode" | "-a" => {
                    if i + 1 < args.len() {
                        audit_mode = match args[i + 1].as_str() {
                            "quick" => AuditMode::Quick,
                            "full" => AuditMode::Full,
                            "deep" => AuditMode::Deep,
                            _ => AuditMode::Full,
                        };
                        i += 1;
                    }
                }
                "--output" | "-o" => {
                    if i + 1 < args.len() {
                        output = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "--verbose" | "-v" => {
                    verbose = true;
                }
                "--help" | "-h" => {
                    print_help();
                    process::exit(0);
                }
                _ => {}
            }
            i += 1;
        }
        
        CliArgs {
            blocks,
            difficulty,
            audit_mode,
            output,
            verbose,
        }
    }
}

fn print_help() {
    println!("Quantum Shor Algorithm - Blockchain Security Audit");
    println!();
    println!("Usage: quantum-shor [OPTIONS]");
    println!();
    println!("Options:");
    println!("  -b, --blocks <num>     Number of blocks to create (default: 3)");
    println!("  -d, --difficulty <num> Mining difficulty (default: 2)");
    println!("  -a, --audit-mode <mode> Audit mode: quick, full, deep (default: full)");
    println!("  -o, --output <file>   Output file for report");
    println!("  -v, --verbose         Enable verbose output");
    println!("  -h, --help            Show this help message");
    println!();
    println!("Examples:");
    println!("  quantum-shor");
    println!("  quantum-shor --blocks 10 --difficulty 3");
    println!("  quantum-shor -a deep -o report.json");
}

pub fn run_with_args(args: CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.verbose {
        println!("=== Configuración ===");
        println!("Bloques: {}", args.blocks);
        println!("Dificultad: {}", args.difficulty);
        println!("Modo: {:?}", args.audit_mode);
        println!();
    }
    
    let mut blockchain = crate::blockchain::Blockchain::new(args.difficulty);
    
    let transactions = vec![
        "Alice -> Bob: 10 coins",
        "Bob -> Charlie: 5 coins",
        "Charlie -> Alice: 3 coins",
        "Alice -> Dave: 2 coins",
    ];
    
    for i in 0..args.blocks {
        let tx = transactions[i % transactions.len()].to_string();
        blockchain.add_block(tx);
    }
    
    println!("Blockchain creado con {} bloques", blockchain.chain_length());
    println!("Integridad: {}", blockchain.verify_integrity());
    println!();
    
    println!("=== Auditoría de Seguridad Cuántica ===");
    let audit_results = crate::audit::run_full_audit(&blockchain);
    
    let report = crate::audit::create_report(audit_results);
    report.print_summary();
    
    if let Some(output_path) = args.output {
        let content = report.to_json();
        std::fs::write(&output_path, content)?;
        println!("\nReporte guardado en: {}", output_path);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_args_default() {
        let args = CliArgs::parse();
        assert_eq!(args.blocks, 3);
        assert_eq!(args.difficulty, 2);
    }
}