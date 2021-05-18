#![no_std]

/// 线程 ID 使用 `isize`，可以用负数表示错误
pub type ThreadID = isize;
pub type MutexID = usize;

pub const SYS_SLEEP: usize = 3;
pub const SYS_JOIN: usize = 4;

pub const SYS_MUTEX_CREATE: usize = 14;
pub const SYS_MUTEX_DESTROY: usize = 15;
pub const SYS_MUTEX_LOCK: usize = 16;
pub const SYS_MUTEX_UNLOCK: usize = 17;

pub const SYS_CREATE_THREAD: usize = 62;
pub const SYS_READ: usize = 63;
pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;
