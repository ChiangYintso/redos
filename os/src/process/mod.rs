pub use processor::PROCESSOR;

pub mod alarm;
pub mod condvar;
mod kernel_stack;
mod lock;
pub mod mutex;
pub mod process;
pub mod processor;
mod scheduler;
pub mod thread;

extern crate alloc;

/// 每个线程的运行栈大小 512 KB
pub const STACK_SIZE: usize = 0x8_0000;

/// 共用的内核栈大小 512 KB
pub const KERNEL_STACK_SIZE: usize = 0x8_0000;
