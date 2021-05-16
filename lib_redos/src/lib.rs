#![no_std]
/// 线程 ID 使用 `isize`，可以用负数表示错误
pub type ThreadID = isize;

pub const SYS_SLEEP: usize = 3;
pub const SYS_CREATE_THREAD: usize = 62;
pub const SYS_READ: usize = 63;
pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;
