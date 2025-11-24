#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MemoryReserveEntry {
    address: u64,
    size: u64,
}

#[derive(Debug, Default)]
pub struct MemoryReserveEntryIter {
    start: *const MemoryReserveEntry,
    curr: usize,
}

impl MemoryReserveEntryIter {
    pub fn new(start: *const u32) -> MemoryReserveEntryIter {
        MemoryReserveEntryIter {
            start: start as *const MemoryReserveEntry,
            curr: 0,
        }
    }
}

impl Iterator for MemoryReserveEntryIter {
    type Item = MemoryReserveEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = unsafe { &*self.start.add(self.curr) };
        self.curr += 1;

        match entry {
            MemoryReserveEntry {
                address: 0,
                size: 0,
            } => None,
            _ => Some(*entry),
        }
    }
}
