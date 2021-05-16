//! 系统调用

use lib_redos;

pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;

/// 将参数放在对应寄存器中，并执行 `ecall`
pub(crate) fn syscall(id: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    // 返回值
    let mut ret = 0;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (id)
            : "memory"      // 如果汇编可能改变内存，则需要加入 memory 选项
            : "volatile"); // 防止编译器做激进的优化（如调换指令顺序等破坏 SBI 调用行为的优化）
    }
    ret
}

/// 读取字符
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    loop {
        let ret = syscall(
            lib_redos::SYS_READ,
            fd,
            buffer as *const [u8] as *const u8 as usize,
            buffer.len(),
        );
        if ret > 0 {
            return ret;
        }
    }
}

/// 打印字符串
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(
        lib_redos::SYS_WRITE,
        fd,
        buffer as *const [u8] as *const u8 as usize,
        buffer.len(),
    )
}

/// 退出并返回数值
pub fn sys_exit(code: isize) -> ! {
    syscall(lib_redos::SYS_EXIT, code as usize, 0, 0);
    unreachable!()
}

fn sys_exit0() -> ! {
    syscall(lib_redos::SYS_EXIT, 0, 0, 0);
    unreachable!()
}
