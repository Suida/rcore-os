use spin::Mutex;
use algorithm::{AllocatorImpl, Allocator};
use crate::memory::{
    Range,
    MemoryResult,
    config::{
        KERNEL_END_ADDRESS,
        MEMORY_END_ADDRESS
    },
    address::{
        PhysicalAddress,
        PhysicalPageNumber
    },
};
use super::frame_tracker::FrameTracker;

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>> = Mutex::new(
        FrameAllocator::new(Range::from(
            PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))..
            PhysicalPageNumber::floor(MEMORY_END_ADDRESS),
        ))
    );
}

pub struct FrameAllocator<T: Allocator> {
    start_ppn: PhysicalPageNumber,
    allocator: T,
}

impl<T: Allocator> FrameAllocator<T> {
    pub fn new(range: impl Into<Range<PhysicalPageNumber>> + Copy) -> Self {
        Self { 
            start_ppn: range.into().start,
            allocator: T::new(range.into().len()),
        }
    }

    pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        self.allocator
            .alloc()
            .ok_or("no available frame to allocate")
            .map(|offset| FrameTracker(self.start_ppn + offset))
    }

    pub(super) fn dealloc(&mut self, frame: &FrameTracker) {
        self.allocator.dealloc(frame.page_number() - self.start_ppn);
    }
}

