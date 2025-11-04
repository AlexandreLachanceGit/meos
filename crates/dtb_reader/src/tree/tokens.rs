use crate::tree::FdtParsingError;

#[derive(Debug)]
pub enum FdtTokens {
    BeginNode,
    EndNode,
    Property,
    Nop,
    End,
}

impl TryFrom<u32> for FdtTokens {
    type Error = FdtParsingError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match u32::from_be(value) {
            1 => Ok(Self::BeginNode),
            2 => Ok(Self::EndNode),
            3 => Ok(Self::Property),
            4 => Ok(Self::Nop),
            9 => Ok(Self::End),
            _ => Err(FdtParsingError::InvalidToken),
        }
    }
}

pub fn skip_nops(current: *const u32) -> Result<*const u32, FdtParsingError> {
    let mut curr = current;
    while matches!(unsafe { FdtTokens::try_from(*curr)? }, FdtTokens::Nop) {
        curr = unsafe { curr.add(1) };
    }
    Ok(curr)
}
