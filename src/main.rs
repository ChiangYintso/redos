//! # 全局属性
//! - `#![no_std]`
//!   禁用标准库
#![no_std]
//!
//! - `#![no_main]`
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口
#![no_main]
//! # 一些 unstable 的功能需要在 crate 层级声明后才可以使用
//! - `#![feature(llvm_asm)]`
//!   内嵌汇编
#![feature(llvm_asm)]
//!
//! - `#![feature(global_asm)]`
//!   内嵌整个汇编文件
#![feature(global_asm)]
//!
//! - `#![feature(panic_info_message)]`
//!   panic! 时，获取其中的信息并打印
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[cfg(target_arch = "riscv64")]
#[path = "arch/riscv64/mod.rs"]
#[macro_use]
pub mod arch;

use arch::*;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
#[macro_use]
pub mod arch;

#[macro_use]
pub mod stdio;
pub mod memory;

#[cfg(target_arch = "riscv64")]
global_asm!(include_str!("riscv64_entry.asm"));


#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel_init();
}

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    #[cfg(target_arch = "x86_64")]
    x64_main();

    #[cfg(target_arch = "riscv64")]
    riscv64_main();

    panic!("unsupported architecture!")
}
