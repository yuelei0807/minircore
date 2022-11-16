#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"This is a minircore.";

#[no_mangle]
pub extern "C" fn _start() -> ! { 
    let vga_buffer = 0xb8000 as *mut u8;
    //0xb8000 is the address of the VGA text buffer

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            //call the offset method to write the string byte
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
            //0xb is the corresponding color byte
        }
    }
    loop {}
}

