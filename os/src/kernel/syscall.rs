//! 实现各种系统调用

use super::*;
use crate::interrupt::context::Context;
use crate::kernel::mutex::{sys_mutex_create, sys_mutex_destroy, sys_mutex_lock};
use crate::process::alarm::sys_sleep;
use crate::process::mutex::sys_mutex_unlock;
use crate::process::thread::{Thread, ThreadID};
use core::ffi::c_void;
use lib_redos::MutexID;

/// 系统调用在内核之内的返回值
pub enum SyscallResult {
    /// 继续执行，带返回值
    Proceed(isize),
    /// 记录返回值，但暂存当前线程
    Park(isize),
    /// 丢弃当前 context，调度下一个线程继续执行
    Kill,
}

/// 系统调用的总入口
pub fn syscall_handler(context: &mut Context) -> *mut Context {
    // 无论如何处理，一定会跳过当前的 ecall 指令
    context.sepc += 4;

    let syscall_id = context.x[17];
    let args = [context.x[10], context.x[11], context.x[12], context.x[13]];

    let result = match syscall_id {
        lib_redos::SYS_SLEEP => sys_sleep(args[0] as u64),
        lib_redos::SYS_MUTEX_CREATE => sys_mutex_create(args[0] as *mut MutexID),
        lib_redos::SYS_MUTEX_DESTROY => sys_mutex_destroy(args[0] as *mut MutexID),
        lib_redos::SYS_MUTEX_LOCK => sys_mutex_lock(args[0] as *mut MutexID),
        lib_redos::SYS_MUTEX_UNLOCK => sys_mutex_unlock(args[0] as *mut MutexID),
        lib_redos::SYS_CREATE_THREAD => sys_create_thread(
            args[0] as *mut ThreadID,
            args[1],
            args[2],
            args[3] as *const c_void,
        ),
        lib_redos::SYS_READ => sys_read(args[0], args[1] as *mut u8, args[2]),
        lib_redos::SYS_WRITE => sys_write(args[0], args[1] as *mut u8, args[2]),
        lib_redos::SYS_EXIT => sys_exit(args[0]),
        _ => {
            println!("unimplemented syscall: {}", syscall_id);
            SyscallResult::Kill
        }
    };

    match result {
        SyscallResult::Proceed(ret) => {
            // 将返回值放入 context 中
            context.x[10] = ret as usize;
            context
        }
        SyscallResult::Park(ret) => {
            // 将返回值放入 context 中
            context.x[10] = ret as usize;
            // 保存 context，准备下一个线程
            PROCESSOR.lock().park_current_thread(context);
            PROCESSOR.lock().prepare_next_thread()
        }
        SyscallResult::Kill => {
            // 终止，跳转到 PROCESSOR 调度的下一个线程
            PROCESSOR.lock().kill_current_thread();
            PROCESSOR.lock().prepare_next_thread()
        }
    }
}

/// 用户进程创建线程
fn sys_create_thread(
    thread_id: *mut ThreadID,
    entry_point: usize,
    exit_fn: usize,
    args: *const c_void,
) -> SyscallResult {
    match Thread::spawn(entry_point, exit_fn, args) {
        Ok(nt) => {
            unsafe {
                *thread_id = nt.id;
            }
            PROCESSOR.lock().add_thread(nt);
            SyscallResult::Proceed(0)
        }
        Err(e) => {
            println!("error in sys_create_thread: {}", e);
            SyscallResult::Proceed(-1)
        }
    }
}
