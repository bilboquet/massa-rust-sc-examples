// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     pub fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }

// #[wasm_bindgen]
// pub fn next_int(value: u32) {
//     println!("next_int2: {}", value);
// }

mod sdk;
use prost::Message;
// use proto::massa::abi::v1 as proto;

use sdk::{abis::echo, get_parameters};

use crate::sdk::encode_length_prefixed;

// ******************************************************
// Function exposed by the SC low level interface to the host
// CHECK: is it required? (as we use export_name)
#[no_mangle]
// specify the function name as it is seems from the outside
#[export_name = "call_echo"]
pub fn call_echo(arg_ptr: u32) -> u32 {
    let arg = get_parameters(arg_ptr);

    let data = arg;

    // data MUST be returned this way
    return encode_length_prefixed(data);

    dbg!(1);
    // get the parameters pointed by arg_ptr
    let arg: Vec<u8> = get_parameters(encode_length_prefixed(data));
    // deserialize the parameters using protobuf
    let arg: Vec<u8> = Message::decode(arg.as_slice()).unwrap();

    // TODO: deserialize the arguments with protobuf
    echo(arg);
    // alert("");
    // println!("next_int2: {}", value);

    // fake return
    0
}

#[test]
fn test_call_echo() {
    let test_msg = "test".to_string().into_bytes();

    // simulate arguments passing from the host to the SC
    let buf_ptr = sdk::test::host_write_buffer(&test_msg);

    // call the SC function and get the result
    let result_ptr = call_echo(buf_ptr);

    // simulate reading the result from the SC
    let result = sdk::test::host_read_buffer(result_ptr);

    // decode the result from the SC
    let result = String::from_utf8_lossy(&result);

    assert_eq!(result, "test");
}
