use std::str;
use std::slice;

// Define a function that is imported into the module.
// By default, the "env" namespace is used.
extern "C" {
    fn ext_set_request_body(ptr: *const u8, len: usize);
    fn ext_set_request_header(key_ptr: *const u8, key_len: usize, val_ptr: *const u8, val_len: usize);
}

#[no_mangle]
pub extern fn handleRequest(ptr: i32, len: i32) {
    let slice = unsafe { slice::from_raw_parts(ptr as _, len as _) };
    let string_from_host = str::from_utf8(&slice).unwrap();
    let out_str = format!("modified: {}", string_from_host);
    
    let new_header_key = "rust";
    let new_header_val = "wasm";
    unsafe {
        // ext_set_request_method(method.as_ptr(), method.len());
        ext_set_request_header(
            new_header_key.as_ptr(), new_header_key.len(),        
            new_header_val.as_ptr(), new_header_val.len(),        
        );

        ext_set_request_body(out_str.as_ptr(), out_str.len());
    }
}