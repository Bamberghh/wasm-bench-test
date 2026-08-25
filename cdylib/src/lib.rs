#[unsafe(no_mangle)]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[unsafe(no_mangle)]
pub extern "C" fn add_host_x(add_host_y: extern "C" fn(i32) -> i32, x: i32) -> i32 {
    x + add_host_y(x)
}
