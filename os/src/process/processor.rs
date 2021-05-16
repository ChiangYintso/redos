//! 实现线程的调度和管理 [`Processor`]

use super::alloc::collections::binary_heap::BinaryHeap;
use super::alloc::sync::Arc;
use super::lock::Lock;
use super::process::Process;
use super::scheduler::*;
use crate::interrupt::context::Context;
use crate::kernel::syscall::SyscallResult;
use crate::process::thread::Thread;
use core::cmp::Ordering;
use hashbrown::HashSet;
use lazy_static::*;

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

lazy_static! {
    /// 全局的 [`Processor`]
    pub static ref PROCESSOR: Lock<Processor> = Lock::new(Processor::default());
}

lazy_static! {
    /// 空闲线程：当所有线程进入休眠时，切换到这个线程——它什么都不做，只会等待下一次中断
    static ref IDLE_THREAD: Arc<Thread> = Thread::new(
        Process::new_kernel().unwrap(),
        wait_for_interrupt as usize,
        None,
    ).unwrap();
}

/// 不断让 CPU 进入休眠等待下一次中断
unsafe fn wait_for_interrupt() {
    loop {
        llvm_asm!("wfi" :::: "volatile");
    }
}

/// 线程调度和管理
///
/// 休眠线程会从调度器中移除，单独保存。在它们被唤醒之前，不会被调度器安排。
///
/// # 用例
///
/// ### 切换线程（在中断中）
/// ```rust
/// processor.park_current_thread(context);
/// processor.prepare_next_thread()
/// ```
///
/// ### 结束线程（在中断中）
/// ```rust
/// processor.kill_current_thread();
/// processor.prepare_next_thread()
/// ```
///
/// ### 休眠线程（在中断中）
/// ```rust
/// processor.park_current_thread(context);
/// processor.sleep_current_thread();
/// processor.prepare_next_thread()
/// ```
///
/// ### 唤醒线程
/// 线程会根据调度器分配执行，不一定会立即执行。
/// ```rust
/// processor.wake_thread(thread);
/// ```
#[derive(Default)]
pub struct Processor {
    /// 当前正在执行的线程
    current_thread: Option<Arc<Thread>>,
    /// 线程调度器，记录活跃线程
    scheduler: SchedulerImpl<Arc<Thread>>,
    /// 保存休眠线程
    sleeping_threads: HashSet<Arc<Thread>>,
    /// 在某一时刻需要唤醒的线程
    alarm_threads: BinaryHeap<ThreadWithAlarmTime>,
    /// 自系统开机初始化后经历的秒数
    num_seconds: u64,
}

impl Processor {
    /// 获取一个当前线程的 `Arc` 引用
    pub fn current_thread(&self) -> Arc<Thread> {
        self.current_thread
            .as_ref()
            .expect("error in `Processor::current_thread`: no thread")
            .clone()
    }

    /// 激活下一个线程的 `Context`
    pub fn prepare_next_thread(&mut self) -> *mut Context {
        // 向调度器询问下一个线程
        if let Some(next_thread) = self.scheduler.get_next() {
            // 准备下一个线程
            let context = next_thread.prepare();
            self.current_thread = Some(next_thread);
            context
        } else {
            // 没有活跃线程
            if self.sleeping_threads.is_empty() && self.alarm_threads.is_empty() {
                // 也没有休眠线程，则退出
                panic!("all threads terminated, shutting down");
            } else {
                // 有休眠线程，则等待中断
                self.current_thread = Some(IDLE_THREAD.clone());
                IDLE_THREAD.prepare()
            }
        }
    }

    /// 添加一个待执行的线程
    pub fn add_thread(&mut self, thread: Arc<Thread>) {
        self.scheduler.add_thread(thread);
    }

    /// 唤醒一个休眠线程
    pub fn wake_thread(&mut self, thread: Arc<Thread>) {
        thread.inner().sleeping = false;
        self.sleeping_threads.remove(&thread);
        self.scheduler.add_thread(thread);
    }

    /// 保存当前线程的 `Context`
    pub fn park_current_thread(&mut self, context: &Context) {
        self.current_thread().park(*context);
    }

    /// 令当前线程进入休眠
    pub fn sleep_current_thread(&mut self) {
        // 从 current_thread 中取出
        let current_thread = self.current_thread();
        // 记为 sleeping
        current_thread.inner().sleeping = true;
        // 从 scheduler 移出到 sleeping_threads 中
        self.scheduler.remove_thread(&current_thread);
        self.sleeping_threads.insert(current_thread);
    }

    /// 终止当前的线程
    pub fn kill_current_thread(&mut self) {
        // 从调度器中移除
        let thread = self.current_thread.take().unwrap();
        self.scheduler.remove_thread(&thread);
    }

    pub fn put_current_thread_to_alarm_threads(&mut self, sec: u64) {
        // 从 current_thread 中取出
        let current_thread = self.current_thread();
        // 记为 sleeping
        current_thread.inner().sleeping = true;
        // 从 scheduler 移出到 sleeping_threads 中
        self.scheduler.remove_thread(&current_thread);
        self.alarm_threads.push(ThreadWithAlarmTime::new(
            current_thread,
            self.num_seconds + sec,
        ))
    }

    /// 每隔1秒查看`alarm_threads`中有没有需要唤醒的线程
    pub fn alarm(&mut self) {
        self.num_seconds += 1;
        if let Some(thread) = self.alarm_threads.peek() {
            if thread.alarm_time < self.num_seconds {
                thread.thread.inner().sleeping = false;
                let t = self.alarm_threads.pop().unwrap();
                if !t.thread.inner().dead {
                    self.scheduler.add_thread(t.thread);
                }
            }
        }
    }
}

/// `context`: 当前线程的上下文
pub(crate) fn sys_sleep(sec: u64, context: &Context) -> SyscallResult {
    PROCESSOR.lock().park_current_thread(context);
    PROCESSOR.lock().put_current_thread_to_alarm_threads(sec);
    SyscallResult::Sleep
}
