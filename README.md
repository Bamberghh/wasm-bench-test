Probably useless microbenchmarks to satisfy my curiousity in Wasmtime call overheads.

To run the benchmarks run `cargo build --release` in each of the module types.

Each of the benchmarked functions is just an `add` function which adds two `i32`s, implemented in the respective form.

Criterion results for my machine:
```
rust_to_cdylib          time:   [1.9623 ns 1.9739 ns 1.9870 ns] // Dynamic library call
rust_to_wasm_module     time:   [26.165 ns 26.210 ns 26.255 ns] // WASM core module call
rust_to_wasm_component  time:   [669.92 ns 671.71 ns 673.65 ns] // WASM component call
```

While I do understand why calling WASM is slower than calling the native dylib, I don't understand why the component call has a 25x higher time than the core module call.
