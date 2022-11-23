use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{OffsetPageTable, PageTable, PhysFrame, Size4KiB, FrameAllocator}
};

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

//the complete physical memory is mapped to virtual memory at  the passed 'physical_memory_offset`. Also, this function must be called once
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable { // return a mutable reference to the active level 4 table
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

// Create an example mapping for the given page to frame `0xb8000`.
//pub fn create_example_mapping(
    //page: Page,
    //mapper: &mut OffsetPageTable,
    //frame_allocator: &mut impl FrameAllocator<Size4KiB>,
//) {
    //use x86_64::structures::paging::PageTableFlags as Flags;

    //let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    //let flags = Flags::PRESENT | Flags::WRITABLE;

    //let map_to_result = unsafe {
        //mapper.map_to(page, frame, flags, frame_allocator)
    //};
    //map_to_result.expect("map_to failed").flush();
//}

// A FrameAllocator that always returns `None`.


// ------------------------------------------------------------------------------------------------------// Boot Info Allocator
// ------------------------------------------------------------------------------------------------------

// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

//implement the FrameAllocator trait
unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

//Create a FrameAllocator from the passed memory map
impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    //return an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // get usable regions from memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        // map each region to its address range
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}



//Translates the given virtual address to the mapped physical address, or `None` if the address is not mapped
//pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    //translate_addr_inner(addr, physical_memory_offset)
//}

//fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr)
    //-> Option<PhysAddr>
//{
    //use x86_64::structures::paging::page_table::FrameError;
    //use x86_64::registers::control::Cr3;

    // read the active level 4 frame from the CR3 register
    //let (level_4_table_frame, _) = Cr3::read();

    //let table_indexes = [
        //addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    //];
    //let mut frame = level_4_table_frame;

    // traverse the multi-level page table
    //for &index in &table_indexes {
        // convert the frame into a page table reference
        //let virt = physical_memory_offset + frame.start_address().as_u64();
        //let table_ptr: *const PageTable = virt.as_ptr();
        //let table = unsafe {&*table_ptr};

        // read the page table entry and update `frame`
        //let entry = &table[index];
        //frame = match entry.frame() {
            //Ok(frame) => frame,
            //Err(FrameError::FrameNotPresent) => return None,
            //Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        //};
    //}

    // calculate the physical address by adding the page offset
    //Some(frame.start_address() + u64::from(addr.page_offset()))
//}
//
