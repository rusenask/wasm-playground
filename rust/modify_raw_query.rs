// Define a function that is imported into the module.
// By default, the "env" namespace is used.
extern "C" {
    // ext_set_request_raw_query function allows updating request query, 
    // for example request "https://example.com/api/foo?foo=bar" raw query is foo=bar
    // so we can here update it to something=else. 
    fn ext_set_request_raw_query(ptr: *const u8, len: usize);    
}

#[no_mangle]
pub extern fn handleRequest(_ptr: i32, _len: i32) {
    let new_url_query = "something=else";
    unsafe {
        ext_set_request_raw_query(new_url_query.as_ptr(), new_url_query.len());
    }
}