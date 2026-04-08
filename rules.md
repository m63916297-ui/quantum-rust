# Reglas y Directrices - Algoritmo Cuántico de Shor

## Reglas de Implementación

### 1. Seguridad

- **NUNCA** exponga claves privadas o seeds en logs
- **NUNCA** guarde datos sensibles en texto plano
- Validar todas las entradas del usuario
- Usar primitivas criptográficas verificadas

### 2. Código

- Usar tipos explícitos en funciones públicas
- Documentar funciones complejas
- Manejar errores explícitamente
- Preferir `Result<T, E>` sobre `panic!`

### 3. Tests

- Toda función pública debe tener tests unitarios
- Tests de integración para flujos completos
- Coverage mínimo: 80%
- Verificar casos límite

### 4. Rendimiento

- Optimizar operaciones críticas
- Usar algoritmos O(n log n) o mejores
- Evitar allocations innecesarias
- Medir con benchmarks

## Directrices de Arquitectura

### Modularidad

```
quantum/
├── core/           # Algoritmos fundamentales
├── blockchain/     # Lógica de blockchain
├── quantum/        # Simulación cuántica
├── audit/          # Análisis de seguridad
└── cli/            # Interfaz de línea de comandos
```

### Interfaces

- Módulos públicos con APIs claras
- Traits para abstracciones
- Configuración via archivos o environment

### Errores

- Usar `thiserror` para errores personalizados
- Contextualizar mensajes de error
- Logging estructurado

## Configuración

### Variables de Entorno

```bash
# Nivel de logging
RUST_LOG=debug

# Dificultad de mining
MINING_DIFFICULTY=4

# Modo de auditoría
AUDIT_MODE=full
```

### Archivo de Configuración

```yaml
blockchain:
  difficulty: 2
  max_blocks: 10000

shor:
  max_iterations: 100
  max_qubits: 20

audit:
  verbose: true
  output_format: json
```

## Estándares de Calidad

### Rust

- Usar `rustfmt` para formatting
- `clippy` para linting
- `rustc` con warnings como errores

### Commits

- Formato conventional commits
- Scope claro (fix, feat, refactor, docs)
- Descripción concisa

## Dependencias Recomendadas

```toml
[dependencies]
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
log = "0.4"
env_logger = "0.10"
rand = "0.8"

[dev-dependencies]
criterion = "0.5"
proptest = "1.0"
```

## Proceso de Release

1. Ejecutar `cargo test`
2. Ejecutar `cargo clippy -- -D warnings`
3. Actualizar version en `Cargo.toml`
4. Crear tag git
5. Build de release

## Contacto

Para Issues o contribuciones, contactar al equipo de desarrollo.