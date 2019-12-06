
// Define a function that is imported into the module.
// By default, the "env" namespace is used.
extern "C" {
    fn ext_set_request_method(ptr: *const u8, len: usize);
}

#[no_mangle]
pub extern fn handleRequest(_ptr: i32, _len: i32) {
    
    let method = "DELETE";
    unsafe {
        ext_set_request_method(method.as_ptr(), method.len());
    }
}