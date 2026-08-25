wit_bindgen::generate!("component-world" in "wit");

use crate::exports::wasm_bench_test::component::component::Guest;
use crate::wasm_bench_test::component::host;

struct ComponentWorld;

impl Guest for ComponentWorld {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    fn add_host_x(x: i32) -> i32 {
        x + host::add_host_y(x)
    }
}

export!(ComponentWorld);
