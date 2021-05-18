#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::ffi::c_void;
use lib_redos::ThreadID;
use user_lib::redos::{create_thread, sleep};

#[no_mangle]
pub fn main() -> usize {
    let mut t: ThreadID = 0;
    create_thread(&mut t, thread_fn, core::ptr::null());
    for _ in 0..10 {
        println!("sleep 5 seconds!");
        sleep(5);
    }
    println!("thread1 exit");
    0
}

fn thread_fn(_: *const c_void) {
    for _ in 0..10 {
        println!("sleep 3 seconds!");
        sleep(3);
    }
    println!("thread2 exit");
}
