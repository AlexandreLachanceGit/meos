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
    InvalidUtf8NodeName(Utf8Error),
}

impl From<Utf8Error> for FdtParsingError {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidUtf8NodeName(value)
    }
}

pub trait FdtTreeNode {
    fn name(&self) -> &'static str;
    fn properties(&self) -> impl Iterator<Item = FdtProperty>;
    fn children(&self) -> impl Iterator<Item = impl FdtTreeNode>;
}

#[derive(Debug, Clone, Copy)]
pub struct FdtNode {
    name: &'static str,
    props_addr: Option<*const u32>,
    children_addr: Option<*const u32>,
    str_block_ptr: *const u8,
}

impl FdtTreeNode for FdtNode {
    fn name(&self) -> &'static str {
        self.name
    }

    fn properties(&self) -> impl Iterator<Item = FdtProperty> {
        PropertyIter::new(self.props_addr, self.str_block_ptr)
    }

    fn children(&self) -> impl Iterator<Item = impl FdtTreeNode> {
        ChildNodeIter::new(self.children_addr)
    }
}

impl FdtNode {
    // https://devicetree-specification.readthedocs.io/en/stable/flattened-format.html#tree-structure
    pub fn parse(
        node_start_ptr: *const u32,
        str_block_ptr: *const u8,
    ) -> Result<FdtNode, FdtParsingError> {
        let mut curr = node_start_ptr;

        unsafe {
            curr = skip_nops(curr)?;

            // FDT_BEGIN_NODE token
            if !matches!(FdtTokens::try_from(*curr)?, FdtTokens::BeginNode) {
                return Err(FdtParsingError::MalformedTree);
            }
            curr = curr.add(1);

            // Node name (null terminated)
            let name_cstr = CStr::from_ptr(curr as *const u8);
            let name = name_cstr.to_str()?;

            // Add (len of string + null terminator) / bytes per u32
            curr = curr.add((name_cstr.count_bytes() + 1).div_ceil(4));

            curr = skip_nops(curr)?;

            match FdtTokens::try_from(*curr)? {
                FdtTokens::BeginNode => {
                    return Ok(FdtNode {
                        name,
                        props_addr: None,
                        children_addr: Some(curr),
                        str_block_ptr,
                    });
                }
                FdtTokens::EndNode => {
                    return Ok(FdtNode {
                        name,
                        props_addr: None,
                        children_addr: None,
                        str_block_ptr,
                    });
                }
                FdtTokens::End => return Err(FdtParsingError::MalformedTree),
                FdtTokens::Nop => unreachable!(), // due to skipping right before
                FdtTokens::Property => {}         // continue
            }

            let props_addr = curr;

            loop {
                let token = FdtTokens::try_from(*curr)?;
                match token {
                    FdtTokens::BeginNode => {
                        return Ok(FdtNode {
                            name,
                            props_addr: Some(props_addr),
                            children_addr: Some(curr),
                            str_block_ptr,
                        });
                    }
                    FdtTokens::EndNode => {
                        return Ok(FdtNode {
                            name,
                            props_addr: Some(props_addr),
                            children_addr: None,
                            str_block_ptr,
                        });
                    }
                    FdtTokens::End => return Err(FdtParsingError::MalformedTree),
                    FdtTokens::Property => {
                        let length = u32::from_be(*curr.add(1));
                        curr = curr.add(3 + length.div_ceil(4) as usize);
                    }
                    FdtTokens::Nop => {}
                }
            }
        }
    }
}
