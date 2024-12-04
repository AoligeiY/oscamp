//! Standard library macros

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// [`println!`]: crate::println
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!($($arg)*));
    }
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!("{}\n", format_args!($($arg)*)));
    }
}

#[macro_export]
macro_rules! print_color {
    ($($arg:tt)*) => {{
        const BLUE: &str = "\x1b[34m"; // 蓝色 ANSI 转义码
        const RESET: &str = "\x1b[0m"; // 重置颜色
        $crate::io::__print_impl(format_args!(
            "{}{}{}",
            BLUE,
            format_args!($($arg)*),
            RESET
        ));
    }};
}

#[macro_export]
macro_rules! println_color {
    () => {
        $crate::print_blue!("\n");
    };
    ($($arg:tt)*) => {{
        const RED: &str = "\x1b[31m"; // 蓝色 ANSI 转义码
        const RESET: &str = "\x1b[0m"; // 重置颜色
        $crate::io::__print_impl(format_args!(
            "{}{}{}\n",
            RED,
            format_args!($($arg)*),
            RESET
        ));
    }};
}