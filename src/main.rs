#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use minircore::println;
use core::panic::PanicInfo;
pub trait Testable {
    fn run(&self);
}
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    minircore::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("This is a minircore!");

    minircore::init();

    // for each recursion, the return address is pushed
    //fn stack_overflow() {
    //    stack_overflow(); 
    //}

    // trigger a stack overflow
    //stack_overflow();

    // trigger a page fault
    //unsafe {
        //*(0xdeadbeef as *mut u64) = 42;
    //};
    //invoke a breakpoint exception
    //x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    #[allow(clippy::empty_loop)]
    loop {}
}


//overwrite the crt0 entry point directly with the _start function defined by ourselves
//mark the function as extern "C" to tell the compiler to use the C calling convention for the
//function
//pub extern "C" fn _start() -> ! {  //the ! return type means the function is diverging, not allowed to ever return
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
