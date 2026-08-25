Probably useless microbenchmarks to satisfy my curiousity in Wasmtime call overheads.

To run the benchmarks run `cargo build --release` in each of the module types: "cdylib", "module", "component".

Each of the benchmarked functions is just an `add` function which adds two `i32`s, implemented in the respective form.

Criterion results for my machine:
```
// Dynamic library call
rust_to_cdylib                         time:   [1.9327 ns 1.9619 ns 1.9960 ns]
rust_to_cdylib_to_rust                 time:   [2.6319 ns 2.6324 ns 2.6330 ns]
// WASM core module call
rust_to_wasm_module                    time:   [26.592 ns 26.616 ns 26.641 ns]
rust_to_wasm_module_to_rust            time:   [33.568 ns 33.647 ns 33.725 ns]
// WASM component call
rust_to_wasm_component                 time:   [683.36 ns 684.22 ns 685.13 ns]
rust_to_wasm_component_to_rust         time:   [922.96 ns 924.81 ns 926.57 ns]
// WASM component call, but with concurrency support (e.g. WASIp3 async) turned off
rust_to_wasm_component_noasync         time:   [186.81 ns 186.93 ns 187.07 ns]
rust_to_wasm_component_noasync_to_rust time:   [263.56 ns 264.02 ns 264.45 ns]
```

While I do understand why calling WASM is slower than calling a native dylib, I don't understand why the component call has a 25x higher time than the core module call.
