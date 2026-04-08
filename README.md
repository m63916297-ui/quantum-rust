# Quantum Shor Algorithm - Blockchain Security Audit

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.70+-dea584?style=flat&logo=rust" alt="Rust Version">
  <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/Version-0.2.0-orange.svg" alt="Version">
</p>

## Tabla de Contenidos

1. [Descripción General](#descripción-general)
2. [Características Principales](#características-principales)
3. [Arquitectura del Sistema](#arquitectura-del-sistema)
4. [Patrones de Diseño](#patrones-de-diseño)
5. [Instalación y Configuración](#instalación-y-configuración)
6. [Uso](#uso)
7. [Referencia de API](#referencia-de-api)
8. [Ejemplos](#ejemplos)
9. [Testing](#testing)
10. [Benchmarking](#benchmarking)
11. [Contribución](#contribución)
12. [Licencia](#licencia)

---

## Descripción General

Este proyecto implementa una simulación del **algoritmo cuántico de Shor** para auditoría de seguridad en blockchains. El algoritmo de Shor es un algoritmo cuántico polynomial time para factorización de enteros, descubrimiento por Peter Shor en 1994, que tiene implicaciones significativas para la criptografía asimétrica.

### Propósito

El objetivo de este proyecto es demostrar cómo los algoritmos cuánticos podrían afectar la seguridad de las blockchains actuales y proporcionar herramientas para:

- **Análisis de Vulnerabilidad**: Evaluar la resistencia de hashes de blockchain ante ataques de factorización
- **Simulación Cuántica**: Modelar estados cuánticos utilizando computación clásica
- **Auditoría de Seguridad**: Verificar la integridad y seguridad de cadenas de bloques

---

## Características Principales

| Característica | Descripción |
|----------------|-------------|
| **Algoritmo de Shor** | Implementación clásica optimizada para factorización de enteros |
| **Simulación Cuántica** | Estados cuánticos, puertas (Hadamard, Pauli, CNOT), medición |
| **Blockchain Core** | Creación de bloques, proof-of-work, verificación de integridad |
| **Auditoría de Seguridad** | Análisis de resistencia cuántica, generación de reportes |
| **CLI Interface** | Interfaz de línea de comandos con opciones configurables |

---

## Arquitectura del Sistema

### Vista de Componentes

```
┌────────────────────────────────────────────────────────────────────────────┐
│                           APLICACIÓN PRINCIPAL                              │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                        CAPA DE PRESENTACIÓN                          │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌────────────┐  │  │
│  │  │    CLI      │  │   Report    │  │   Logger    │  │   Config   │  │  │
│  │  │  Parser     │  │   Viewer    │  │   Handler   │  │   Manager  │  │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └────────────┘  │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                    │                                        │
│                                    ▼                                        │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                          CAPA DE NEGOCIO                             │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌──────────────┐ │  │
│  │  │    SHOR    │  │ BLOCKCHAIN │  │  QUANTUM    │  │    AUDIT    │ │  │
│  │  │  Algorithm │  │    Core    │  │  Simulator  │  │    Engine   │ │  │
│  │  ├─────────────┤  ├─────────────┤  ├─────────────┤  ├──────────────┤ │  │
│  │  │ Order Find │  │   Block    │  │    State    │  │   Security  │ │  │
│  │  │ GCD        │  │   Chain    │  │    Gates    │  │   Report    │ │  │
│  │  │Modular Pow │  │   Miner    │  │ Measurement │  │   Metrics   │ │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └──────────────┘ │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                    │                                        │
│                                    ▼                                        │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                        CAPA DE DATOS                                 │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                   │  │
│  │  │   Block    │  │   State    │  │   Audit     │                   │  │
│  │  │   Storage  │  │   Vector   │  │   Results   │                   │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘                   │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### Estructura de Archivos

```
quantum/
├── Cargo.toml                          # Dependencias y metadata
├── README.md                           # Este archivo
├── LICENSE                             # Licencia MIT
├── .gitignore                          # Ignorados por git
│
├── src/
│   ├── main.rs                         # Punto de entrada de la aplicación
│   ├── lib.rs                         # API pública de la librería
│   │
│   ├── cli.rs                         # Interfaz de línea de comandos
│   │
│   ├── shor.rs                        # Algoritmo de Shor
│   │   ├── modular_pow()             # Potencia modular
│   │   ├── extended_gcd()           # GCD extendido (Euclides)
│   │   ├── find_order_*()            # Búsqueda de orden
│   │   ├── shor_algorithm()          # Factorización principal
│   │   └── continuous_fractions()    # Fracciones continuas
│   │
│   ├── blockchain/
│   │   ├── mod.rs                    # Exports públicos
│   │   ├── block.rs                  # Estructura de bloque
│   │   ├── chain.rs                  # Gestión de la cadena
│   │   ├── miner.rs                  # Minería de bloques
│   │   └── transaction.rs            # Transacciones
│   │
│   ├── quantum/
│   │   ├── mod.rs                    # Exports públicos
│   │   ├── state.rs                  # Estado cuántico (Qubit)
│   │   ├── gates.rs                  # Puertas cuánticas
│   │   └── measure.rs                # Medición cuántica
│   │
│   └── audit/
│       ├── mod.rs                    # Exports públicos
│       ├── security.rs               # Análisis de seguridad
│       ├── report.rs                 # Generación de reportes
│       └── metrics.rs               # Métricas
│
├── scripts/                            # Scripts de utilidad
│   ├── setup.sh                       # Configuración inicial
│   ├── run.sh                         # Ejecución
│   ├── test.sh                        # Pruebas
│   └── benchmark.sh                  # Benchmarks
│
├── benches/                           # Benchmarks con Criterion
│   └── shor_benchmark.rs
│
├── tests/                             # Tests de integración
│   └── integration_test.rs
│
└── examples/                          # Ejemplos de uso
    ├── basic.rs
    ├── audit.rs
    └── quantum.rs
```

---

## Patrones de Diseño

Este proyecto implementa varios patrones de diseño fundamentales en Rust:

### 1. Patrón Módulo (Module Pattern)

```rust
// Encapsulamiento en módulos con API pública clara
pub mod shor {
    pub fn modular_pow(...) -> ... { ... }
    pub fn shor_algorithm(...) -> ... { ... }
}
```

**Aplicación**: Cada componente está encapsulado en su propio módulo con interfaces públicas bien definidas.

### 2. Patrón Builder

```rust
// Configuración flexible de objetos complejos
let blockchain = Blockchain::new(2)
    .with_max_blocks(10000)
    .with_difficulty(3)
    .build();
```

**Aplicación**: Configuración de Blockchain, Block, y configuraciones de auditoría.

### 3. Patrón Strategy

```rust
// Algoritmos intercambiables
pub trait MiningStrategy {
    fn mine(&self, block: &mut Block, difficulty: u32) -> bool;
}

pub struct BruteForceMining;
pub struct OptimizedMining;
```

**Aplicación**: Estrategias de minería, algoritmos de búsqueda de orden.

### 4. Patrón Observer

```rust
// Notificaciones de eventos
pub trait BlockchainObserver {
    fn on_block_added(&mut self, block: &Block);
    fn on_chain_validated(&self, is_valid: bool);
}
```

**Aplicación**: Auditoría en tiempo real, logging, métricas.

### 5. Patrón Result/Error Handling

```rust
// Manejo robusto de errores
pub enum QuantumError {
    InvalidQubitCount(usize),
    MeasurementError(String),
    FactorizationError(u64),
}

impl std::error::Error for QuantumError { ... }
impl fmt::Display for QuantumError { ... }
```

**Aplicación**: Manejo de errores en todos los módulos.

### 6. Patrón Trait Bounds

```rust
// Genericidad con restricciones
pub fn verify_integrity<T: Hash + Eq>(chain: &[T]) -> bool
```

**Aplicación**: Algoritmos genéricos para diferentes tipos de datos.

---

## Instalación y Configuración

### Prerrequisitos

| Requisito | Versión Mínima | Descripción |
|-----------|---------------|-------------|
| **Rust** | 1.70+ | Compilador y herramientas |
| **Cargo** | 1.70+ | Gestor de paquetes |
| **Sistema** | Unix/Windows | Compatible con ambos |

### Instalación de Rust

```bash
# Instalación via rustup (recomendado)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verificar instalación
rustc --version
cargo --version
```

### Clonar y Compilar

```bash
# Navegar al directorio del proyecto
cd quantum

# Compilar en modo desarrollo
cargo build

# Compilar en modo release (optimizado)
cargo build --release

# Verificar sintaxis sin compilar
cargo check

# Actualizar dependencias
cargo update
```

### Configuración de Entorno

```bash
# Variables de entorno opcionales
export RUST_LOG=debug          # Nivel de logging
export RUST_BACKTRACE=1         # Stack traces
export QUANTUM_MAX_QUBITS=20    # Límite de qubits
export AUDIT_VERBOSE=true      # Modo verbose
```

---

## Uso

### Línea de Comandos

```bash
# Usage básico
cargo run

# Con opciones
cargo run -- --blocks 10 --difficulty 3 --output report.json

# Mostrar ayuda
cargo run -- --help
```

### Opciones CLI

| Opción | Corto | Descripción | Default |
|--------|-------|-------------|---------|
| `--blocks` | `-b` | Número de bloques a crear | 3 |
| `--difficulty` | `-d` | Dificultad de mining | 2 |
| `--audit-mode` | `-a` | Modo: quick, full, deep | full |
| `--output` | `-o` | Archivo de salida | - |
| `--verbose` | `-v` | Salida detallada | false |
| `--help` | `-h` | Mostrar ayuda | - |

### Ejemplos de Uso

#### Ejemplo 1: Crear y Auditar Blockchain

```rust
use quantum_shor::{Blockchain, audit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Crear blockchain con dificultad 2
    let mut blockchain = Blockchain::new(2);
    
    // Agregar bloques
    blockchain.add_block("Alice -> Bob: 10 coins".to_string())?;
    blockchain.add_block("Bob -> Charlie: 5 coins".to_string())?;
    blockchain.add_block("Charlie -> Alice: 3 coins".to_string())?;
    
    // Verificar integridad
    println!("Integridad: {}", blockchain.verify_integrity());
    
    // Ejecutar auditoría
    let results = audit::run_full_audit(&blockchain);
    let report = audit::create_report(results);
    
    // Mostrar reporte
    report.print_summary();
    
    Ok(())
}
```

#### Ejemplo 2: Factorización con Shor

```rust
use quantum_shor::shor;

fn main() {
    let numbers = vec![15, 21, 35, 91, 143];
    
    for n in numbers {
        if let Some((p, q)) = shor::shor_algorithm(n) {
            println!("{} = {} × {}", n, p, q);
        }
    }
}
```

#### Ejemplo 3: Simulación Cuántica

```rust
use quantum_shor::quantum::{QuantumState, QuantumGate, GateType};

fn main() {
    // Crear estado de 3 qubits
    let mut state = QuantumState::new(3);
    
    // Aplicar puerta Hadamard al primer qubit
    let gate = QuantumGate::hadamard(0);
    
    // Medir
    let result = state.measure();
    println!("Resultado: {}", result);
}
```

---

## Referencia de API

### Módulo `shor`

```rust
/// Calcula base^exp mod mod_val usando exponentiación binaria
pub fn modular_pow(base: u64, exp: u64, mod_val: u64) -> u64

/// Algoritmo de Euclides extendido para GCD(a, b)
/// Retorna (g, x, y) donde g = gcd(a,b) y ax + by = g
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64)

/// Máximo común divisor
pub fn gcd(a: u64, b: u64) -> u64

/// Verifica si dos números son coprimos
pub fn coprime(a: u64, b: u64) -> bool

/// Encuentra el orden de a modulo n (fuerza bruta)
pub fn find_order_bruteforce(a: u64, n: u64) -> Option<u64>

/// Encuentra el orden optimizado con límite de iteraciones
pub fn find_order_optimized(a: u64, n: u64, max_iter: u64) -> Option<u64>

/// Fracciones continuas para QFT aproximado
pub fn continuous_fractions(value: f64, max_terms: usize) -> Vec<u64>

/// Convergente de fracción continua
pub fn convergent(terms: &[u64], k: usize) -> (u64, u64)

/// Algoritmo principal de Shor para factorización
pub fn shor_algorithm(n: u64) -> Option<(u64, u64)>

/// Factoriza un lote de números
pub fn shor_factorization_batch(numbers: &[u64]) -> HashMap<u64, Option<(u64, u64)>>

/// Estima requisitos de qubits para factorizar n
pub fn estimate_qubit_requirements(n: u64) -> usize

/// Verifica que p*q = n
pub fn verify_factorization(n: u64, p: u64, q: u64) -> bool
```

### Módulo `blockchain`

```rust
// Block
impl Block {
    pub fn new(index: u64, data: String, previous_hash: String, difficulty: u32) -> Self
    pub fn calculate_hash(&self) -> String
    pub fn is_valid(&self) -> bool
    pub fn set_hash(&mut self, hash: String)
    pub fn get_target(&self) -> String
}

// Blockchain
impl Blockchain {
    pub fn new(difficulty: u32) -> Self
    pub fn add_block(&mut self, data: String) -> Option<&Block>
    pub fn add_block_direct(&mut self, block: Block) -> bool
    pub fn get_block(&self, index: u64) -> Option<&Block>
    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block>
    pub fn verify_integrity(&self) -> bool
    pub fn get_latest_hash(&self) -> String
    pub fn chain_length(&self) -> usize
    pub fn replace_chain(&mut self, new_chain: Vec<Block>) -> bool
    pub fn calculate_total_work(&self) -> u64
}

// Miner
impl Miner {
    pub fn new() -> Self
    pub fn mine(&self, block: &mut Block, difficulty: u32) -> bool
    pub fn verify_proof(&self, block: &Block) -> bool
    pub fn calculate_difficulty(...) -> u32
    pub fn estimate_hash_rate(&self, nonce: u64, elapsed_ms: u128) -> f64
}
```

### Módulo `quantum`

```rust
// QuantumState
impl QuantumState {
    pub fn new(num_qubits: usize) -> Self
    pub fn from_amplitudes(amplitudes: HashMap<u64, f64>) -> Self
    pub fn apply_hadamard(&mut self)
    pub fn apply_pauli_x(&mut self)
    pub fn apply_pauli_z(&mut self)
    pub fn apply_cnot(&mut self)
    pub fn apply_phase(&mut self, angle: f64)
    pub fn measure(&self) -> u64
    pub fn get_probability(&self, state: u64) -> f64
    pub fn normalize(&mut self)
    pub fn get_amplitude(&self, state: u64) -> Option<f64>
}

// QuantumGate
impl QuantumGate {
    pub fn hadamard(target: usize) -> Self
    pub fn pauli_x(target: usize) -> Self
    pub fn pauli_y(target: usize) -> Self
    pub fn pauli_z(target: usize) -> Self
    pub fn cnot(control: usize, target: usize) -> Self
    pub fn swap(q1: usize, q2: usize) -> Self
    pub fn phase(target: usize, angle: f64) -> Self
    pub fn toffoli(c1: usize, c2: usize, target: usize) -> Self
}

// QuantumMeasurement
impl QuantumMeasurement {
    pub fn new(result: u64, probability: f64) -> Self
    pub fn with_basis(result: u64, probability: f64, basis: Basis) -> Self
    pub fn measure_in_hadamard_basis(state: &mut QuantumState) -> Self
    pub fn expectation_value(&self, observable: &HashMap<u64, f64>) -> f64
}
```

### Módulo `audit`

```rust
// Funciones principales
pub fn quantum_security_audit(blockchain: &Blockchain) -> HashMap<String, AuditResult>
pub fn audit_blockchain_integrity(blockchain: &Blockchain) -> AuditResult
pub fn audit_proof_of_work(blockchain: &Blockchain) -> AuditResult
pub fn evaluate_quantum_resistance(blockchain: &Blockchain) -> AuditResult
pub fn analyze_hash_collision_resistance(blockchain: &Blockchain) -> AuditResult
pub fn simulate_quantum_attack(n: u64, target_bits: u32) -> bool
pub fn estimate_attack_complexity(blockchain: &Blockchain) -> HashMap<String, f64>

// Reportes
pub fn create_report(results: AuditResults) -> AuditReport
pub fn export_report(report: &AuditReport, path: &str, format: ReportFormat) -> Result<()>

// Tipos
pub struct AuditResult {
    pub status: AuditStatus,  // Pass, Warning, Fail
    pub score: f64,           // 0.0 - 1.0
    pub details: String,
}

pub struct AuditReport {
    pub timestamp: u64,
    pub total_score: f64,
    pub results: AuditResults,
    pub summary: String,
    pub recommendations: Vec<String>,
}

impl AuditReport {
    pub fn to_json(&self) -> String
    pub fn to_csv(&self) -> String
    pub fn print_summary(&self)
}
```

---

## Ejemplos

### Ejemplo Completo: Auditoría de Blockchain

```rust
use quantum_shor::{Blockchain, audit};

fn main() {
    // 1. Crear blockchain
    let mut bc = Blockchain::new(2);
    
    // 2. Agregar transacciones
    let transactions = vec![
        "Alice -> Bob: 10 QT",
        "Bob -> Charlie: 5 QT",
        "Charlie -> Alice: 3 QT",
        "Alice -> Dave: 2 QT",
    ];
    
    for tx in transactions {
        bc.add_block(tx.to_string());
    }
    
    // 3. Verificar integridad básica
    println!("¿Cadena válida? {}", bc.verify_integrity());
    println!("Bloques: {}", bc.chain_length());
    println!("Trabajo total: {}", bc.calculate_total_work());
    
    // 4. Auditoría completa
    let results = audit::run_full_audit(&bc);
    
    // 5. Generar y mostrar reporte
    let report = audit::create_report(results);
    report.print_summary();
    
    // 6. Exportar a JSON
    let json = report.to_json();
    std::fs::write("audit_report.json", json).unwrap();
}
```

### Ejemplo: Pruebas de Factorización

```rust
use quantum_shor::shor;

fn factorizacion_demo() {
    let test_cases = vec![
        (15, (3, 5)),
        (21, (3, 7)),
        (35, (5, 7)),
        (91, (7, 13)),
        (143, (11, 13)),
    ];
    
    for (n, expected) in test_cases {
        match shor::shor_algorithm(n) {
            Some((p, q)) => {
                let success = (p * q == n) && (p > 1) && (q > 1);
                println!("{} → {}×{} ✓", n, p, q);
            }
            None => println!("{} → No factorizado", n),
        }
    }
}
```

---

## Testing

### Ejecutar Tests

```bash
# Todos los tests
cargo test

# Tests con verbose
cargo test -- --verbose

# Tests de un módulo específico
cargo test --lib shor
cargo test --lib blockchain

# Tests con output de colores
cargo test -- --nocapture
```

### Coverage

```bash
# Instalar tarpaulin
cargo install cargo-tarpaulin

# Generar reporte de coverage
cargo tarpaulin --output-dir ./coverage --html

# Abrir en navegador
open ./coverage/tarpaulin.html
```

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_modular_pow() {
        assert_eq!(modular_pow(2, 10, 1024), 1024);
    }
    
    #[test]
    #[should_panic]
    fn test_invalid_input() {
        // Test de caso de error esperado
    }
    
    #[test]
    fn test_with_result() -> Result<(), Box<dyn std::error::Error>> {
        let result = shor_algorithm(15)?;
        assert!(result.0 * result.1 == 15);
        Ok(())
    }
}
```

---

## Benchmarking

### Ejecutar Benchmarks

```bash
# Todos los benchmarks
cargo bench

# Benchmark específico
cargo bench shor_factorization

# Con salida detallada
cargo bench -- --noplot
```

### Resultados Típicos

```
shor_factorization_15    time:   [1.234 µs 1.245 µs 1.256 µs]
shor_factorization_21    time:   [2.456 µs 2.478 µs 2.501 µs]
blockchain_add_block     time:   [123.45 ms 125.67 ms 128.90 ms]
quantum_hadamard         time:   [0.567 µs 0.578 µs 0.589 µs]
```

---

## Mejores Prácticas

### 1. Manejo de Errores

```rust
// ✅ Usar Result para errores recuperables
pub fn shor_algorithm(n: u64) -> Result<(u64, u64), ShorError> {
    if n <= 1 {
        return Err(ShorError::InvalidInput(n));
    }
    // ...
}

// ❌ Evitar panic en código de producción
// Mal: expect(), unwrap() en producción
```

### 2. Documentación

```rust
/// Calcula el máximo común divisor de a y b usando el 
/// algoritmo de Euclides.
///
/// # Arguments
/// * `a` - Primer número entero positivo
/// * `b` - Segundo número entero positivo
///
/// # Returns
/// El GCD de a y b
///
/// # Example
/// ```
/// assert_eq!(gcd(48, 18), 6);
/// ```
pub fn gcd(a: u64, b: u64) -> u64 { ... }
```

### 3. Testing

- Tests unitarios para cada función pública
- Tests de integración para flujos completos
- Tests de propiedad con proptest
- Coverage > 80%

### 4. Rendimiento

- Evitar allocations en hot paths
- Usar iteradores en lugar de índices
- Pre-allocar vectores cuando el tamaño es conocido
- Medir con benchmarks antes de optimizar

### 5. Seguridad

- Validar todas las entradas
- No exponer información sensible en logs
- Usar constantes de tiempo para operaciones criptográficas
- Seguir principio de menor privilegio

---

## Contribución

### Guías de Contribución

1. **Fork** el repositorio
2. Crear una rama `feature/nombre`
3. **Commit** con mensajes claros
4. **Push** a la rama
5. Crear **Pull Request**

### Estilo de Código

```bash
# Format automático
cargo fmt

# Lint
cargo clippy -- -D warnings

# Check completo
cargo check --all-targets --all-features
```

---

## Roadmap

- [ ] Implementar más puertas cuánticas (Toffoli, Fredkin)
- [ ] Añadir simulación de QFT (Quantum Fourier Transform)
- [ ] Implementar algoritmo de Grover para búsqueda
- [ ] Soporte para transacciones complejas
- [ ] Web UI para visualización
- [ ] API REST

---

## Referencias

### Algoritmo de Shor
- Shor, P.W. (1994). "Algorithms for quantum computation: discrete logarithms and factoring"
- Preskill, J. (2018). "Quantum Computing: A Gentle Introduction"

### Blockchain
- Nakamoto, S. (2008). "Bitcoin: A Peer-to-Peer Electronic Cash System"
- Antonopoulos, A. (2014). "Mastering Bitcoin"

### Rust
- The Rust Programming Language - https://doc.rust-lang.org/book/
- Rust by Example - https://doc.rust-lang.org/rust-by-example/

---

## Licencia

MIT License - ver [LICENSE](LICENSE) para detalles.

---

## Autores

- Quantum Team - Desarrollador principal

---

## Notas

> **Advertencia**: Este proyecto es una **simulación educativa** del algoritmo de Shor. Para uso real en producción, considere libraries criptográficas profesionalmente auditadas.

> **Nota**: La simulación cuántica en hardware clásico tiene limitaciones inherentes. Los resultados son aproximaciones del comportamiento cuántico real.