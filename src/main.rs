#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(minircore::test_runner)]
#![reexport_test_harness_main = "test_main"]
use bootloader::{BootInfo, entry_point};
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


//define the real lower level _start entry point: kernel_main Rust function, using macro
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    //use minircore::memory::active_level_4_table;
    use minircore::memory;
    use x86_64::VirtAddr;
    println!("This is a minircore!");

    minircore::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    
    let mut _mapper = unsafe { memory::init(phys_mem_offset) };
    //let mut frame_allocator = memory::EmptyFrameAllocator;
    let mut _frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    //let page = Page::containing_address(VirtAddr::new(0));
    //memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    //let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    //unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    #[cfg(test)]
    test_main();
    println!("It did not crash!");
    //use the hlt_loop instead of the endless loop
    minircore::hlt_loop();
}

 


//let addresses = [
        //the identity-mapped vga buffer page
        //0xb8000,
        //some code page
        //0x201008,
        //some stack page
        //0x0100_0020_1a10,
        //virtual address mapped to physical address 0
        //boot_info.physical_memory_offset,
    //];

    //for &address in &addresses {
        //let virt = VirtAddr::new(address);
        //let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        //let phys = mapper.translate_addr(virt);
        //println!("{:?} -> {:?}", virt, phys);
    //let level4_table = unsafe { active_level_4_table(phys_mem_offset) };
    //for (i, entry) in level4_table.iter().enumerate() {
    //    if !entry.is_unused() {
    //        println!("The entries of the level 4 table {}: {:?}", i, entry);
    //        // get the physical address from the entry and convert it
    //        let phys = entry.frame().unwrap().start_address();
    //        let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //        let ptr = VirtAddr::new(virt).as_mut_ptr();
    //        let level3_table: &PageTable = unsafe { &*ptr };
    //
            // print non-empty entries of the level 3 table
            //for (i, entry) in level3_table.iter().enumerate() {
                //if !entry.is_unused() {
                    //println!("The entries of the level 3 table {}: {:?}", i, entry);
                //}
            //}
        //}
    //
    //use x86_64::registers::control::Cr3;

    //The Cr3::read function of the x86_64 returns the currently active level 4 page table from the CR3 register
    //let (level_4_page_table, _) = Cr3::read();
    //println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

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
