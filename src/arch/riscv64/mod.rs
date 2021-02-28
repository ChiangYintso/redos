pub mod console;
pub mod interrupt;
pub mod panic;
pub mod sbi;
#[cfg(feature = "test_riscv64")]
mod tests;

use crate::memory;
use crate::println;

#[no_mangle]
pub extern "C" fn riscv64_main() -> ! {
    let hello = "Hello riscv World!";
    println!("{}", hello);

    // 初始化各种模块
    interrupt::init();
    memory::init();

    #[cfg(feature = "test_riscv64")]
    tests::test_riscv64_main();

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    loop {}
    panic!("end of rust_main")
}
