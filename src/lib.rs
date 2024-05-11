#![no_main]
#![no_std]

pub mod vga_buffer;

use core::arch::asm;
use core::panic::PanicInfo;

static HELLO: &str = "Hello World!";

#[no_mangle]
pub extern "C" fn kernel_start() -> ! {
    let mut writer = vga_buffer::Writer::new();
    writer.write_line(HELLO);

    unsafe { asm!("hlt"); }
    panic!();
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}