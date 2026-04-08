use crate::audit::{AuditResults, AuditStatus, AuditResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub timestamp: u64,
    pub total_score: f64,
    pub results: AuditResults,
    pub summary: String,
    pub recommendations: Vec<String>,
}

impl AuditReport {
    pub fn new(results: AuditResults) -> Self {
        let total_score = Self::calculate_score(&results);
        let summary = Self::generate_summary(&results);
        let recommendations = Self::generate_recommendations(&results);
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        AuditReport {
            timestamp,
            total_score,
            results,
            summary,
            recommendations,
        }
    }

    fn calculate_score(results: &AuditResults) -> f64 {
        if results.is_empty() {
            return 0.0;
        }
        let total: f64 = results.values().map(|r| r.score).sum();
        total / results.len() as f64
    }

    fn generate_summary(results: &AuditResults) -> String {
        let passed = results.values().filter(|r| r.status == AuditStatus::Pass).count();
        let warnings = results.values().filter(|r| r.status == AuditStatus::Warning).count();
        let failed = results.values().filter(|r| r.status == AuditStatus::Fail).count();
        
        format!("Passed: {}, Warnings: {}, Failed: {}", passed, warnings, failed)
    }

    fn generate_recommendations(results: &AuditResults) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        for (key, result) in results {
            match result.status {
                AuditStatus::Fail => {
                    recommendations.push(format!("CRÍTICO: {} - {}", key, result.details));
                }
                AuditStatus::Warning => {
                    recommendations.push(format!("ADVERTENCIA: {} - {}", key, result.details));
                }
                _ => {}
            }
        }
        
        if recommendations.is_empty() {
            recommendations.push("La blockchain pasa todos los controles de seguridad".to_string());
        }
        
        recommendations
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn to_csv(&self) -> String {
        let mut csv = String::from("Test,Status,Score,Details\n");
        
        for (key, result) in &self.results {
            let status = match result.status {
                AuditStatus::Pass => "PASS",
                AuditStatus::Warning => "WARNING",
                AuditStatus::Fail => "FAIL",
            };
            csv.push_str(&format!(
                "{},{},{:.2},{}\n",
                key, status, result.score, result.details
            ));
        }
        
        csv
    }

    pub fn print_summary(&self) {
        println!("╔══════════════════════════════════════════════════════════╗");
        println!("║          REPORTE DE AUDITORÍA CUÁNTICA                    ║");
        println!("╠══════════════════════════════════════════════════════════╣");
        println!("║ Timestamp: {}                                           ║", self.timestamp);
        println!("║ Score: {:.1}%                                          ║", self.total_score * 100.0);
        println!("╠══════════════════════════════════════════════════════════╣");
        
        for (test, result) in &self.results {
            let status = match result.status {
                AuditStatus::Pass => "✓ PASS",
                AuditStatus::Warning => "⚠ WARN",
                AuditStatus::Fail => "✗ FAIL",
            };
            println!("║ {:30} {:8} {:5.1}% ║", 
                test.chars().take(30).collect::<String>(),
                status,
                result.score * 100.0
            );
        }
        
        println!("╠══════════════════════════════════════════════════════════╣");
        println!("║ RECOMENDACIONES:                                         ║");
        for rec in &self.recommendations {
            println!("║ - {}", rec.chars().take(50).collect::<String>());
        }
        println!("╚══════════════════════════════════════════════════════════╝");
    }
}

pub fn create_report(results: AuditResults) -> AuditReport {
    AuditReport::new(results)
}

pub fn export_report(report: &AuditReport, path: &str, format: ReportFormat) -> std::io::Result<()> {
    let content = match format {
        ReportFormat::Json => report.to_json(),
        ReportFormat::Csv => report.to_csv(),
        ReportFormat::Text => format!("{:#?}", report),
    };
    
    std::fs::write(path, content)
}

#[derive(Debug, Clone, Copy)]
pub enum ReportFormat {
    Json,
    Csv,
    Text,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_report_creation() {
        let mut results = HashMap::new();
        results.insert("test".to_string(), AuditResult::pass(1.0, "OK"));
        
        let report = AuditReport::new(results);
        assert!(report.total_score > 0.0);
    }

    #[test]
    fn test_report_json() {
        let mut results = HashMap::new();
        results.insert("test".to_string(), AuditResult::pass(1.0, "OK"));
        
        let report = AuditReport::new(results);
        let json = report.to_json();
        assert!(json.len() > 0);
    }
}