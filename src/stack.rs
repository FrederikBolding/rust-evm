use primitive_types::H256;

pub struct Stack {
    _stack: Vec<H256>,
}

impl Stack {
    pub fn new() -> Self {
        Self { _stack: Vec::new() }
    }

    pub fn push(&mut self, val: H256) {
        self._stack.push(val)
    }

    pub fn pop(&mut self) -> H256 {
        let result = self._stack.pop();
        match result {
            Some(result) => result,
            None => panic!("Pop failed!"),
        }
    }

    pub fn pop_n(&mut self, n: usize) -> Vec<H256> {
        // Needs reverse?
        return self._stack.split_off(self._stack.len() - n);
    }

    pub fn swap(&mut self, i: u8) {
        todo!();
    }

    pub fn peek(&self, n: usize) -> H256 {
        return self._stack[self._stack.len() - n];
    }

    pub fn dup(&mut self, i: u8) {
        let value = self.peek(i.into());
        self.push(value);
    }
}
