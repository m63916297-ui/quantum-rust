#!/bin/bash
# test.sh - Ejecutar pruebas

echo "=== Ejecutando pruebas ==="

# Tests unitarios
echo "1. Tests unitarios:"
cargo test

# Tests con verbose
echo ""
echo "2. Tests con verbose:"
cargo test -- --verbose

# Coverage (si está disponible)
if command -v cargo-tarpaulin &> /dev/null; then
    echo ""
    echo "3. Coverage:"
    cargo tarpaulin --output-dir ./coverage
fi

echo ""
echo "=== Pruebas completadas ==="