use crate::syscall;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};
use lib_redos::MutexID;

pub struct MutexGuard<'a, T> {
    mutex_id: &'a MutexID,
    locked: &'a AtomicBool,
    is_heavy: &'a AtomicBool,
    data: &'a mut T,
}

impl<'a, T> core::ops::Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, T> core::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.locked.store(false, Ordering::Release);
        if self.is_heavy.load(Ordering::Acquire) {
            syscall(
                lib_redos::SYS_MUTEX_UNLOCK,
                self.mutex_id as *const MutexID as usize,
                0,
                0,
                0,
            );
        }
    }
}

/// FIXME: 互斥锁不起作用
pub struct Mutex<T> {
    mutex_id: MutexID,
    locked: AtomicBool,
    is_heavy: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Mutex<T> {
        let mut mu = Mutex {
            mutex_id: 0,
            data: UnsafeCell::new(data),
            locked: AtomicBool::new(false),
            is_heavy: AtomicBool::new(false),
        };
        syscall(
            lib_redos::SYS_MUTEX_CREATE,
            &mut mu.mutex_id as *mut _ as usize,
            0,
            0,
            0,
        );
        mu
    }

    pub fn lock(&self) -> MutexGuard<T> {
        if !self.is_heavy.load(Ordering::Acquire) {
            for _ in 0..1000 {
                if self
                    .locked
                    .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
                {
                    let mu = MutexGuard {
                        mutex_id: &self.mutex_id,
                        locked: &self.locked,
                        is_heavy: &self.is_heavy,
                        data: unsafe { self.data.get().as_mut().unwrap() },
                    };
                    return mu;
                }
            }
            self.is_heavy.store(true, Ordering::Release);
        }

        syscall(
            lib_redos::SYS_MUTEX_LOCK,
            &self.mutex_id as *const _ as usize,
            0,
            0,
            0,
        );

        let mu = MutexGuard {
            mutex_id: &self.mutex_id,
            locked: &self.locked,
            is_heavy: &self.is_heavy,
            data: unsafe { self.data.get().as_mut().unwrap() },
        };
        return mu;
    }
}

impl<T> Drop for Mutex<T> {
    fn drop(&mut self) {
        let res = syscall(
            lib_redos::SYS_MUTEX_DESTROY,
            &self.mutex_id as *const _ as usize,
            0,
            0,
            0,
        );
        debug_assert_eq!(res, 0);
    }
}
