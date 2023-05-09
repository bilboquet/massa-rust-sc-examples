use cfg_if::cfg_if;

use crate::sdk::allocator::encode_length_prefixed;

// ******************************************************
// Function from the abi used by the SC

// Interface between the sdk and the abi

// specify the "namespace" of the function being imported i.e. "massa"
#[link(wasm_import_module = "massa")]
// specify the function name as it is in the abi
// CHECK: extern "C" implies no_mangle
extern "C" {
    // may be use to "rename" a function
    // #[link_name = "actual_symbol_name"]
    fn abi_abort(arg: u32) -> u32;
}

// ******************************************************

// Interface between the sdk and the SC i.e. seen by the user
// Wrapped function to "hide" unsafe and manage serialize/deserialize of the
// parameters
fn impl_abort(arg: String) {
    let ptr = encode_length_prefixed(arg.into_bytes());

    // call the function from the abi
    unsafe { abi_abort(ptr) };
}

// ******************************************************
// mocked version of the abi so one can dev and write tests without the need
// to call the host
cfg_if! {
    if #[cfg(test)] {
        // Should we leave it up to the user to implement the mock?
        // Should we mock at the abi_level?
        // Can mockall do the job?
        fn mock_abort(arg: String)  {
            dbg!("mocked abort");
        }
    }
}

pub fn abort(arg: String) {
    cfg_if! {
        if #[cfg(test)]    {
            mock_abort(arg)
        }
         else {
            impl_abort(arg)
        }
    }
}