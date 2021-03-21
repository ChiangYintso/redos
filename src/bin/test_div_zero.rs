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
#![feature(asm)]

#[macro_use]
extern crate redos;

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() {
    println!("Hello ktest1!");
    // 初始化各种模块
    redos::interrupt::init();
    let x = div();
    println!("{}", x);
}

fn div() -> u32 {
    let mut x: u32;
    let mut y: u32;
    let z: u32;
    unsafe {
        asm!("li {}, 50", out(reg) x);
        asm!("li {}, 0", out(reg) y);
        asm!("divu {}, {}, {}", out(reg) z, inout(reg) x, inout(reg) y);
    }
    z
}
