pub mod sbi;
pub mod console;
pub mod panic;
pub mod interrupt;

use crate::println;

#[no_mangle]
pub extern "C" fn riscv64_main() -> ! {
    let hello = "Hello riscv World!";
    println!("{}", hello);

    #[cfg(feature = "test_riscv64")]
        test_riscv64_main();

    // 初始化各种模块
    interrupt::init();

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    loop {}
    panic!("end of rust_main")
}

#[cfg(feature = "test_riscv64")]
fn test_riscv64_main() {
    println!("test!!");
}
