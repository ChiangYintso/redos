#[cfg(target_arch = "x86_64")]
pub fn put_acsii_char(ch: u8) {
    static VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
    unsafe {

    }
}

#[cfg(target_arch = "riscv64imac")]
pub fn put_ascii_char(ch: u8) {

}