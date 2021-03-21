use super::KERNEL_END_ADDRESS;
use crate::arena::arena_alloc;
use crate::memory::addr::{PhysicalAddress, PhysicalPageNumber};
use crate::memory::MEMORY_END_ADDRESS;
use crate::KResult;
use core::intrinsics::size_of;

/// Next fit allocator using bit vector
pub struct FrameAllocator {
    start: PhysicalPageNumber,
    frame_total: usize,
    used: usize,
    bit_vector: *const u8,
    /// pointer to the next available byte
    next_byte: usize,
}

impl Default for FrameAllocator {
    fn default() -> FrameAllocator {
        let start = PhysicalPageNumber::ceil(*KERNEL_END_ADDRESS);
        let length = PhysicalPageNumber::floor(MEMORY_END_ADDRESS) - start;
        assert_ne!(length, 0);
        println!("init frame allocator");
        println!("start: {}; frame_total: {}", start, length);
        let bit_vector = arena_alloc((length + 1) / 8);
        unsafe {
            core::ptr::write_bytes(bit_vector, 0xff, length);
        }

        FrameAllocator {
            start,
            frame_total: length,
            used: 0,
            bit_vector,
            next_byte: 0,
        }
    }
}

impl FrameAllocator {
    pub fn alloc(&mut self) -> KResult<PhysicalPageNumber> {
        if self.used == self.frame_total {
            return Err("no frame available");
        }

        unsafe {
            let mut bt = self.bit_vector.add(self.next_byte) as *mut u8;
            let flags = *bt;
            let place = flags & ((!flags).unchecked_add(1));
            *bt = flags ^ place;
            self.used += 1;
            if self.used < self.frame_total {
                while *bt == 0 {
                    self.next_byte = (self.next_byte + 1) % self.frame_total;
                    bt = self.bit_vector.add(self.next_byte) as *mut u8;
                }
            }

            Ok(self.start + (self.next_byte * 8 + (place.trailing_zeros() + 1) as usize))
        }
    }

    pub fn dealloc(&mut self, ppn: PhysicalPageNumber) -> KResult<()> {
        todo!();
        Ok(())
    }

    #[inline]
    pub fn frame_total(&self) -> usize {
        self.frame_total
    }
}

/// Bitmap implemented on tree-like array.
struct TreeBitMap {
    bit_vector: *const usize,
    length: usize,
}

impl TreeBitMap {
    pub fn new(length: usize) -> TreeBitMap {
        unsafe {
            let bit_vector = arena_alloc(length / (size_of::<usize>() * 8) + 1) as *mut usize;
            core::ptr::write_bytes(bit_vector, 0x1, length);
            TreeBitMap { bit_vector, length }
        }
    }

    pub fn find_and_update(&mut self) -> usize {
        0
    }
}
