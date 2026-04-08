#!/bin/bash
# benchmark.sh - Ejecutar benchmarks

echo "=== Suite de Benchmarks ==="

# Benchmark principal
echo "1. Benchmarks de factorización de Shor:"
cargo bench shor_factorization

echo ""
echo "2. Benchmarks de operaciones de blockchain:"
cargo bench blockchain

echo ""
echo "3. Benchmarks de simulación cuántica:"
cargo bench quantum

echo ""
echo "4. Benchmarks de búsqueda de orden:"
cargo bench find_order

echo ""
echo "=== Benchmarks completados ==="