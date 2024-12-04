#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;
use axstd::println_color;
use axstd::print_color;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    // println!("[WithColor]: Hello, Arceos!");
    println_color!("[WithColor]: Hello, Arceos!");
    print_color!("Hello, Arceos!");
}
