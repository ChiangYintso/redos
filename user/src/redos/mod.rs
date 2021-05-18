use core::ffi::c_void;
use lib_redos::ThreadID;

pub mod mutex;
pub mod syscall;

pub fn create_thread(thread_id: &mut ThreadID, f: fn(*const c_void), args: *const c_void) -> isize {
    crate::syscall(
        lib_redos::SYS_CREATE_THREAD,
        thread_id as *mut ThreadID as usize,
        f as *const c_void as usize,
        crate::sys_exit as usize,
        args as usize,
    )
}

pub fn sleep(sec: u64) {
    crate::syscall(lib_redos::SYS_SLEEP, sec as usize, 0, 0, 0);
}
