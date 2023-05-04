pub(crate) mod abi;
mod allocator;

pub use prost::Message;

pub(crate) fn get_parameters(arg_ptr: u32) -> Vec<u8> {
    allocator::get_parameters(arg_ptr)
}

pub(crate) fn encode_length_prefixed(data: Vec<u8>) -> u32 {
    allocator::encode_length_prefixed(data)
}

// Include the `abi` module, which is generated from the proto files.
include!(concat!(env!("OUT_DIR"), "/abi.rs"));
