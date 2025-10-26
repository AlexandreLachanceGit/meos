use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};

use spin::Mutex;

use crate::common::Allocator;

static UNINIT_MSG: &str = "Allocator uninitialized";

pub struct GlobalAllocator<A: Allocator>(Mutex<Option<A>>);

impl<A: Allocator> GlobalAllocator<A> {
    pub const fn new() -> GlobalAllocator<A> {
        GlobalAllocator(Mutex::new(None))
    }

    pub fn init(&self, start: usize, size: usize) {
        *self.0.lock() = Some(A::init(start, size));
    }

    pub fn get_available(&self) -> usize {
        self.0.lock().as_ref().expect(UNINIT_MSG).get_available()
    }
}

impl<A: Allocator> Default for GlobalAllocator<A> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<A: Allocator> GlobalAlloc for GlobalAllocator<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .as_mut()
            .expect(UNINIT_MSG)
            .allocate(layout)
            .ok()
            .map_or(core::ptr::null_mut(), |a| a.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0
            .lock()
            .as_mut()
            .expect(UNINIT_MSG)
            .deallocate(unsafe { NonNull::new_unchecked(ptr) }, layout)
    }
}
