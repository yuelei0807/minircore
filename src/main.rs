//use the #![no_std] attribute to tell the compiler not to use the standard rust library but still
//include the C runtime
#![no_std]
//use the #![no_main] attribute to tell the compiler not to use the normal entry point chain
#![no_main]

//mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//overwrite the crt0 entry point directly with the _start function defined by ourselves
//mark the function as extern "C" to tell the compiler to use the C calling convention for the
//function
#[no_mangle]
pub extern "C" fn _start() -> ! {  //the ! return type means the function is diverging, not allowed to ever return
    //vga_buffer::print_something();
    loop {}
}
