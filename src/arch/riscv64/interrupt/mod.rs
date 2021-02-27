//! 中断模块
//!
//!

mod context;
mod handler;

use crate::println;

/// 初始化中断相关的子模块
///
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    handler::init();
    println!("mod interrupt initialized");
}
