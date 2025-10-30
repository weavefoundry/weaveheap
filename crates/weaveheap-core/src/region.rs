#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct RegionId(pub u64);

#[derive(Debug, Default)]
pub struct Region {
    id: RegionId,
}

impl Region {
    pub fn new() -> Self {
        Self { id: RegionId(0) }
    }

    pub fn id(&self) -> RegionId {
        self.id
    }
}
