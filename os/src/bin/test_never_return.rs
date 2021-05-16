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

#[macro_use]
extern crate redos;

use redos::memory;
use redos::memory::addr::PhysicalAddress;
use redos::process::process::Process;
use redos::process::thread::{create_kernel_thread, create_user_process};
use redos::process::PROCESSOR;
use redos::{drivers, fs, interrupt};

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress) -> ! {
    memory::init();
    interrupt::init();
    drivers::init(dtb_pa);
    fs::init();

    {
        let mut processor = PROCESSOR.lock();
        processor.add_thread(create_user_process("never_return"));
    }

    extern "C" {
        fn __restore(context: usize);
    }
    // 获取第一个线程的 Context
    let context = PROCESSOR.lock().prepare_next_thread();
    // 启动第一个线程
    unsafe { __restore(context as usize) };
    unreachable!()
}
