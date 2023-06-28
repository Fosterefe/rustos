#![no_std] // no standar library
#![no_main] // no entry point

use core::panic::PanicInfo;
mod vga_buffer;

static HELLO: &[u8] = b"Hello world";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
   /* let vga_buffer = 0xb8000 as *mut u8;

    for(i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;

        }
    }*/

    // the linker look for _start
    vga_buffer::print_something;
    vga_buffer::WRITER.lock().write_str("Hello String").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {} ", 42, 1.0/3.0).unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
