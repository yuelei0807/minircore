use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
};

//the complete physical memory is mapped to virtual memory at  the passed 'physical_memory_offset`. Also, this function must be called once
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable { // return a mutable reference to the active level 4 table
    use x86_64::registers::control::Cr3;

    //read the physical frame of the active level 4 table from the CR3 register.
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();

    //take its physical start address, convert it to a u64, and add it to physical_memory_offset to get the virtual address where the page table frame is mapped
    let virt = physical_memory_offset + phys.as_u64();

    //use the as_mut_ptr to convert the virtual address to a *mut PageTable raw pointer
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafely return &mut PageTable reference
}
