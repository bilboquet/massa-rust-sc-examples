use cfg_if::cfg_if;
use prost::Message;

// Include the `abi` module, which is generated from the proto files.
use super::{
    abi::proto::massa::abi::v1::{TestRequest, TestResponse},
    encode_length_prefixed, get_parameters,
};

// ******************************************************
// Function from the abi used by the SC

// Interface between the sdk and the abi

// specify the "namespace" of the function being imported i.e. "massa"
#[link(wasm_import_module = "massa")]
// specify the function name as it is in the abi
// CHECK: extern "C" implies no_mangle
extern "C" {
    // maybe use to "rename" a function
    // #[link_name = "actual_symbol_name"]
    fn abi_echo(arg: u32) -> u32;
}

// ******************************************************

// Interface between the sdk and the SC i.e. seen by the user
// Wrapped function to "hide" unsafe and manage serialize/deserialize of the
// parameters
fn impl_echo(arg: Vec<u8>) -> Vec<u8> {
    // serialize the arguments with protobuf
    let req = TestRequest { message_in: arg };
    let req_bytes = req.encode_to_vec();

    let arg_ptr: u32 = encode_length_prefixed(req_bytes);

    // call the function from the abi
    let resp_ptr = unsafe { abi_echo(arg_ptr) };

    // deserialize the returned value with protobuf
    let resp_bytes: Vec<u8> = get_parameters(resp_ptr);
    let resp: TestResponse =
        TestResponse::decode(resp_bytes.as_slice()).unwrap();

    resp.message_out
}

// ******************************************************
// mocked version of the abi so one can dev and write tests without the need to
// call the host
cfg_if! {
    if #[cfg(test)] {
        // Should we leave it up to the user to implement the mock?
        // Should we mock at the abi_level?
        // Can mockall do the job?
        fn mock_echo(arg: Vec<u8>) -> Vec<u8> {
            dbg!("mocked echo");

            let req = TestRequest {message_in: arg };
            let req_bytes = req.encode_to_vec();

            let arg_ptr: u32 = encode_length_prefixed(req_bytes);

            // deserialize the returned value with protobuf
            let resp_bytes: Vec<u8> = get_parameters(arg_ptr);
            // drop the first 4 bytes as they are the length of the message
            let resp_bytes: Vec<u8> = resp_bytes[4..].to_vec();

            let resp: TestResponse = TestResponse::decode(resp_bytes.as_slice()).unwrap();

            resp.message_out
        }
    }
}

pub fn echo(arg: Vec<u8>) -> Vec<u8> {
    cfg_if! {
        if #[cfg(test)]    {
            mock_echo(arg)
        }
         else {
            impl_echo(arg)
        }
    }
}
