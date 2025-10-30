#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Handle {
    raw: u64,
}

impl Handle {
    pub fn new(raw: u64) -> Self {
        Self { raw }
    }

    pub fn raw(&self) -> u64 {
        self.raw
    }
}
