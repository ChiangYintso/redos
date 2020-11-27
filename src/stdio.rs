/// 实现类似于标准库中的 `print!` 宏
///
/// 使用实现了 [`core::fmt::Write`] trait 的 [`console::Stdout`]
#[macro_export]
macro_rules! print {
    () => ();
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch::io::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// 实现类似于标准库中的 `println!` 宏
///
/// 使用实现了 [`core::fmt::Write`] trait 的 [`console::Stdout`]
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch::io::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
