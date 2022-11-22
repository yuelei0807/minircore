//use the #![no_std] attribute to tell the compiler not to use the standard rust library but still
//include the C runtime
#![no_std]
//use the #![no_main] attribute to tell the compiler not to use the normal entry point chain
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//static HELLO: &[u8] = b"This is a minircore.";

//overwrite the crt0 entry point directly with the _start function defined by ourselves
//mark the function as extern "C" to tell the compiler to use the C calling convention for the
//function
#[no_mangle]
pub extern "C" fn _start() -> ! {  //the ! return type means the function is diverging, not allowed to ever return
    //vga_buffer::print_something();
    //cast the interger 0xb8000 into a raw pointer *mut, 0xb8000 is the address of the VGA text
    //buffer
    //let vga_buffer = 0xb8000 as *mut u8;

    //iterate over the bytes of the static HELLO byte string, and use the enumerate method to get a
    //running variable i
    //for (i, &byte) in HELLO.iter().enumerate() {
        //the unsafe block tells the compiler that we are sure that the operations are valid
        //unsafe {
            //call the offset method to write the string byte
            //*vga_buffer.offset(i as isize * 2) = byte;
            //call the offset method to write the corresponding color byte, 0xb is light cyan
            //*vga_buffer.offset(i as isize * 2 + 1) = 0xb;
/*
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", To print some numbers: {} {}", 1, 1.1).unwrap();
*/
    println!("This is a minircore{}", "!");
    panic!("Some panic messages.");
    loop {}
}
