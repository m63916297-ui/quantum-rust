#!/bin/bash
# setup.sh - Configuración del entorno de desarrollo

echo "=== Configuración del entorno Quantum Shor ==="

# Verificar Rust
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust no está instalado"
    echo "Instala Rust desde: https://rustup.rs/"
    exit 1
fi

echo "Versión de Rust:"
rustc --version
cargo --version

# Verificar estructura del proyecto
echo ""
echo "=== Estructura del proyecto ==="
ls -la

echo ""
echo "=== Compilando proyecto ==="
cargo build --release

echo ""
echo "=== Configuración completada ==="