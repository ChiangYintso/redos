//! 进程 [`Process`]

extern crate alloc;

use crate::fs::stdin::STDIN;
use crate::fs::INode;
use crate::fs::STDOUT;
use crate::kernel::thread::ThreadID;
use crate::memory::addr::VirtualAddress;
use crate::memory::mapping::{Flags, MapType, MemorySet, Segment};
use crate::memory::range::Range;
use crate::memory::PAGE_SIZE;
use crate::process::thread::Thread;
use crate::KResult;
use alloc::sync::Weak;
use alloc::{vec, vec::Vec};
use hashbrown::HashMap;
use lib_redos::MutexID;
use spin::Mutex;
use xmas_elf::ElfFile;

use super::alloc::sync::Arc;

/// 进程的信息
pub struct Process {
    /// 是否属于用户态
    pub is_user: bool,
    /// 用 `Mutex` 包装一些可变的变量
    pub inner: Mutex<ProcessInner>,
}

pub struct ProcessInner {
    /// 进程中的线程公用页表 / 内存映射
    pub memory_set: MemorySet,
    /// 打开的文件描述符
    pub descriptors: Vec<Arc<dyn INode>>,
    pub threads: HashMap<ThreadID, Weak<Thread>>,
    pub mutex_queue: HashMap<MutexID, super::mutex::Mutex>,
    next_mutex_id: MutexID,
}

#[allow(unused)]
impl Process {
    /// 创建一个内核进程
    pub fn new_kernel() -> KResult<Arc<Self>> {
        Ok(Arc::new(Self {
            is_user: false,
            inner: Mutex::new(ProcessInner {
                memory_set: MemorySet::new_kernel()?,
                descriptors: vec![STDIN.clone(), STDOUT.clone()],
                threads: HashMap::default(),
                mutex_queue: HashMap::default(),
                next_mutex_id: 0,
            }),
        }))
    }

    /// 创建进程，从文件中读取代码
    pub fn from_elf(file: &ElfFile, is_user: bool) -> KResult<Arc<Self>> {
        Ok(Arc::new(Process {
            is_user,
            inner: Mutex::new(ProcessInner {
                memory_set: MemorySet::from_elf(file, is_user)?,
                descriptors: vec![STDIN.clone(), STDOUT.clone()],
                threads: HashMap::default(),
                mutex_queue: HashMap::default(),
                next_mutex_id: 0,
            }),
        }))
    }

    /// 上锁并获得可变部分的引用
    pub fn inner(&self) -> spin::MutexGuard<ProcessInner> {
        self.inner.lock()
    }

    /// 分配一定数量的连续虚拟空间
    ///
    /// 从 `memory_set` 中找到一段给定长度的未占用虚拟地址空间，分配物理页面并建立映射。返回对应的页面区间。
    ///
    /// `flags` 只需包括 rwx 权限，user 位会根据进程而定。
    pub fn alloc_page_range(&self, size: usize, flags: Flags) -> KResult<Range<VirtualAddress>> {
        let memory_set = &mut self.inner().memory_set;

        // memory_set 只能按页分配，所以让 size 向上取整页
        let alloc_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        // 从 memory_set 中找一段不会发生重叠的空间
        let mut range = Range::<VirtualAddress>::from(0x1000000..0x1000000 + alloc_size);
        while memory_set.overlap_with(range.into()) {
            range.start += alloc_size;
            range.end += alloc_size;
        }
        // 分配物理页面，建立映射
        memory_set.add_segment(
            Segment {
                map_type: MapType::Framed,
                range,
                flags: flags | Flags::user(self.is_user),
            },
            None,
        )?;
        // 返回地址区间（使用参数 size，而非向上取整的 alloc_size）
        Ok(Range::from(range.start..(range.start + size)))
    }

    pub fn create_mutex(&self) -> MutexID {
        let mut guard = self.inner.lock();
        let id: MutexID = guard.next_mutex_id;
        guard.mutex_queue.insert(id, super::mutex::Mutex::default());
        guard.next_mutex_id += 1;
        id
    }
}
