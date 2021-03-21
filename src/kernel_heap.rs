use core::alloc::{GlobalAlloc, Layout};
use core::cmp::max;

/// 操作系统动态分配内存所用的堆大小（8M）
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;

/// 进行动态内存分配所用的堆空间
///
/// 大小为 [`KERNEL_HEAP_SIZE`]
/// 这段空间编译后会被放在操作系统执行程序的 bss 段
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

static mut BIT_ARR: [u8; KERNEL_HEAP_SIZE / 8 * 2] = [0; KERNEL_HEAP_SIZE / 8 * 2];

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new();

/// 堆，动态内存分配器
///
/// ### `#[global_allocator]`
/// 可以为全局需要用到堆的地方分配空间。例如 `Box` `Arc` 等
struct Allocator {
    used: usize,
}

impl Allocator {
    pub const fn new() -> Allocator {
        Allocator { used: 0 }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size().next_power_of_two();
        unimplemented!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unimplemented!()
    }
}
