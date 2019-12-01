// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

use std::str;
use std::slice;

// Define a function that is imported into the module.
// By default, the "env" namespace is used.
extern "C" {
    // fn ext_set_response(ptr: *const u8, len: usize);
    fn ext_set_request_method(ptr: *const u8, len: usize);
    fn ext_set_request_body(ptr: *const u8, len: usize);
}

// Define a string that is accessible within the wasm
// linear memory.
// static HELLO: &'static str = "Hello, World!";

// // Export a function named "hello_wasm". This can be called
// // from the embedder!
// #[no_mangle]
// pub extern fn hello_wasm() {
//     // Call the function we just imported and pass in
//     // the offset of our string and its length as parameters.
//     unsafe {
//         response(HELLO.as_ptr(), HELLO.len());
//     }
// }


// #[no_mangle]
// pub extern fn handleRequest(ptr: i32, len: i32) {
//     let slice = unsafe { slice::from_raw_parts(ptr as _, len as _) };
//     let string_from_host = str::from_utf8(&slice).unwrap();
//     let out_str = format!("{}", string_from_host);
//     unsafe {
//         handleResponse(out_str.as_ptr(), out_str.len());
//     }
// }

#[no_mangle]
pub extern fn handleRequest(ptr: i32, len: i32) {
    let slice = unsafe { slice::from_raw_parts(ptr as _, len as _) };
    let string_from_host = str::from_utf8(&slice).unwrap();
    let out_str = format!("modified: {}", string_from_host);
    
    let method = "DELETE";
    // out_str.as_ptr()
    unsafe {
        // ext_set_response(method.as_ptr(), method.len());
        ext_set_request_method(method.as_ptr(), method.len());
        ext_set_request_body(out_str.as_ptr(), out_str.len());
    }
}