#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    POP,
    MLOAD,
    MSTORE,
    PUSH1,
    DUP1,
    RETURN,
}

pub fn op(code: Opcode) -> u8 {
    code.to_u8()
}

impl Opcode {
    pub fn to_u8(self) -> u8 {
        match self {
            Opcode::STOP => 0x00,
            Opcode::ADD => 0x01,
            Opcode::MUL => 0x02,
            Opcode::SUB => 0x03,
            Opcode::DIV => 0x04,
            Opcode::POP => 0x50,
            Opcode::MLOAD => 0x51,
            Opcode::MSTORE => 0x52,
            Opcode::PUSH1 => 0x60,
            Opcode::DUP1 => 0x80,
            Opcode::RETURN => 0xf3,
        }
    }

    pub fn decode(byte: u8) -> Option<Opcode> {
        match byte {
            0x00 => Some(Opcode::STOP),
            0x01 => Some(Opcode::ADD),
            0x02 => Some(Opcode::MUL),
            0x03 => Some(Opcode::SUB),
            0x04 => Some(Opcode::DIV),
            0x50 => Some(Opcode::POP),
            0x51 => Some(Opcode::MLOAD),
            0x52 => Some(Opcode::MSTORE),
            0x60 => Some(Opcode::PUSH1),
            0x80 => Some(Opcode::DUP1),
            0xf3 => Some(Opcode::RETURN),
            _ => None,
        }
    }
}
