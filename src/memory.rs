use crate::error::VmError;

#[derive(Debug, Default)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn resize(&mut self, size: usize) {
        if self.data.len() < size {
            self.data.resize(size, 0);
        }
    }

    pub fn store(&mut self, offset: usize, value: &[u8]) {
        let end = offset + value.len();
        self.resize(end);
        self.data[offset..end].copy_from_slice(value);
    }

    pub fn load(&self, offset: usize, size: usize) -> Result<Vec<u8>, VmError> {
        let end = offset + size;
        if end > self.data.len() {
            return Err(VmError::MemoryOutOfBounds);
        }
        Ok(self.data[offset..end].to_vec())
    }

    pub fn store_u128(&mut self, offset: usize, value: u128) {
        let bytes = value.to_be_bytes();
        self.store(offset, &bytes);
    }

    pub fn load_u128(&self, offset: usize) -> Result<u128, VmError> {
        let bytes = self.load(offset, 16)?;
        let mut buf = [0u8; 16];
        buf.copy_from_slice(&bytes);
        Ok(u128::from_be_bytes(buf))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
}
