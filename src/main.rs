#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(redos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use crate::vga_buffer::*;
use core::fmt::Write;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    redos::test_panic_handler(info)
}

fn print_something() {
    WRITER.lock().write_byte(b'H');
    WRITER.lock().write_str("ello ");
    WRITER.lock().write_str("Wörld!\n");
    write!(WRITER.lock(), "1 + 1 = {}\n", 1 + 1).unwrap();
    println!("hello world");
    println!("1 + 2 = {}", 1 + 2);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    print_something();
    panic!("Some panic message");
    loop {}
}
