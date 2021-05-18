#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::sync::Arc;
use core::ffi::c_void;
use lazy_static::*;
use user_lib::redos::create_thread;
use user_lib::redos::mutex::*;

lazy_static! {
    static ref DATA: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}
static ID: isize = 2;
static ID3: isize = 3;
#[no_mangle]
pub fn main() -> usize {
    println!("mutex_lock!");

    let mut id = 0;
    create_thread(&mut id, thread_fn, &ID3 as *const _ as *const c_void);
    thread_fn(&ID as *const _ as *const c_void);

    let d = DATA.lock();
    let c: &i32 = &d;
    println!("A: {}", *c);
    0
}

fn thread_fn(id: *const c_void) {
    let id = unsafe { (id as *const isize).as_ref().unwrap() };
    for _ in 0..100000 {
        let mut guard = DATA.lock();
        let before = *guard;
        *guard += 1;
        for i in 0..10 {
            let after = *guard;
            if after != before + 1 {
                println!("{} before: {}, after: {}", i, before, after);
            }
        }

        drop(guard);
    }
    let guard = DATA.lock();
    println!("done! A: {}", *guard);
}
