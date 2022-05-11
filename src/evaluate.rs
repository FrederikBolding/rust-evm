use crate::Opcode;
use crate::EVM;
use primitive_types::{H256, U256};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum EvalResult {
    Continue(),
    Jump(usize),
    Exit(),
    Error(),
}

pub fn evaluate(vm: &mut EVM, opcode: Opcode) -> EvalResult {
    println!("Evaluating {:?}", opcode);
    match opcode {
        _ if opcode.is_push() => {
            let push_size = usize::from(opcode.as_u8() - 0x5f);
            let start = vm.program_counter + 1;
            let push_value = &vm.code[start..(start + push_size)];

            // Copy slice to array
            let mut val = [0u8; 32];
            val[(32 - push_value.len())..32].copy_from_slice(push_value);

            vm.stack.push(H256(val));
            vm.program_counter += push_size;
            return EvalResult::Continue();
        }
        _ if opcode.is_swap() => {
            let stack_pos = opcode.as_u8() - 0x8f;
            vm.stack.swap(stack_pos);
            return EvalResult::Continue();
        }
        _ if opcode.is_dup() => {
            let stack_pos = opcode.as_u8() - 0x7f;
            vm.stack.dup(stack_pos);
            return EvalResult::Continue();
        }
        Opcode::STOP => EvalResult::Exit(),
        Opcode::ADD => {
            let popped = vm.stack.pop_n(2);
            vm.stack
                .push(to_h256(to_u256(popped[0]) + to_u256(popped[1])));
            return EvalResult::Continue();
        }
        Opcode::SUB => {
            let popped = vm.stack.pop_n(2);
            vm.stack
                .push(to_h256(to_u256(popped[0]) - to_u256(popped[1])));
            return EvalResult::Continue();
        }
        Opcode::MUL => {
            let popped = vm.stack.pop_n(2);
            vm.stack
                .push(to_h256(to_u256(popped[0]) * to_u256(popped[1])));
            return EvalResult::Continue();
        }
        Opcode::DIV => {
            let popped = vm.stack.pop_n(2);
            vm.stack
                .push(to_h256(to_u256(popped[0]) / to_u256(popped[1])));
            return EvalResult::Continue();
        }
        Opcode::EQ => {
            let popped = vm.stack.pop_n(2);
            vm.stack.push(if popped[0] == popped[1] {
                to_h256(U256::one())
            } else {
                H256::zero()
            });
            return EvalResult::Continue();
        }
        Opcode::LT => {
            let popped = vm.stack.pop_n(2);
            vm.stack.push(if popped[0] < popped[1] {
                to_h256(U256::one())
            } else {
                H256::zero()
            });
            return EvalResult::Continue();
        }
        Opcode::GT => {
            let popped = vm.stack.pop_n(2);
            vm.stack.push(if popped[0] > popped[1] {
                to_h256(U256::one())
            } else {
                H256::zero()
            });
            return EvalResult::Continue();
        }
        Opcode::EXP => {
            let popped = vm.stack.pop_n(2);
            let first = to_u256(popped[0]);
            let second = to_u256(popped[1]);
            let exp = to_h256(first.pow(second));
            vm.stack.push(exp);
            return EvalResult::Continue();
        }
        Opcode::POP => {
            vm.stack.pop();
            return EvalResult::Continue();
        }
        Opcode::CALLDATALOAD => {
            let pos = to_u256(vm.stack.pop()).as_usize();
            let load = &vm.data[pos..pos + 32];
            vm.stack.push(H256::from_slice(load));
            return EvalResult::Continue();
        }
        Opcode::JUMP => {
            let destination = to_u256(vm.stack.pop());
            // todo: check valid jump
            return EvalResult::Jump(destination.as_usize());
        }
        Opcode::JUMPI => {
            let popped = vm.stack.pop_n(2);
            let destination = to_u256(popped[0]);
            let condition = to_u256(popped[1]);
            if !condition.is_zero() {
                // todo: check valid jump
                return EvalResult::Jump(destination.as_usize());
            }
            return EvalResult::Continue();
        }
        // Effectively no-op
        Opcode::JUMPDEST => EvalResult::Continue(),
        _ => {
            panic!("{:?} is not implemented", opcode);
        }
    }
}

fn to_u256(x: H256) -> U256 {
    let bytes = x.to_fixed_bytes();
    return U256::from(bytes);
}

fn to_h256(x: U256) -> H256 {
    let mut value = H256::default();
    x.to_big_endian(&mut value[..]);
    return value;
}
