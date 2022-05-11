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
        let mut split = self._stack.split_off(self._stack.len() - n);
        split.reverse();
        return split;
    }

    pub fn swap(&mut self, i: usize) {
        let length = self._stack.len();
        let head_index = length - 1;
        let head = self._stack[head_index];
        let swap_index = length - (i + 1);
        let swap_value = self._stack[swap_index];

        self._stack[swap_index] = head;
        self._stack[head_index] = swap_value;
    }

    pub fn peek(&self, n: usize) -> H256 {
        return self._stack[self._stack.len() - n];
    }

    pub fn dup(&mut self, i: u8) {
        let value = self.peek(i.into());
        self.push(value);
    }
}
