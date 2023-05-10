// ****************************************************************************

// TODO: move in root module of the sdk
// needet to use #[panic_handler]
// #![no_std]

// ****************************************************************************

#[no_mangle]
pub extern "C" fn main(_arg: i32) -> i32 {
    panic!("end");
}

mod sdk;

use crate::sdk::abis::log::log;
use crate::sdk::encode_length_prefixed;
use sdk::{
    abis::{abort::abort, echo::echo},
    get_parameters,
};

// ****************************************************************************
// Function exposed by the SC low level interface to the host
// CHECK: is it required? (as we use export_name)
#[no_mangle]
// specify the function name as it is seems from the outside
#[export_name = "call_echo"]
pub fn call_echo(arg_ptr: u32) -> u32 {
    let arg = get_parameters(arg_ptr);
    log("call_echo".to_string());
    // panic!(" ** here I am **");
    abort("here I am".to_owned());

    // assert_eq!(arg.len(), 4);
    log(format!("arg len: {}", arg.len()));

    // assert_eq!(arg, "test".to_string().into_bytes());
    let arg = "test".to_string().into_bytes();
    let ret = echo(arg);
    log(format!("ret len: {}", ret.len()));

    // data MUST be returned this way
    encode_length_prefixed(ret)
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
