use super::address::PhysicalAddress;

pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;
pub const PAGE_SIZE: usize = 0x1000;

pub const MEMORY_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x8000_0000);
pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000);

lazy_static! {
    pub static ref KERNEL_END_ADDRESS: PhysicalAddress = PhysicalAddress(kernel_end as usize);
}

extern "C" {
    fn kernel_end();
}

