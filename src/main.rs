#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

use core::panic::PanicInfo;

#[cfg(target_arch = "riscv64imac")]
extern crate riscv64;
#[cfg(target_arch = "x86_64")]
extern crate x86_64;

/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(target_arch = "x86_64")]
    {
        static HELLO: &[u8] = b"Hello x86_64 World!";
        x86_64::print_str(HELLO);
    }

    loop {}
}
