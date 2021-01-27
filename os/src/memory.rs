pub mod config;
mod heap;
mod address;
pub mod frame;
mod range;

pub use range::Range;
pub use frame::allocator::FRAME_ALLOCATOR;

pub type MemoryResult<T> = Result<T, &'static str>;

pub fn init() {
    heap::init();

    println!("mod memory initialized");
}
