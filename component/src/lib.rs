wit_bindgen::generate!("component-world" in "wit");

use crate::exports::wasm_bench_test::component::component::Guest;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct ComponentWorld;

impl Guest for ComponentWorld {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

export!(ComponentWorld);

