//! 线程 [`Thread`]

use super::*;
use crate::fs::{INodeExt, ROOT_INODE};
use crate::interrupt::context::Context;
use crate::kernel::SyscallResult;
use crate::kernel::SyscallResult::{Park, Proceed};
use crate::memory::addr::VirtualAddress;
use crate::memory::mapping::Flags;
use crate::memory::range::Range;
use crate::process::condvar::Condvar;
use crate::process::kernel_stack::KERNEL_STACK;
use crate::process::process::Process;
use crate::process::thread::ThreadState::{Dead, Runnable};
use crate::KResult;
use alloc::sync::Arc;
use core::ffi::c_void;
use core::hash::{Hash, Hasher};
use spin::Mutex;
use xmas_elf::ElfFile;

/// 线程 ID 使用 `isize`，可以用负数表示错误
pub type ThreadID = isize;

/// 线程计数，用于设置线程 ID
static mut THREAD_COUNTER: ThreadID = 0;

/// 线程的信息
pub struct Thread {
    /// 线程 ID
    pub id: ThreadID,
    /// 线程的栈
    pub stack: Range<VirtualAddress>,
    /// 所属的进程
    pub process: Arc<Process>,
    /// 用 `Mutex` 包装一些可变的变量
    pub inner: Mutex<ThreadInner>,

    pub join_handle: Condvar,
}

#[derive(Eq, PartialEq)]
pub enum ThreadState {
    Runnable,
    Sleeping,
    Dead,
}

/// 线程中需要可变的部分
pub struct ThreadInner {
    /// 线程执行上下文
    ///
    /// 当且仅当线程被暂停执行时，`context` 为 `Some`
    pub context: Option<Context>,
    pub state: ThreadState,
}

impl Thread {
    /// 准备执行一个线程
    ///
    /// 激活对应进程的页表，并返回其 Context
    pub fn prepare(&self) -> *mut Context {
        // 激活页表
        self.process.inner().memory_set.activate();
        // 取出 Context
        let parked_frame = self.inner().context.take().unwrap();
        // 将 Context 放至内核栈顶
        unsafe { KERNEL_STACK.push_context(parked_frame) }
    }

    /// 发生时钟中断后暂停线程，保存状态
    pub fn park(&self, context: Context) {
        // 检查目前线程内的 context 应当为 None
        assert!(self.inner().context.is_none());
        // 将 Context 保存到线程中
        self.inner().context.replace(context);
    }

    /// 当前进程创建一个新的线程
    pub fn spawn(entry_point: usize, exit_fn: usize, args: *const c_void) -> KResult<Arc<Thread>> {
        let current_thread = PROCESSOR.lock().current_thread();
        let t = Thread::new(
            current_thread.process.clone(),
            entry_point,
            Some(&[args as usize]),
        )?;
        t.as_ref().inner().context.as_mut().unwrap().set_ra(exit_fn);
        Ok(t)
    }

    /// 创建一个线程
    pub fn new(
        process: Arc<Process>,
        entry_point: usize,
        arguments: Option<&[usize]>,
    ) -> KResult<Arc<Thread>> {
        // 让所属进程分配并映射一段空间，作为线程的栈
        let stack = process.alloc_page_range(STACK_SIZE, Flags::READABLE | Flags::WRITABLE)?;

        // 构建线程的 Context
        let context = Context::new(stack.end.into(), entry_point, arguments, process.is_user);

        // 打包成线程
        let thread = Arc::new(Thread {
            id: unsafe {
                THREAD_COUNTER += 1;
                THREAD_COUNTER
            },
            stack,
            process,
            join_handle: Condvar::default(),
            inner: Mutex::new(ThreadInner {
                context: Some(context),
                state: Runnable,
            }),
        });

        {
            let mut process_inner = thread.process.inner.lock();
            process_inner
                .threads
                .insert(thread.id, Arc::downgrade(&thread));
        }
        Ok(thread)
    }

    /// 上锁并获得可变部分的引用
    pub fn inner(&self) -> spin::MutexGuard<ThreadInner> {
        self.inner.lock()
    }

    pub fn wait(&self) {
        self.join_handle.wait();
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        {
            let mut process_inner = self.process.inner.lock();
            process_inner.threads.remove(&self.id);
        }
        self.join_handle.notify_all();
    }
}

/// 通过线程 ID 来判等
impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// 通过线程 ID 来判等
///
/// 在 Rust 中，[`PartialEq`] trait 不要求任意对象 `a` 满足 `a == a`。
/// 将类型标注为 [`Eq`]，会沿用 `PartialEq` 中定义的 `eq()` 方法，
/// 同时声明对于任意对象 `a` 满足 `a == a`。
impl Eq for Thread {}

/// 通过线程 ID 来哈希
impl Hash for Thread {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_isize(self.id);
    }
}

/// 打印线程除了父进程以外的信息
impl core::fmt::Debug for Thread {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("Thread")
            .field("thread_id", &self.id)
            .field("stack", &self.stack)
            .field("context", &self.inner().context)
            .finish()
    }
}

pub fn sys_join(tid: ThreadID) -> SyscallResult {
    let current_thread = PROCESSOR.lock().current_thread();
    let guard = current_thread.process.inner.lock();
    match guard.threads.get(&tid) {
        Some(t) => unsafe {
            (*t.as_ptr()).wait();
            Park(0)
        },
        None => Proceed(-1),
    }
}

/// 内核线程需要调用这个函数来退出
fn kernel_thread_exit() {
    // 当前线程标记为结束
    PROCESSOR.lock().current_thread().as_ref().inner().state = Dead;
    // 制造一个中断来交给操作系统处理
    unsafe { llvm_asm!("ebreak" :::: "volatile") };
}

/// 创建一个内核进程
pub fn create_kernel_thread(
    process: Arc<Process>,
    entry_point: usize,
    arguments: Option<&[usize]>,
) -> Arc<Thread> {
    // 创建线程
    let thread = Thread::new(process, entry_point, arguments).unwrap();
    // 设置线程的返回地址为 kernel_thread_exit
    thread
        .as_ref()
        .inner()
        .context
        .as_mut()
        .unwrap()
        .set_ra(kernel_thread_exit as usize);
    thread
}

/// 创建一个用户进程，从指定的文件名读取 ELF
pub fn create_user_process(name: &str) -> Arc<Thread> {
    // 从文件系统中找到程序
    let app = ROOT_INODE.find(name).unwrap();
    // 读取数据
    let data = app.readall().unwrap();
    // 解析 ELF 文件
    let elf = ElfFile::new(data.as_slice()).unwrap();
    // 利用 ELF 文件创建线程，映射空间并加载数据
    let process = Process::from_elf(&elf, true).unwrap();
    // 再从 ELF 中读出程序入口地址
    Thread::new(process, elf.header.pt2.entry_point() as usize, None).unwrap()
}
