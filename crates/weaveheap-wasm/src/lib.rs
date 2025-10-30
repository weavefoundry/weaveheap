#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn weaveheap_wasm_version_major() -> i32 {
    0
}

#[cfg(not(target_arch = "wasm32"))]
pub fn weaveheap_wasm_version_major() -> i32 {
    0
}
