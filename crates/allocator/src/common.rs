use core::{alloc::Layout, ptr::NonNull};

pub trait Allocator {
    fn init(start: usize, end: usize) -> Self;
    fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, ()>;
    fn deallocate(&mut self, _ptr: NonNull<u8>, _layout: Layout);
    fn get_available(&self) -> usize;
}

pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
