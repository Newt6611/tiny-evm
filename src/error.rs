#[derive(Debug)]
pub enum VmError {
    StackUnderflow,
    StackOverflow,
    InvalidOpcode(u8),
    CodeOutOfBounds,
    MemoryOutOfBounds,
}
