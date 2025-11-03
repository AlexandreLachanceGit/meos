use crate::FdtNode;

#[derive(Debug, Clone, Copy)]
pub struct ChildNodeIter {
    curr: Option<*const u32>,
}

impl ChildNodeIter {
    pub fn new(start: Option<*const u32>) -> ChildNodeIter {
        ChildNodeIter { curr: start }
    }
}

impl Iterator for ChildNodeIter {
    type Item = FdtNode;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
