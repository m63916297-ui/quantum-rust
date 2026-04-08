use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quantum_shor::{shor, blockchain::Blockchain, quantum::QuantumState};

fn bench_shor_factorization(c: &mut Criterion) {
    c.bench_function("shor_factorization_15", |b| {
        b.iter(|| shor::shor_algorithm(black_box(15)));
    });
    
    c.bench_function("shor_factorization_21", |b| {
        b.iter(|| shor::shor_algorithm(black_box(21)));
    });
    
    c.bench_function("shor_factorization_35", |b| {
        b.iter(|| shor::shor_algorithm(black_box(35)));
    });
}

fn bench_modular_pow(c: &mut Criterion) {
    c.bench_function("modular_pow_small", |b| {
        b.iter(|| shor::modular_pow(black_box(2), black_box(100), black_box(1000)));
    });
    
    c.bench_function("modular_pow_large", |b| {
        b.iter(|| shor::modular_pow(black_box(12345), black_box(67890), black_box(100000)));
    });
}

fn bench_blockchain_add(c: &mut Criterion) {
    c.bench_function("blockchain_add_block", |b| {
        b.iter(|| {
            let mut bc = Blockchain::new(1);
            bc.add_block(black_box("test data".to_string()))
        });
    });
}

fn bench_blockchain_verify(c: &mut Criterion) {
    c.bench_function("blockchain_verify", |b| {
        b.iter(|| {
            let mut bc = Blockchain::new(1);
            bc.add_block("block1".to_string());
            bc.add_block("block2".to_string());
            bc.add_block("block3".to_string());
            bc.verify_integrity()
        });
    });
}

fn bench_quantum_state(c: &mut Criterion) {
    c.bench_function("quantum_state_creation", |b| {
        b.iter(|| QuantumState::new(black_box(4)));
    });
    
    c.bench_function("quantum_hadamard", |b| {
        b.iter(|| {
            let mut state = QuantumState::new(black_box(3));
            state.apply_hadamard();
        });
    });
}

fn bench_find_order(c: &mut Criterion) {
    c.bench_function("find_order_small", |b| {
        b.iter(|| shor::find_order_bruteforce(black_box(7), black_box(15)));
    });
    
    c.bench_function("find_order_optimized", |b| {
        b.iter(|| shor::find_order_optimized(black_box(7), black_box(15), black_box(20)));
    });
}

criterion_group!(
    benches,
    bench_shor_factorization,
    bench_modular_pow,
    bench_blockchain_add,
    bench_blockchain_verify,
    bench_quantum_state,
    bench_find_order
);
criterion_main!(benches);