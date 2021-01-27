use super::VectorAllocator;
use bit_field::BitArray;
use core::cmp::min;

const BITMAP_SIZE: usize = 4096;

struct BitmapVectorAllocator {
    capacity: usize,
    bitmap: [u8; usize],
}

impl VectorAllocator for BitmapVectorAllocator {
    fn new(capacity: usize) {
        Self {
            capacity: min(BITMAP_SIZE, capacity),
            bitmap: [0u8; BITMAP_SIZE / 8],
        }
    }

    fn alloc(&mut self, size: usize, align: usize) -> Option<usize> {
        for start in (0..self.capacity - size).step_by(align) {
            if (start..start + size).all(|i| !self.bitmap.get_bit(i)) {
                (start..start + size).for_each(|i| self.bitmap.set_bit(i, true));
                return Some(start);
            }
        }
        None
    }

    fn dealloc(&mut self, start: usize, size: usize, align: usize) {
        assert!(self.bitmap.get_bit(start));
        (start..start + size).for_each(|i| self.bitmap.set_bit(i, false));
    }
}
