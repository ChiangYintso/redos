//! The kernel allocates memory in arena and never deallocates.

/// 1M
pub const ARENA_SIZE: usize = 0x10_0000;

pub static mut ARENA: [u8; ARENA_SIZE] = [0; ARENA_SIZE];

pub static mut ARENA_IDX: usize = 0;

pub fn arena_alloc(n: usize) -> *mut u8 {
    unsafe {
        let res = ARENA.as_mut_ptr().add(ARENA_IDX);
        ARENA_IDX += n;
        debug_assert!(ARENA_IDX < ARENA_SIZE, "arena out of memory!");
        res
    }
}
