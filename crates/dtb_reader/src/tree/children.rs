use crate::{
    FdtNode,
    tree::tokens::{FdtTokens, skip_nops},
};

#[derive(Debug, Clone, Copy)]
pub struct ChildNodeIter {
    curr: Option<*const u32>,
    str_block_ptr: *const u8,
}

impl ChildNodeIter {
    pub fn new(start: Option<*const u32>, str_block_ptr: *const u8) -> ChildNodeIter {
        ChildNodeIter {
            curr: start,
            str_block_ptr,
        }
    }
}

impl Iterator for ChildNodeIter {
    type Item = FdtNode;

    fn next(&mut self) -> Option<Self::Item> {
        let mut curr_ptr = self.curr.take()?;

        curr_ptr = skip_nops(curr_ptr).unwrap();

        let token = unsafe { FdtTokens::try_from(*curr_ptr).unwrap() };
        if !matches!(token, FdtTokens::BeginNode) {
            return None;
        }

        let node = FdtNode::parse(curr_ptr, self.str_block_ptr).unwrap();
        self.curr = Some(unsafe { node.end().add(1) }); // Add 1 to skip EndNode

        Some(node)
    }
}
