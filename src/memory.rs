use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
    PhysAddr,
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

//Translates the given virtual address to the mapped physical address, or `None` if the address is not mapped
pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr)
    -> Option<PhysAddr>
{
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    // read the active level 4 frame from the CR3 register
    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];
    let mut frame = level_4_table_frame;

    // traverse the multi-level page table
    for &index in &table_indexes {
        // convert the frame into a page table reference
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe {&*table_ptr};

        // read the page table entry and update `frame`
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    // calculate the physical address by adding the page offset
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
