#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(global_asm)]
#![feature(llvm_asm)]

#[cfg(target_arch = "riscv64")]
#[path = "arch/riscv64/mod.rs"]
#[macro_use]
pub mod arch;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
#[macro_use]
pub mod arch;

#[macro_use]
pub mod stdio;

#[cfg(target_arch = "riscv64")]
global_asm!(include_str!("riscv64_entry.asm"));

use core::fmt;
use core::fmt::Write;
use core::panic::PanicInfo;

/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let hello = "Hello x86_64 World!";
    print!("{}", hello);
    loop {}
}

#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn riscv64_main() -> ! {
    let hello = "Hello riscv World!";
    print!("hello");
    loop {}
}
