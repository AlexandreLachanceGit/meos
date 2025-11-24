use core::{ffi::CStr, ptr::slice_from_raw_parts};

use crate::tree::tokens::{Tokens, skip_nops};

#[derive(Debug, Clone, Copy)]
pub struct NodeProperty {
    pub name: &'static str,
    pub value: &'static [u8],
}

#[derive(Debug, Clone, Copy)]
pub struct PropertyIter {
    curr: Option<*const u32>,
    str_block_ptr: *const u8,
}

impl PropertyIter {
    pub fn new(start: Option<*const u32>, str_block_ptr: *const u8) -> PropertyIter {
        PropertyIter {
            curr: start,
            str_block_ptr,
        }
    }
}

impl Iterator for PropertyIter {
    type Item = NodeProperty;

    fn next(&mut self) -> Option<Self::Item> {
        let mut curr_ptr = self.curr.take()?;

        curr_ptr = skip_nops(curr_ptr).unwrap();

        let token = unsafe { Tokens::try_from(*curr_ptr).unwrap() };
        if !matches!(token, Tokens::Property) {
            return None;
        }

        unsafe {
            let length = { u32::from_be(*curr_ptr.add(1)) } as usize;

            let name_offset = u32::from_be(*curr_ptr.add(2)) as isize;
            let str_ptr = self.str_block_ptr.byte_offset(name_offset);

            let value: &[u8] = {
                let raw_slice_ref = slice_from_raw_parts(curr_ptr.add(3) as *const u8, length);
                &*raw_slice_ref
            };

            curr_ptr = curr_ptr.add(3 + length.div_ceil(4));

            self.curr = Some(curr_ptr);

            let name = CStr::from_ptr(str_ptr).to_str().unwrap();

            Some(NodeProperty { name, value })
        }
    }
}
