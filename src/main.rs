#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};

mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().write_string("Hello world.\n");
    write!(vga_buffer::WRITER.lock(), "Hello again.").unwrap();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}