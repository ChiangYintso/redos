use core::fmt;
use crate::arch::vga::_print;
use core::fmt::Write;

#[doc(hidden)]
pub fn print(args: fmt::Arguments) {
    _print(args);
}
