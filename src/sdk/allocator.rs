// ****************************************************************************
// As we go no_std we need to import alloc and use a global allocator
// Using dlmalloc allocator, the default allocator when target is
// wasm32-unknown-unknown
// ****************************************************************************
extern crate alloc;

#[global_allocator]
static A: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

// ****************************************************************************
// may try lol_alloc allocator
// ****************************************************************************
// extern crate alloc;

// use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

// // SAFETY: This application is single threaded, so using AssumeSingleThreaded
// is // allowed.
// #[global_allocator]
// static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
//     unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };
// ****************************************************************************

// ****************************************************************************
// __alloc function used by to host to allocate memory for for exchange with the
// wasm module
// ****************************************************************************

use alloc::vec::Vec;

static mut SHARED_MEM: Vec<u8> = Vec::new();

#[cfg(test)]
static mut IS_SHARED_MEM_CONSUMED: bool = true;

#[no_mangle]
#[export_name = "__alloc"]
fn myalloc(size: u32) -> u32 {
    unsafe {
        #[cfg(test)]
        {
            use log::warn;

            if !IS_SHARED_MEM_CONSUMED {
                warn!("SHARED_MEM has not been consumed yet, possible memory leak");
            }
            IS_SHARED_MEM_CONSUMED = false;
        }

        SHARED_MEM = Vec::with_capacity(size as usize);
        SHARED_MEM.as_ptr() as u32
    }
}

fn get_shared_mem_as_u32() -> u32 {
    unsafe { SHARED_MEM.as_ptr() as u32 }
}

pub(super) fn get_parameters(arg_ptr: u32) -> Vec<u8> {
    // This is a check to ensure that the memory is not consumed twice.
    // Only useful for tests while developing.
    // The assert below has a broader scope but is less explicit.
    #[cfg(test)]
    unsafe {
        if IS_SHARED_MEM_CONSUMED {
            panic!(
                "SHARED_MEM has been consumed, get_parameters() called twice \
                without new memory allocation."
            );
        }
    }

    assert_eq!(arg_ptr, get_shared_mem_as_u32());

    // take the parameter
    unsafe {
        #[cfg(test)]
        {
            IS_SHARED_MEM_CONSUMED = true;
        }

        core::mem::take(&mut SHARED_MEM)
    }
}

pub(super) fn encode_length_prefixed(data: Vec<u8>) -> u32 {
    let data_len: u32 = data.len().try_into().expect("size fit in u32");
    let buf_len: u32 = data_len + 4;

    unsafe {
        // allocate memory and bind it to our global buffer
        myalloc(buf_len);

        SHARED_MEM.extend(data_len.to_le_bytes());
        SHARED_MEM.extend(data);

        // return the pointer to the global buffer
        SHARED_MEM.as_ptr() as u32
    }
}

#[cfg(test)]
// The below functions will only be compiled and available during tests,
pub(super) mod test {
    use super::alloc::vec::Vec;
    use crate::sdk::allocator::{get_parameters, myalloc, SHARED_MEM};

    #[cfg(test)]
    // Function that writes the [u8] argument to SHARED_MEM
    pub fn host_write_buffer(data: &[u8]) -> u32 {
        let buf_ptr = myalloc(data.len().try_into().expect("size fit in u32"));

        unsafe {
            SHARED_MEM.extend_from_slice(data);
        }
        buf_ptr
    }

    #[cfg(test)]
    // Function that reads the [u8] argument from SHARED_MEM
    pub(crate) fn host_read_buffer(arg_ptr: u32) -> Vec<u8> {
        let arg = get_parameters(arg_ptr);

        // get the first 4 bytes of arg in a array of u8
        let arg_len: [u8; 4] = arg[0..4].try_into().expect(
            "First 4 bytes of arg must contain the length of the argument",
        );

        // The first 4 bytes of arg are the length of the argument in little
        // endian
        let arg_len = u32::from_le_bytes(arg_len) as u32; // can't fail

        // verify that the length is correct
        assert_eq!(arg_len + 4, arg.len() as u32);

        // return the argument
        arg[4..].to_vec()
    }
}
