use crate::println;
use crate::tests::alloc_test::{alloc_test, physical_page_memory_test};

#[cfg(feature = "test_riscv64")]
pub(crate) fn test_riscv64_main() -> ! {
    println!("test!!");

    alloc_test();
    physical_page_memory_test();
    panic!("exit");
}
