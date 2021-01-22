use crate::memory::address::{PhysicalAddress, PhysicalPageNumber};

pub struct FrameTracker(pub(super) PhysicalPageNumber);

impl FrameTracker {
    pub fn address(&self) -> PhysicalAddress {
        self.0.into()
    }
}


