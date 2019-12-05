use std::panic;

#[no_mangle]
pub extern fn handleRequest(_ptr: i32, _len: i32) {
    panic!("time to panic");
}