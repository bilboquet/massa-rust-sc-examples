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
// Wrapped function to "hide" unsafe and manage serialize/deserialize of the parameters
pub fn echo(arg: Vec<u8>) {
    // TODO: serialize the arguments with protobuf

    // prepend size of the serialize arguments in u32 little endian
    let len: u32 = arg.len().try_into().expect("size fit in u32");
    let mut arg_with_len: Vec<u8> = Vec::with_capacity(len as usize + 4usize);
    arg_with_len.extend(len.to_le_bytes());
    arg_with_len.extend(arg);
    // get the address of the serialized arguments to pass to the function abi
    let arg_ptr: u32 = arg_with_len.as_ptr() as u32;
    unsafe {
        abi_echo(arg_ptr);
    }
}
