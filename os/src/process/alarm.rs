use core::cmp::Ordering;

use lazy_static::*;

use crate::kernel::SyscallResult;
use crate::process::lock::Lock;
use crate::process::thread::Thread;
use crate::process::thread::ThreadState::Dead;
use crate::process::PROCESSOR;

use super::alloc::collections::BinaryHeap;
use super::alloc::sync::Arc;

lazy_static! {
    /// 全局的 [`AlarmClock`]
    pub static ref ALARM: Lock<AlarmClock> = Lock::default();
}

struct ThreadWithAlarmTime {
    thread: Arc<Thread>,
    alarm_time: u64,
}

impl ThreadWithAlarmTime {
    fn new(thread: Arc<Thread>, alarm_time: u64) -> ThreadWithAlarmTime {
        ThreadWithAlarmTime { thread, alarm_time }
    }
}

impl PartialEq for ThreadWithAlarmTime {
    fn eq(&self, other: &Self) -> bool {
        self.alarm_time == other.alarm_time
    }
}

impl Eq for ThreadWithAlarmTime {}

impl PartialOrd for ThreadWithAlarmTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.alarm_time.partial_cmp(&self.alarm_time)
    }
}

impl Ord for ThreadWithAlarmTime {
    fn cmp(&self, other: &Self) -> Ordering {
        other.alarm_time.cmp(&self.alarm_time)
    }
}

#[derive(Default)]
pub struct AlarmClock {
    clock: u64,
    /// 在某一时刻需要唤醒的线程
    alarm_threads: BinaryHeap<ThreadWithAlarmTime>,
}

impl AlarmClock {
    pub fn put_current_thread_to_alarm_threads(&mut self, sec: u64) {
        let current_thread = PROCESSOR.lock().sleep_current_thread();
        self.alarm_threads
            .push(ThreadWithAlarmTime::new(current_thread, self.clock + sec))
    }

    /// 每隔1秒查看`alarm_threads`中有没有需要唤醒的线程
    pub fn alarm(&mut self) {
        self.clock += 1;
        while let Some(thread) = self.alarm_threads.peek() {
            if thread.alarm_time <= self.clock {
                if thread.thread.inner().state != Dead {
                    let t = self.alarm_threads.pop().unwrap();
                    PROCESSOR.lock().wake_thread(t.thread);
                }
            } else {
                return;
            }
        }
    }
}

/// `context`: 当前线程的上下文
pub(crate) fn sys_sleep(sec: u64) -> SyscallResult {
    ALARM.lock().put_current_thread_to_alarm_threads(sec);
    SyscallResult::Park(0)
}
