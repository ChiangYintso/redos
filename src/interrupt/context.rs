use riscv::register::{sstatus::Sstatus, scause::Scause};

/// 中断时保存了各种寄存器的结构体, 共34个字节
#[repr(C)]
#[derive(Debug)]
pub struct Context {
    pub x: [usize; 32],     // 32 个通用寄存器
    /// Supervisor Status Register
    pub sstatus: Sstatus,
    /// Exception Program Counter, 用来记录触发中断的指令的地址
    pub sepc: usize
}