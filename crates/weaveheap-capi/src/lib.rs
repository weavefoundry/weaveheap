use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
pub struct weaveheap_error {
    pub code: i32,
    pub message: *const c_char,
}

#[no_mangle]
pub extern "C" fn weaveheap_error_clear(err: *mut weaveheap_error) {
    if err.is_null() {
        return;
    }
    unsafe {
        if !(*err).message.is_null() {
            // Safety: Only valid if message was allocated via CString::into_raw
            let _ = CString::from_raw((*err).message as *mut c_char);
        }
        (*err).code = 0;
        (*err).message = ptr::null();
    }
}

#[no_mangle]
pub extern "C" fn weaveheap_version_major() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn weaveheap_version_minor() -> i32 {
    1
}

#[no_mangle]
pub extern "C" fn weaveheap_version_patch() -> i32 {
    0
}
