#[repr(C)]
#[derive(Debug)]
pub struct FdtReserveEntry {
    address: u64,
    size: u64,
}

#[derive(Debug, Default)]
pub struct FtdReserveEntryIter {
    start: *const FdtReserveEntry,
    curr: usize,
}

impl FtdReserveEntryIter {
    pub fn new(start: *const u32) -> FtdReserveEntryIter {
        FtdReserveEntryIter {
            start: start as *const FdtReserveEntry,
            curr: 0,
        }
    }
}

impl Iterator for FtdReserveEntryIter {
    type Item = &'static FdtReserveEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = unsafe { &*self.start.add(self.curr) };
        self.curr += 1;

        match entry {
            FdtReserveEntry {
                address: 0,
                size: 0,
            } => None,
            _ => Some(entry),
        }
    }
}
