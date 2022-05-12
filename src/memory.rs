use primitive_types::{H256};

pub struct Memory {
    pub _memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            _memory: Vec::new(),
        }
    }

    pub fn extend(&mut self, offset: usize, size: usize) {
        if size == 0 {
            return;
        }
        // May need to be capped
        let extend_length = offset + size;
        let new_length = self._memory.len() + extend_length;
        self._memory.resize(new_length, 0);
    }

    pub fn write(&mut self, offset: usize, size: usize, value: H256) {
        if size == 0 {
            return;
        }
        let bytes = value.as_bytes();
        self._memory[offset..(offset + size)].clone_from_slice(&bytes[..size])
    }
}
