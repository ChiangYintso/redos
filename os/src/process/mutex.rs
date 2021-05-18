use core::sync::atomic::{AtomicIsize, Ordering};

use super::alloc::collections::VecDeque;
use super::alloc::sync::Arc;
use crate::kernel::SyscallResult;
use crate::process::thread::Thread;
use crate::process::thread::ThreadState::Sleeping;
use crate::process::PROCESSOR;
use lib_redos::MutexID;

static NO_OWNER: isize = -123;

/// 通过阻塞其它线程实现的互斥锁
pub struct Mutex {
    queue: VecDeque<Arc<Thread>>,
    owner_thread_id: AtomicIsize,
}

impl Default for Mutex {
    fn default() -> Self {
        Mutex {
            queue: VecDeque::with_capacity(2),
            owner_thread_id: AtomicIsize::new(NO_OWNER),
        }
    }
}

impl Mutex {
    pub fn lock(&mut self) -> SyscallResult {
        let mut processor_guard = PROCESSOR.lock();
        let current_thread = processor_guard.current_thread();

        let res = self.owner_thread_id.compare_exchange(
            NO_OWNER,
            current_thread.id,
            Ordering::Acquire,
            Ordering::Relaxed,
        );
        match res {
            Ok(old) => {
                debug_assert_eq!(old, NO_OWNER);
                SyscallResult::Proceed(0)
            }
            Err(_) => {
                self.queue.push_back(current_thread);
                processor_guard.sleep_current_thread();
                SyscallResult::Park(0)
            }
        }
    }

    pub fn unlock(&mut self) -> SyscallResult {
        self.owner_thread_id.store(NO_OWNER, Ordering::Release);
        let mut processor_guard = PROCESSOR.lock();
        while let Some(t) = self.queue.pop_front() {
            if t.inner().state == Sleeping {
                processor_guard.wake_thread(t);
                break;
            }
        }
        SyscallResult::Park(0)
    }
}

pub(crate) fn sys_mutex_create(mutex_id: *mut MutexID) -> SyscallResult {
    match unsafe { mutex_id.as_mut() } {
        Some(m) => {
            let current_thread = PROCESSOR.lock().current_thread();
            let mid: MutexID = current_thread.process.create_mutex();

            *m = mid;
            SyscallResult::Proceed(0)
        }
        None => SyscallResult::Proceed(-1),
    }
}

pub(crate) fn sys_mutex_lock(mutex_id: *const MutexID) -> SyscallResult {
    if let Some(m) = unsafe { mutex_id.as_ref() } {
        let current_thread = PROCESSOR.lock().current_thread();
        let mut guard = current_thread.process.inner();
        if let Some(mu) = guard.mutex_queue.get_mut(m) {
            return mu.lock();
        }
    }
    SyscallResult::Proceed(-1)
}

pub(crate) fn sys_mutex_unlock(mutex_id: *const MutexID) -> SyscallResult {
    if let Some(m) = unsafe { mutex_id.as_ref() } {
        let current_thread = PROCESSOR.lock().current_thread();
        let mut guard = current_thread.process.inner();
        if let Some(mu) = guard.mutex_queue.get_mut(m) {
            return mu.unlock();
        }
    }
    SyscallResult::Proceed(-1)
}

pub(crate) fn sys_mutex_destroy(mutex_id: *const MutexID) -> SyscallResult {
    if let Some(m) = unsafe { mutex_id.as_ref() } {
        let current_thread = PROCESSOR.lock().current_thread();
        let mut guard = current_thread.process.inner();
        if guard.mutex_queue.remove(m).is_some() {
            return SyscallResult::Proceed(0);
        }
    }
    SyscallResult::Proceed(-1)
}
