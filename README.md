# Algoritmo Cuántico de Shor - Auditoría de Blockchain

## Descripción

Implementación del algoritmo de Shor para auditoría de seguridad cuántica en blockchains. Este proyecto proporciona herramientas para analizar la resistencia de una cadena de bloques ante ataques cuánticos mediante factorización de enteros.

## Características

- **Algoritmo de Shor**: Factorización de enteros usando métodos clásicos optimizados
- **Auditoría de Blockchain**: Verificación de integridad y resistencia cuántica
- **Simulación de Ataques**: Pruebas de vulnerabilidad cuántica
- **Generación de Bloques**: Creación de blockchain con proof-of-work

## Arquitectura del Sistema

```
quantum/
├── src/
│   ├── main.rs           # Punto de entrada
│   ├── shor/
│   │   ├── mod.rs        # Módulo de algoritmo de Shor
│   │   ├── order_finding.rs    # Búsqueda de orden
│   │   └── factorizacion.rs    # Factorización
│   ├── blockchain/
│   │   ├── mod.rs        # Módulo blockchain
│   │   ├── block.rs      # Estructura de bloques
│   │   ├── chain.rs      # Gestión de cadena
│   │   └── miner.rs      # Minería de bloques
│   ├── quantum/
│   │   ├── mod.rs        # Estado cuántico simulado
│   │   ├── gates.rs      # Puertas cuánticas
│   │   └── measure.rs    # Medición
│   └── audit/
│       ├── mod.rs        # Módulo de auditoría
│       ├── security.rs   # Análisis de seguridad
│       └── report.rs     # Reportes
├── scripts/
│   ├── run.sh            # Script de ejecución
│   ├── test.sh           # Pruebas
│   └── benchmark.sh      # Benchmarks
├── Cargo.toml
├── README.md
└── rules.md
```

## Instalación

```bash
# Clonar o navegar al directorio
cd quantum

# Compilar
cargo build --release

# Ejecutar
cargo run
```

## Uso

### Ejecución Básica

```bash
cargo run
```

### Ejecutar Pruebas

```bash
cargo test
```

### Benchmark

```bash
cargo bench
```

## Estructura del Código

### Módulos Principales

#### 1. Algoritmo de Shor (`shor/`)

```rust
// Factorización usando algoritmo de Shor
pub fn shor_algorithm(n: u64) -> Option<(u64, u64)>

// Búsqueda de orden
pub fn find_order(a: u64, n: u64) -> Option<u64>

// Potencia modular
pub fn modular_pow(base: u64, exp: u64, mod_val: u64) -> u64
```

#### 2. Blockchain (`blockchain/`)

```rust
// Crear nuevo bloque
Block::new(index, data, previous_hash)

// Minar bloque
block.mine(difficulty)

// Verificar integridad
blockchain.verify_integrity()
```

#### 3. Estado Cuántico (`quantum/`)

```rust
// Inicializar estado cuántico
QuantumState::new(num_qubits)

// Aplicar puerta Hadamard
state.apply_hadamard()

// Medir estado
state.measure()
```

#### 4. Auditoría (`audit/`)

```rust
// Auditoría de seguridad cuántica
quantum_security_audit(blockchain)

// Simular ataque cuántico
simulate_quantum_attack(n, target_bits)

// Generar reporte
generate_report(audit_results)
```

## Ejemplo de Uso

```rust
use quantum_shor::*;

fn main() {
    // Crear blockchain
    let mut blockchain = Blockchain::new(2);
    
    // Agregar bloques
    blockchain.add_block("Transacción 1".to_string());
    blockchain.add_block("Transacción 2".to_string());
    
    // Auditoría cuántica
    let results = quantum_security_audit(&blockchain);
    
    // Imprimir resultados
    for (key, value) in results {
        println!("{}: {}", key, value);
    }
}
```

## Métodos del Algoritmo de Shor

### 1. Búsqueda de Orden

El algoritmo de Shor comienza encontrando el orden de `a` módulo `n`, donde `a` es un número aleatorio coprimo con `n`.

```rust
fn find_order(a: u64, n: u64) -> Option<u64> {
    for r in 1..=n {
        if modular_pow(a, r, n) == 1 {
            return Some(r);
        }
    }
    None
}
```

### 2. Factorización

Una vez encontrado el orden `r`, si `r` es par, podemos factorizar `n`:

```rust
fn factor_from_order(a: u64, r: u64, n: u64) -> Option<(u64, u64)> {
    let x = modular_pow(a, r / 2, n);
    let (g1, _, _) = extended_gcd(x as i64 + 1, n as i64);
    let (g2, _, _) = extended_gcd(x as i64 - 1, n as i64);
    
    if g1 > 1 && g1 < n { return Some((g1 as u64, n / g1 as u64)); }
    if g2 > 1 && g2 < n { return Some((g2 as u64, n / g2 as u64)); }
    None
}
```

## Estado Cuántico Simulado

Para simular estados cuánticos en hardware clásico:

```rust
struct QuantumState {
    amplitudes: HashMap<u64, f64>,
}

impl QuantumState {
    fn apply_hadamard(&mut self) {
        // Superposición cuántica
    }
    
    fn measure(&self) -> u64 {
        // Colapso de función de onda
    }
}
```

## Scripts

### run.sh

```bash
#!/bin/bash
echo "Ejecutando algoritmo cuántico de Shor..."
cargo run
```

### test.sh

```bash
#!/bin/bash
echo "Ejecutando pruebas..."
cargo test --verbose
```

### benchmark.sh

```bash
#!/bin/bash
echo "Ejecutando benchmarks..."
cargo bench
```

## Referencias

- Shor, P.W. (1994). "Algorithms for quantum computation: discrete logarithms and factoring"
- Nielsen, M.A. & Chuang, I.L. (2010). "Quantum Computation and Quantum Information"

## Licencia

MIT License