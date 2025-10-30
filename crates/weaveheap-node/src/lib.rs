use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn version_major() -> i32 {
    weaveheap_capi::weaveheap_version_major()
}
