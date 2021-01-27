mod stacked_allocator;
mod bitmap_vector_allocator;

pub trait Allocator {
    fn new(capacity: usize) -> Self;
    fn alloc(&mut self) -> Option<usize>;
    fn dealloc(&mut self, index: usize);
}

pub trait VectorAllocator {
    fn new(capacity: usize) -> Self;
    fn alloc(&mut self, size: usize, align: usize) -> Option<usize>;
    fn dealloc(&mut self, start: usize, size: usize, align: usize);
}

use stacked_allocator::StackedAllocator;
use bitmap_vector_allocator::BitmapVectorAllocator;

pub type AllocatorImpl = StackedAllocator;
pub type VectorAllocatorImpl = BitmapVectorAllocator;

