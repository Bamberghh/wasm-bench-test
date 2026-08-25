use criterion::{Criterion, criterion_group, criterion_main};
use std::{hint::black_box, time::Duration};
use wasm_bench_test::{CDylibHost, ComponentHost, ModuleHost};

fn rust_to_cdylib(c: &mut Criterion) {
    let host = CDylibHost::new().unwrap();
    c.bench_function("rust_to_cdylib", |b| {
        b.iter(|| host.call_add(black_box(1), black_box(2)))
    });
}

fn rust_to_cdylib_to_rust(c: &mut Criterion) {
    let host = CDylibHost::new().unwrap();
    c.bench_function("rust_to_cdylib_to_rust", |b| {
        b.iter(|| host.call_add_host(black_box(1)))
    });
}

fn rust_to_wasm_module(c: &mut Criterion) {
    let mut host = ModuleHost::new().unwrap();
    c.bench_function("rust_to_wasm_module", |b| {
        b.iter(|| host.call_add(black_box(1), black_box(2)).unwrap())
    });
}

fn rust_to_wasm_module_to_rust(c: &mut Criterion) {
    let mut host = ModuleHost::new().unwrap();
    c.bench_function("rust_to_wasm_module_to_rust", |b| {
        b.iter(|| host.call_add_host(black_box(1)).unwrap())
    });
}

fn rust_to_wasm_component(c: &mut Criterion) {
    let mut host = ComponentHost::new().unwrap();
    c.bench_function("rust_to_wasm_component", |b| {
        b.iter(|| host.call_add(black_box(1), black_box(2)).unwrap())
    });
}

fn rust_to_wasm_component_to_rust(c: &mut Criterion) {
    let mut host = ComponentHost::new().unwrap();
    c.bench_function("rust_to_wasm_component_to_rust", |b| {
        b.iter(|| host.call_add_host(black_box(1)).unwrap())
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(10))
        .sample_size(200);
    targets =
        rust_to_cdylib,
        rust_to_cdylib_to_rust,
        rust_to_wasm_module,
        rust_to_wasm_module_to_rust,
        rust_to_wasm_component,
        rust_to_wasm_component_to_rust,
);
criterion_main!(benches);
