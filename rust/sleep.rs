use std::str;
use std::slice;

// Define a function that is imported into the module.
// By default, the "env" namespace is used.
extern "C" {
    fn ext_set_request_body(ptr: *const u8, len: usize);
    fn ext_debug_sleep(duration: i32);
}

#[no_mangle]
pub extern fn handleRequest(ptr: i32, len: i32) {
    let slice = unsafe { slice::from_raw_parts(ptr as _, len as _) };
    let string_from_host = str::from_utf8(&slice).unwrap();
    let out_str = format!("after sleep: {}", string_from_host);

    // sleeping for 5 seconds, it should terminate after 3
    // let five_secs = time::Duration::from_millis(5000);

    // thread::sleep(five_secs);    
    unsafe {
        ext_debug_sleep(2000);

        ext_set_request_body(out_str.as_ptr(), out_str.len());
    }
}