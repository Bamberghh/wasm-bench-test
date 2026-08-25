Probably useless microbenchmarks to satisfy my curiousity in Wasmtime call overheads.

To run the benchmarks run `cargo build --release` in each of the module types: "cdylib", "module", "component".

Each of the benchmarked functions is just an `add` function which adds two `i32`s, implemented in the respective form.

Criterion results for my machine:
```
// Dynamic library call
rust_to_cdylib                 time:   [1.6834 ns 1.6853 ns 1.6874 ns]
rust_to_cdylib_to_rust         time:   [2.6480 ns 2.6505 ns 2.6532 ns]
// WASM core module call
rust_to_wasm_module            time:   [26.853 ns 26.899 ns 26.950 ns]
rust_to_wasm_module_to_rust    time:   [33.701 ns 33.754 ns 33.808 ns]
// WASM component call
rust_to_wasm_component         time:   [669.40 ns 670.67 ns 672.01 ns]
rust_to_wasm_component_to_rust time:   [894.75 ns 896.21 ns 897.69 ns]
```

While I do understand why calling WASM is slower than calling a native dylib, I don't understand why the component call has a 25x higher time than the core module call.
