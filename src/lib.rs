//! # 全局属性
//! - `#![no_std]`
//!   禁用标准库
#![no_std]
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
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(unchecked_math)]
#![feature(core_intrinsics)]

#[macro_use]
pub mod console;
pub mod arena;
pub mod interrupt;
mod kernel_heap;
pub mod memory;
mod panic;
pub mod sbi;

type KResult<T> = Result<T, &'static str>;

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("entry.asm"));
