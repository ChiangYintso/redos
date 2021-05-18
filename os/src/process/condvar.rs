//! 条件变量

extern crate alloc;

use crate::kernel::*;
use crate::process::thread::Thread;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::option::Option::Some;
use spin::Mutex;

#[derive(Default)]
pub struct Condvar {
    /// 所有等待此条件变量的线程
    watchers: Mutex<VecDeque<Arc<Thread>>>,
}

impl Condvar {
    /// 令当前线程休眠，等待此条件变量
    pub fn wait(&self) {
        let thread = PROCESSOR.lock().sleep_current_thread();
        self.watchers.lock().push_back(thread);
    }

    /// 唤起一个等待此条件变量的线程
    pub fn notify_one(&self) {
        if let Some(thread) = self.watchers.lock().pop_front() {
            PROCESSOR.lock().wake_thread(thread);
        }
    }

    pub fn notify_all(&self) {
        let mut guard = self.watchers.lock();
        let mut processor_guard = PROCESSOR.lock();
        while let Some(t) = guard.pop_front() {
            processor_guard.wake_thread(t);
        }
    }
}
