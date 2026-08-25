#[link(wasm_import_module = "")]
unsafe extern "C" {
    fn add_host_y(value: i32) -> i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[unsafe(no_mangle)]
pub extern "C" fn add_host_x(x: i32) -> i32 {
    x + unsafe { add_host_y(x) }
}
