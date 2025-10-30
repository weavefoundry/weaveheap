use thiserror::Error;

#[derive(Debug, Error)]
pub enum WeaveHeapError {
    #[error("unimplemented")]
    Unimplemented,
}
