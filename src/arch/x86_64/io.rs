use core::fmt;
use crate::arch::vga::Stdout;
use core::fmt::Write;

#[doc(hidden)]
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}
