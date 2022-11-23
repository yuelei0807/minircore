#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(minircore::test_runner)]
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
    minircore::hlt_loop();
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

    use x86_64::registers::control::Cr3;

    //The Cr3::read function of the x86_64 returns the currently active level 4 page table from the CR3 register
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

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

    //access some memory outside the kernel, the CR2 register contains the address 0xdeadbeaf that we tried to access
    //let ptr = 0x204de7 as *mut u32;

    //read from a code page
    //unsafe { 
    //    let x = *ptr; 
    //}
    //println!("read worked!");
    
    //write to a code page
    //unsafe {
        //*ptr = 42
    //}
    //println!("write worked!");
    
    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    //use the hlt_loop instead of the endless loop
    minircore::hlt_loop();
    //#[allow(clippy::empty_loop)]
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
