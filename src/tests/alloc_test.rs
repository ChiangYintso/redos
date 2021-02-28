extern crate alloc;
use crate::memory::*;
use crate::memory::frame::allocator::FRAME_ALLOCATOR;

pub fn alloc_test() {
    // 动态内存分配测试
    use alloc::boxed::Box;
    use alloc::vec::Vec;


    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10000);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }
    println!("heap test passed");

    println!("kernel end: {}", *KERNEL_END_ADDRESS);
}

pub fn physical_page_memory_test() {
    println!("test physical page memory");

    // 物理页分配
    for _ in 0..2 {
        let frame_0 = match FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_1 = match FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        println!("{} and {}", frame_0.address(), frame_1.address());
    }
    println!("physical_page_memory_test passed");
}