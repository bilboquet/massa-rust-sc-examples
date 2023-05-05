mod abi;
pub(crate) mod abis;
mod allocator;

// pub use prost::Message;

pub(crate) fn get_parameters(arg_ptr: u32) -> Vec<u8> {
    allocator::get_parameters(arg_ptr)
}

pub(crate) fn encode_length_prefixed(data: Vec<u8>) -> u32 {
    allocator::encode_length_prefixed(data)
}

#[cfg(test)]
pub(crate) mod test {
    use super::allocator;

    // The below functions will only be compiled and available during tests,
    #[cfg(test)]
    /// Simulate arguments passed by the host to the SC
    pub fn host_write_buffer(data: &[u8]) -> u32 {
        allocator::test::host_write_buffer(data)
    }

    #[cfg(test)]
    /// Simulate reading arguments passed by the host to the SC
    pub fn host_read_buffer(arg_ptr: u32) -> Vec<u8> {
        allocator::test::host_read_buffer(arg_ptr)
    }
}
