use crate::{error::VmError, memory::Memory, opcodes::Opcode, stack::Stack};

#[derive(Debug)]
pub struct Interpreter {
    code: Vec<u8>,
    pc: usize,
    mem: Memory,
    stack: Stack,
    stopped: bool,
    returned_data: Vec<u8>,
}

impl Interpreter {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            code,
            pc: 0,
            mem: Memory::new(),
            stack: Stack::new(),
            stopped: false,
            returned_data: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<Vec<u8>, VmError> {
        while !self.stopped {
            if self.pc >= self.code.len() {
                self.stopped = true;
                break;
            }

            let opcode_byte = self.code[self.pc];
            let opcode = Opcode::decode(opcode_byte).ok_or(VmError::InvalidOpcode(opcode_byte))?;
            self.pc += 1;

            match opcode {
                Opcode::STOP => {
                    self.stopped = true;
                }
                Opcode::ADD => {
                    let value1 = self.stack.pop()?;
                    let value2 = self.stack.pop()?;
                    self.stack.push(value1.wrapping_add(value2))?;
                }
                Opcode::MUL => {
                    let value1 = self.stack.pop()?;
                    let value2 = self.stack.pop()?;
                    self.stack.push(value1.wrapping_mul(value2))?;
                }
                Opcode::SUB => {
                    let value1 = self.stack.pop()?;
                    let value2 = self.stack.pop()?;
                    self.stack.push(value2.wrapping_sub(value1))?;
                }
                Opcode::DIV => {
                    let value1 = self.stack.pop()?;
                    let value2 = self.stack.pop()?;
                    if value1 == 0 {
                        self.stack.push(0);
                    } else {
                        self.stack.push(value2 / value1)?;
                    }
                }
                Opcode::POP => {
                    let _ = self.stack.pop()?;
                }
                Opcode::MLOAD => {
                    let offset = self.stack.pop()? as usize;
                    let value = self.mem.load_u128(offset)?;
                    self.stack.push(value)?;
                }
                Opcode::MSTORE => {
                    let offset = self.stack.pop()? as usize;
                    let value = self.stack.pop()?;
                    self.mem.store_u128(offset, value);
                }
                Opcode::PUSH1 => {
                    let value = *self.code.get(self.pc).ok_or(VmError::CodeOutOfBounds)? as u128;
                    self.pc += 1;
                    self.stack.push(value)?;
                }
                Opcode::DUP1 => {
                    let value = self.stack.peek()?;
                    self.stack.push(value)?;
                }
                Opcode::RETURN => {
                    let offset = self.stack.pop()? as usize;
                    let size = self.stack.pop()? as usize;
                    self.returned_data = self.mem.load(offset, size)?;
                    self.stopped = true;
                }
            }
        }

        Ok(self.returned_data.clone())
    }

    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    pub fn memory(&self) -> &Memory {
        &self.mem
    }
}
