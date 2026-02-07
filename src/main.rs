#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};

mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World! {}", "\nHello from new line!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}