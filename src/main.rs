#![no_std]
#![no_main]

mod vga_buffer;

use crate::vga_buffer::*;
use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn print_something() {
    WRITER.lock().write_byte(b'H');
    WRITER.lock().write_str("ello ");
    WRITER.lock().write_str("Wörld!\n");
    write!(WRITER.lock(), "1 + 1 = {}\n", 1 + 1).unwrap();
    println!("hello world");
    println!("1 + 1 = {}", 1 + 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_something();
    panic!("Some panic message");
    loop {}
}
