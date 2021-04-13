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
use redos::process::thread::create_kernel_thread;
use redos::process::PROCESSOR;
use redos::{drivers, fs};

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress) -> ! {
    println!("Hello rCore-Tutorial!");
    // 初始化各种模块
    redos::interrupt::init();
    memory::init();
    drivers::init(dtb_pa);

    let remap = memory::mapping::MemorySet::new_kernel().unwrap();
    remap.activate();

    println!("kernel remapped");
    extern "C" {
        fn __restore(context: usize);
    }

    {
        let mut processor = PROCESSOR.lock();
        // 创建一个内核进程
        let kernel_process = Process::new_kernel().unwrap();
        // 为这个进程创建多个线程，并设置入口均为 sample_process，而参数不同
        for i in 1..12usize {
            processor.add_thread(create_kernel_thread(
                kernel_process.clone(),
                sample_process as usize,
                Some(&[i]),
            ));
        }
    }

    // 获取第一个线程的 Context，具体原理后面讲解
    let context = PROCESSOR.lock().prepare_next_thread();
    // 启动第一个线程
    unsafe { __restore(context as usize) };
    unreachable!();
}

fn sample_process(id: usize) {
    println!("hello from kernel thread {}", id);
}
