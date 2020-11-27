use core::fmt;
use core::fmt::Write;

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let vga_buffer = 0xb8000 as *mut u8;

        for (i, byte) in s.bytes().enumerate() {
            unsafe {
                *vga_buffer.offset(i as isize * 2) = byte;
                *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
            }
        }
        Ok(())
    }
}
