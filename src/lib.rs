#![no_main]
#![no_std]

pub mod vga_buffer;

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_start() -> ! {
    println!("Hello World!");

    unsafe { asm!("hlt"); }
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    println!("Kernel panic: {}", info);
    unsafe { asm!("hlt"); }
    loop {}
}