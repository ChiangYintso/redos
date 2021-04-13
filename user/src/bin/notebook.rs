#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::console::*;

#[no_mangle]
pub fn main() -> usize {
    println!("\x1b[2J<notebook>");
    loop {
        let string = getchars();
        println!("{} len: {}", string, string.len());
        if string == "exit\n" {
            println!("bye");
            return 0;
        }
    }
}
