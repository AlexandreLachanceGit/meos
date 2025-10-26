use core::{alloc::Layout, ptr::NonNull};

use crate::common::{Allocator, align_up};

pub struct BumpAllocator {
    heap_end: usize,
    next_paddr: usize,
}

impl Allocator for BumpAllocator {
    fn init(start: usize, end: usize) -> Self {
        BumpAllocator {
            heap_end: end,
            next_paddr: start,
        }
    }

    fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let start_paddr = align_up(self.next_paddr, layout.align());

        let end_paddr = match start_paddr.checked_add(layout.size()) {
            Some(end) => end,
            None => return Err(()),
        };

        if end_paddr > self.heap_end {
            return Err(());
        }

        self.next_paddr = end_paddr;

        Ok(unsafe { NonNull::new_unchecked(start_paddr as *mut u8) })
    }

    fn deallocate(&mut self, _ptr: NonNull<u8>, _layout: Layout) {
        // No deallocation in bump allocator
    }

    fn get_available(&self) -> usize {
        self.heap_end - self.next_paddr
    }
}
