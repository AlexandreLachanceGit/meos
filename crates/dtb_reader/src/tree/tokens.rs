use crate::tree::ParsingError;

#[derive(Debug)]
pub enum Tokens {
    BeginNode,
    EndNode,
    Property,
    Nop,
    End,
}

impl TryFrom<u32> for Tokens {
    type Error = ParsingError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match u32::from_be(value) {
            1 => Ok(Self::BeginNode),
            2 => Ok(Self::EndNode),
            3 => Ok(Self::Property),
            4 => Ok(Self::Nop),
            9 => Ok(Self::End),
            _ => Err(ParsingError::InvalidToken),
        }
    }
}

pub fn skip_nops(current: *const u32) -> Result<*const u32, ParsingError> {
    let mut curr = current;
    while matches!(unsafe { Tokens::try_from(*curr)? }, Tokens::Nop) {
        curr = unsafe { curr.add(1) };
    }
    Ok(curr)
}
