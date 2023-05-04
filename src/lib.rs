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
use sdk::{abi::echo, get_parameters};

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
    // echo(arg);
    // alert("");
    // println!("next_int2: {}", value);

    // fake return
    0
}


#[test]
fn test_call_echo() {
    // create a TestRequest instance
use crate::sdk::proto::TestRequest;

let request = TestRequest {
    test_field: "test".to_string(),
};

// encode the TestRequest instance using protobuf
let mut buf = Vec::new();
request.encode(&mut buf).unwrap();

// pass the encoded TestRequest as argument to the SC function
let arg_ptr = sdk::store_data(&buf);

// call the SC function and get the result
let result_ptr = call_echo(arg_ptr);

// decode the result from the SC function using protobuf
let result_buf = sdk::get_data(result_ptr);
let result = TestResponse::decode(result_buf.as_slice()).unwrap();
    call_echo(1);
}
