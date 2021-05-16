use lib_redos::ThreadID;

pub mod syscall;

pub fn create_thread(thread_id: &mut ThreadID, func: fn()) -> isize {
    crate::syscall(
        lib_redos::SYS_CREATE_THREAD,
        thread_id as *mut ThreadID as usize,
        func as usize,
        crate::sys_exit as usize,
    )
}

pub fn sleep(sec: u64) {
    crate::syscall(lib_redos::SYS_SLEEP, sec as usize, 0, 0);
}