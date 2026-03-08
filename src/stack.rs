use crate::error::VmError;

const STACK_LIMIT: usize = 1024;

#[derive(Debug)]
pub struct Stack {
    data: Vec<u128>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, value: u128) -> Result<(), VmError> {
        if self.data.len() >= STACK_LIMIT {
            return Err(VmError::StackOverflow);
        }
        self.data.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<u128, VmError> {
        self.data.pop().ok_or(VmError::StackUnderflow)
    }

    pub fn peek(&self) -> Result<u128, VmError> {
        self.data.last().copied().ok_or(VmError::StackUnderflow)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn as_slice(&self) -> &[u128] {
        &self.data
    }
}
