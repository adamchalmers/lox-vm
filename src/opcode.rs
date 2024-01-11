#[derive(Debug)]
#[repr(u8)]
pub enum Opcode {
    Constant,
    Return,
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for Opcode {
    type Error = CouldNotDecodeOpcode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let opcode = match value {
            0 => Self::Constant,
            1 => Self::Return,
            _ => return Err(CouldNotDecodeOpcode(value)),
        };
        Ok(opcode)
    }
}

#[derive(Debug)]
pub struct CouldNotDecodeOpcode(pub u8);
