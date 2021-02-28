use crate::println;
use core::panic::PanicInfo;

/// 这个函数将在 panic 时被调用
#[cfg(target_arch = "x86_64")]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("panic: {}", info.message().unwrap());
    loop {}
}
