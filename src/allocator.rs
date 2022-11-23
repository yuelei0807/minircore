use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

#[global_allocator]
static ALLOCATOR: Dummy = Dummy;

pub struct Dummy;

//The dummy allocator implements the trait and always returns an error when alloc is called
unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("The dealloc function should be never called")
    }
}
