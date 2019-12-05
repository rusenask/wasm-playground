extern crate base64;

use std::slice;
use std::str;

use base64::{decode};

use serde::{Deserialize, Serialize};

// Define a function that is imported into the module.
// By default, the "env" namespace is used.
extern "C" {
    fn ext_set_request_method(ptr: *const u8, len: usize);
    fn ext_set_request_body(ptr: *const u8, len: usize);
    fn ext_set_request_raw_query(ptr: *const u8, len: usize);
    fn ext_set_request_header(
        key_ptr: *const u8,
        key_len: usize,
        val_ptr: *const u8,
        val_len: usize,
    );
}

#[derive(Serialize, Deserialize)]
pub struct PayloadStruct {
    // Use the type's implementation of std::default::Default if
    // "method" or other fields are not included in the input.
    #[serde(default)]
    method: String,
    #[serde(default)]
    raw_query: String,
    #[serde(default)]
    body: String,
}

#[no_mangle]
pub extern "C" fn handleRequest(ptr: i32, len: i32) {
    let slice = unsafe { slice::from_raw_parts(ptr as _, len as _) };
    // need to parse here the contents into some struct where we can get body
    // example: {"body":"some-body-here","method":"PUT", "raw_query": "/foo/bar"}
    let string_from_host = str::from_utf8(&slice).unwrap();

    // let payload: PayloadStruct = serde_json::from_str(string_from_host).unwrap();
    let payload = parse_payload(string_from_host);

    let out_str = format!("{{\"text\": \"{}\"}}", payload.body);
    // let out_str = format!("{{\"text\": \"{}\"}}", string_from_host);
    let method = "POST";
    // slack webhook URL
    let new_query = "/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX";
    
    let new_header_key = "Content-type";
    let new_header_val = "application/json";
    // out_str.as_ptr()
    unsafe {
        // ext_set_response(method.as_ptr(), method.len());
        ext_set_request_method(method.as_ptr(), method.len());
        ext_set_request_body(out_str.as_ptr(), out_str.len());
        ext_set_request_raw_query(new_query.as_ptr(), new_query.len());
        // setting header
        ext_set_request_header(
            new_header_key.as_ptr(),
            new_header_key.len(),
            new_header_val.as_ptr(),
            new_header_val.len(),
        );
    }
}

pub fn parse_payload(payload_string: &str) -> PayloadStruct {
    let mut parsed_payload: PayloadStruct = serde_json::from_str(&payload_string).unwrap();
    parsed_payload.body = String::from_utf8(decode(&parsed_payload.body).unwrap()).unwrap();
    parsed_payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_payload_method() {
        let result = parse_payload("{\"method\":\"POST\"}");
        assert_eq!(result.method, "POST");
    }

    #[test]
    fn test_parse_payload_body() {
        let result = parse_payload("{\"method\":\"POST\", \"body\": \"Zm9vbw==\"}");
        assert_eq!(result.method, "POST");
        assert_eq!(result.body, "fooo");
    }
}