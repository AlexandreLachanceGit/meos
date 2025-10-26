use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};

use spin::Mutex;

pub struct Allocator {
    heap_start: Option<usize>,
    heap_end: Option<usize>,
    heap_size: usize,
    next_paddr: usize,
}

impl Allocator {
    pub const fn new() -> Allocator {
        Allocator {
            heap_start: None,
            heap_end: None,
            heap_size: 0,
            next_paddr: 0,
        }
    }

    pub fn init(&mut self, start: usize, end: usize) {
        self.heap_start = Some(start);
        self.heap_end = Some(end);
        self.heap_size = end - start;
        self.next_paddr = start;
    }

    fn align_up(addr: usize, align: usize) -> usize {
        (addr + align - 1) & !(align - 1)
    }

    pub fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        if self.heap_start.is_some() && self.heap_end.is_some() {
            let start_paddr = Allocator::align_up(self.next_paddr, layout.align());

            let end_paddr = match start_paddr.checked_add(layout.size()) {
                Some(end) => end,
                None => return Err(()),
            };

            if end_paddr > self.heap_end.unwrap() {
                return Err(());
            }

            self.next_paddr = end_paddr;

            Ok(unsafe { NonNull::new_unchecked(start_paddr as *mut u8) })
        } else {
            Err(())
        }
    }

    pub fn deallocate(&mut self, _ptr: NonNull<u8>, _layout: Layout) {
        // TODO
    }

    pub fn get_available(&self) -> usize {
        if let Some(end) = self.heap_end {
            end - self.next_paddr
        } else {
            0
        }
    }
}

pub struct GlobalAllocator(Mutex<Allocator>);

impl GlobalAllocator {
    pub const fn new() -> GlobalAllocator {
        GlobalAllocator(Mutex::new(Allocator::new()))
    }

    pub fn init(&self, start: usize, size: usize) {
        let mut alloc = self.0.lock();
        alloc.init(start, size);
    }

    pub fn get_available(&self) -> usize {
        self.0.lock().get_available()
    }
}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .allocate(layout)
            .ok()
            .map_or(core::ptr::null_mut(), |a| a.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0
            .lock()
            .deallocate(unsafe { NonNull::new_unchecked(ptr) }, layout)
    }
}
