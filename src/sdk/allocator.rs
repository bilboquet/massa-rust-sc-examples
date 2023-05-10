// ****************************************************************************
// section related to panic hook
// ****************************************************************************
// set_panick_hook() has to be setup at runtime
// let's use a static_lazy global variable to trigger the setup
// and acces it in __alloc as we know it will be called before any other
// function

use lazy_static::lazy_static;

lazy_static! {
    static ref PANIC_HOOK: bool = init_panic_handler();
}
fn init_panic_handler() -> bool {
    std::panic::set_hook(Box::new(|panic_info| {
        super::abis::abort::abort(panic_info.to_string());
    }));
    true
}

// ****************************************************************************

static mut SHARED_MEM: Vec<u8> = vec![];

#[cfg(test)]
static mut IS_SHARED_MEM_CONSUMED: bool = true;

#[no_mangle]
#[export_name = "__alloc"]
fn myalloc(size: u32) -> u32 {
    // trigger the setup of the panic hook
    if *PANIC_HOOK {
        // do nothing
    }

    unsafe {
        #[cfg(test)]
        {
            use log::warn;

            if !IS_SHARED_MEM_CONSUMED {
                warn!("SHARED_MEM has not been consumed yet, possible memory leak");
            }
            IS_SHARED_MEM_CONSUMED = false;
        }

        // allocate AND fill memory with 0
        SHARED_MEM = vec![0u8; size as usize];
        // clear the buffer to extend() writes at the biginning
        SHARED_MEM.clear();
        SHARED_MEM.as_ptr() as u32
    }
}

fn get_shared_mem_as_u32() -> u32 {
    unsafe { SHARED_MEM.as_ptr() as u32 }
}

pub(crate) fn get_parameters(arg_ptr: u32) -> Vec<u8> {
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
        std::mem::take(&mut SHARED_MEM)
    }
}

pub(crate) fn encode_length_prefixed(data: Vec<u8>) -> u32 {
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
