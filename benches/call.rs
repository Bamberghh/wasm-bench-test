use criterion::{Criterion, criterion_group, criterion_main};
use wasm_bench_test::{ComponentHost, ModuleHost};
use std::hint::black_box;


fn rust_to_wasm_module(c: &mut Criterion) {
    let mut host = ModuleHost::new().unwrap();
    c.bench_function("rust_to_wasm_module", |b| {
        b.iter(|| host.call_add(black_box(1), black_box(2)).unwrap())
    });
}

fn rust_to_wasm_component(c: &mut Criterion) {
    let mut host = ComponentHost::new().unwrap();
    c.bench_function("rust_to_wasm_component", |b| {
        b.iter(|| host.call_add(black_box(1), black_box(2)).unwrap())
    });
}

criterion_group!(
    benches,
    rust_to_wasm_module,
    rust_to_wasm_component,
);
criterion_main!(benches);

