// ****************************************************************************

// TODO: move in root module of the sdk
// needed to use #[panic_handler]
#![no_std]
// #![feature(alloc_error_handler)]

// as we go no_std we need to import alloc
extern crate alloc;
use alloc::{format, string::ToString};

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(test)] {
    } else {
        use sdk::abis::abort::abort;
        #[panic_handler]
        fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
            abort("my panic".into());
            unreachable!()
        }
    }
}

// #[alloc_error_handler]
// fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
//     panic!("memory allocation of {} bytes failed", layout.size())
// }

// ****************************************************************************
// try dlmalloc allocator, the default allocator when target is
// wasm32-unknown-unknown
// ****************************************************************************

#[global_allocator]
static A: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;


// ****************************************************************************
// try lol_alloc allocator
// ****************************************************************************
// extern crate alloc;

// use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

// // SAFETY: This application is single threaded, so using AssumeSingleThreaded
// is // allowed.
// #[global_allocator]
// static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
//     unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };
// ****************************************************************************

#[no_mangle]
pub extern "C" fn main(_arg: i32) -> i32 {
    panic!("end");
}

mod sdk;

use sdk::{
    abis::{echo::echo, log::log},
    encode_length_prefixed, get_parameters,
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
    panic!(" ** here I am **");

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
    let result = alloc::string::String::from_utf8_lossy(&result);

    assert_eq!(result, "test");
}
