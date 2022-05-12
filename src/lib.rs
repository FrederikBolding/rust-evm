mod evaluate;
mod memory;
mod opcodes;
mod stack;

use evaluate::{evaluate, EvalResult, ExitReason};
use memory::Memory;
use opcodes::Opcode;
use stack::Stack;

pub struct EVM {
    program_counter: usize,
    code: Vec<u8>,
    data: Vec<u8>,
    stack: Stack,
    memory: Memory,
    pub return_value: Vec<u8>,
}

impl EVM {
    pub fn new(code: Vec<u8>, data: Vec<u8>, program_counter: usize) -> Self {
        Self {
            data,
            code,
            program_counter,
            stack: Stack::new(),
            memory: Memory::new(),
            return_value: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        while self.program_counter < self.code.len() {
            let result = self.step();
            match result {
                EvalResult::Continue() => self.program_counter += 1,
                EvalResult::Jump(p) => self.program_counter = p,
                EvalResult::Exit(reason) => {
                    if reason == ExitReason::Error {
                        panic!("Execution resulted in an error!")
                    }
                    break;
                }
            }
        }
    }

    fn step(&mut self) -> EvalResult {
        let opcode = Opcode(self.code[self.program_counter]);
        return evaluate(self, opcode);
    }
}
