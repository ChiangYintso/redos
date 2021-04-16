#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::create_thread;

static mut A: i32 = 0;

#[no_mangle]
pub fn main() -> usize {
    println!("create_thread_test!");
    let mut id = 0;
    create_thread(&mut id, thread_fn);
    println!("create thread: {}", id);
    thread_fn();
    unsafe {
        println!("A: {}", A);
    }
    0
}

fn thread_fn() {
    for _ in 0..100 {
        unsafe {
            A += 1;
            println!("{}", A);
        }
    }
    println!("done!");
}
