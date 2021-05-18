#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::ffi::c_void;
use user_lib::redos::create_thread;

static mut A: i32 = 0;
static S: i32 = 999;

#[no_mangle]
pub fn main() -> usize {
    println!("create_thread_test!");
    let mut id = 0;
    create_thread(&mut id, thread_fn, &S as *const i32 as *const c_void);
    println!("create thread: {}", id);
    thread_fn(&S as *const i32 as *const c_void);
    unsafe {
        println!("A: {}", A);
    }
    0
}

fn thread_fn(a: *const c_void) {
    println!("{:?}", a);
    let b = a as *const i32;
    println!("{}", unsafe { *b });
    for _ in 0..1000000 {
        unsafe {
            A += 1;
        }
    }
    println!("done!");
}
