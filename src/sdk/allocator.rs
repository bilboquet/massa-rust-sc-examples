static mut SHARED_MEM: Vec<u8> = vec![];

#[no_mangle]
#[export_name = "__alloc"]
fn myalloc(size: u32) -> u32 {
    unsafe {
        SHARED_MEM = vec![0u8; size as usize];
        SHARED_MEM.as_ptr() as u32
    }
}

fn get_shared_mem_as_u32() -> u32 {
    unsafe { SHARED_MEM.as_ptr() as u32 }
}

pub(crate) fn get_parameters(arg_ptr: u32) -> Vec<u8> {
    assert_eq!(arg_ptr, get_shared_mem_as_u32());
    // take the parameter
    unsafe { std::mem::take(&mut SHARED_MEM) }
}

pub(crate) fn encode_length_prefixed(data: Vec<u8>) -> u32 {
    let data_len: u32 = data.len().try_into().expect("size fit in u32");

    unsafe {
        // allocate memory and bind it to our global buffer
        SHARED_MEM = Vec::with_capacity(data_len as usize + 4usize);

        SHARED_MEM.extend(data_len.to_le_bytes());
        SHARED_MEM.extend(data);

        // return the pointer to the global buffer
        SHARED_MEM.as_ptr() as u32
    }
}
