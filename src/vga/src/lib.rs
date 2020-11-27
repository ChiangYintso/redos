#[cfg(target_arch = "x86_64")]
pub fn puts(s: &[u8]) {
    static VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
    unsafe {
        for (i, &byte) in s.iter().enumerate() {
            unsafe {
                *VGA_BUFFER.offset(i as isize * 2) = byte;
                *VGA_BUFFER.offset(i as isize * 2 + 1) = 0xb;
            }
        }
    }
}

#[cfg(target_arch = "riscv64imac")]
pub fn puts(s: &[u8]) {}
