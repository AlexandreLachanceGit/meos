use core::{ffi::CStr, str::Utf8Error};

use crate::tree::{
    children::ChildNodeIter,
    properties::{FdtProperty, PropertyIter},
    tokens::{FdtTokens, skip_nops},
};

#[derive(Debug)]
pub enum FdtParsingError {
    InvalidToken,
    MalformedTree,
    UnexpectedToken {
        expected: FdtTokens,
        found: FdtTokens,
    },
    EarlyEnd,
    InvalidUtf8NodeName(Utf8Error),
}

impl From<Utf8Error> for FdtParsingError {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidUtf8NodeName(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FdtNode {
    name: &'static str,
    props_ptr: Option<*const u32>,
    children_ptr: Option<*const u32>,
    str_block_ptr: *const u8,
    end_ptr: *const u32,
}

impl FdtNode {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn properties(&self) -> impl Iterator<Item = FdtProperty> {
        PropertyIter::new(self.props_ptr, self.str_block_ptr)
    }

    pub fn children(&self) -> impl Iterator<Item = FdtNode> {
        ChildNodeIter::new(self.children_ptr, self.str_block_ptr)
    }

    pub fn get_child(&self, name: &str) -> Option<FdtNode> {
        ChildNodeIter::new(self.children_ptr, self.str_block_ptr).find(|c| c.name() == name)
    }

    pub fn get_property(&self, name: &str) -> Option<FdtProperty> {
        PropertyIter::new(self.props_ptr, self.str_block_ptr).find(|p| p.name == name)
    }
}

impl FdtNode {
    // https://devicetree-specification.readthedocs.io/en/stable/flattened-format.html#tree-structure
    pub(crate) fn parse(
        node_start_ptr: *const u32,
        str_block_ptr: *const u8,
    ) -> Result<FdtNode, FdtParsingError> {
        let mut curr = skip_nops(node_start_ptr)?;

        unsafe {
            let node = FdtTokens::try_from(*curr)?;
            if !matches!(node, FdtTokens::BeginNode) {
                return Err(FdtParsingError::UnexpectedToken {
                    expected: FdtTokens::BeginNode,
                    found: node,
                });
            }
            curr = curr.add(1);

            let name_cstr = CStr::from_ptr(curr as *const u8);
            let name = name_cstr.to_str()?;
            curr = curr.add((name_cstr.count_bytes() + 1).div_ceil(4));

            curr = skip_nops(curr)?;

            let props_ptr = curr;

            let mut children_ptr = None;
            let mut depth = 0;
            while let Ok(token) = FdtTokens::try_from(*curr) {
                match token {
                    FdtTokens::BeginNode => {
                        depth += 1;

                        if children_ptr.is_none() {
                            children_ptr = Some(curr);
                        }

                        curr = curr.add(1);

                        // Skip name
                        let name_cstr = CStr::from_ptr(curr as *const u8);
                        curr = curr.add((name_cstr.count_bytes() + 1).div_ceil(4));
                    }
                    FdtTokens::EndNode => {
                        if depth == 0 {
                            // End of current node
                            let end_ptr = curr;
                            let has_props = props_ptr != curr;

                            let node = FdtNode {
                                name,
                                props_ptr: if has_props { Some(props_ptr) } else { None },
                                children_ptr,
                                str_block_ptr,
                                end_ptr,
                            };
                            return Ok(node);
                        } else {
                            // End of nested node
                            depth -= 1;
                            curr = curr.add(1);
                        }
                    }
                    FdtTokens::End => {
                        // Should not happen, EndNode should be before it
                        return Err(FdtParsingError::EarlyEnd);
                    }
                    FdtTokens::Property => {
                        // Skip property
                        let length = u32::from_be(*curr.add(1));
                        curr = curr.add(3 + length.div_ceil(4) as usize);
                    }
                    FdtTokens::Nop => {
                        // Skip NOP
                        curr = curr.add(1);
                    }
                }
                curr = skip_nops(curr)?;
            }

            // Return the error that caused the escape from the loop
            Err(FdtTokens::try_from(*curr).err().unwrap())
        }
    }

    pub(crate) fn end(&self) -> *const u32 {
        self.end_ptr
    }
}
