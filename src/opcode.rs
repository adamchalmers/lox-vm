#[derive(Debug)]
#[repr(u8)]
pub enum Opcode {
    Constant,
    Negate,
    Return,
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for Opcode {
    type Error = CouldNotDecodeOpcode;

    fn try_from(opcode: u8) -> Result<Self, Self::Error> {
        let opcode = match opcode {
            0 => Self::Constant,
            1 => Self::Negate,
            2 => Self::Return,
            _ => return Err(CouldNotDecodeOpcode { opcode }),
        };
        Ok(opcode)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Could not decode opcode '{opcode}'")]
pub struct CouldNotDecodeOpcode {
    pub opcode: u8,
}
