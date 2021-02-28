pub mod vga;
pub mod console;
pub mod panic;

use crate::println;

pub fn x64_main() -> ! {
    let hello = "Hello x86_64 World!";
    println!("{}", hello);
    panic!("ohhhhhhhh")
}
