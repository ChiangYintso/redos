#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(global_asm)]
#![feature(llvm_asm)]

#[cfg(target_arch = "riscv64")]
global_asm!(include_str!("riscv64_entry.asm"));

use core::panic::PanicInfo;

#[cfg(target_arch = "riscv64imac")]
extern crate riscv64;
#[cfg(target_arch = "x86_64")]
extern crate x86_64;

extern crate vga;
/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    static HELLO: &[u8] = b"Hello x86_64 World!";
    vga::puts(HELLO);
    loop {}
}

#[cfg(target_arch = "riscv64")]
pub fn console_putchar(ch: u8) {
    let _ret: usize;
    let arg0: usize = ch as usize;
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;
    unsafe {
        llvm_asm!("ecall"
             : "={x10}" (_ret)
             : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
             : "memory"
             : "volatile"
        );
    }

}

#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn riscv64_main() -> ! {
    console_putchar(b'O');
    console_putchar(b'K');
    // static HELLO: &[u8] = b"Hello riscv64 World!";
    // vga::puts(HELLO);
    loop {}
}
