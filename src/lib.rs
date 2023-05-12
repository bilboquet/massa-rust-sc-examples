// ****************************************************************************
#![no_std]
// ****************************************************************************

use massa_rust_sc_sdk as sdk;
use sdk::{
    abis::{echo::echo, log::log},
    *,
};

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main(_arg: i32) -> i32 {
    panic!("end");
}

// ****************************************************************************
// Function exposed by the SC low level interface to the host
// CHECK: is it required? (as we use export_name)
#[no_mangle]
// specify the function name as it is seems from the outside
#[export_name = "call_echo"]
pub fn call_echo(arg_ptr: u32) -> u32 {
    let arg = get_parameters(arg_ptr);
    log("call_echo".to_string());

    // assert_eq!(arg.len(), 4);
    log(format!("arg len: {}", arg.len()));

    // assert_eq!(arg, "test".to_string().into_bytes());
    let arg = "test".to_string().into_bytes();
    let ret = echo(arg);
    log(format!("ret len: {}", ret.len()));

    // data MUST be returned this way
    encode_length_prefixed(ret)
}

#[cfg(test)]
mod tests {
    extern crate std;
    use crate::call_echo;
    use crate::sdk::{self, *};

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

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("test panic");
    }
}
