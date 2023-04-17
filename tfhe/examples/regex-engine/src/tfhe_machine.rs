use tfhe::shortint::{ciphertext::Ciphertext, ServerKey};
use tfhe_regex::EncodedCipherTrait;

use crate::program::{CipherInstruction, CipherProgram};

#[derive(Default, Clone, Debug)]
struct Context {
    program_counter: usize,
    string_counter: usize,
}

type Stack = Vec<Context>;

pub struct TFHEMachine<T: EncodedCipherTrait + Clone> {
    program_counter: usize,
    string_counter: usize,
    program: CipherProgram<T>,
    stack: Stack,
    server_key: ServerKey,
}

pub trait CheckerCipherTrait {
    fn is_true(&self, ct_result: &Ciphertext) -> bool;
}

impl<T> TFHEMachine<T>
where
    T: EncodedCipherTrait + Clone,
{
    fn ct_are_equal(&self, checker: &impl CheckerCipherTrait, left: T, right: T) -> bool {
        let result = left.equal(&self.server_key, right);
        checker.is_true(&result)
    }

    fn ct_in_range(&self, checker: &impl CheckerCipherTrait, value: T, start: T, end: T) -> bool {
        let greater = value.clone().greater_or_equal(&self.server_key, start);
        let less = value.less_or_equal(&self.server_key, end);
        let result = self.server_key.unchecked_mul_lsb(&less, &greater);
        checker.is_true(&result)
    }

    pub fn new(program: CipherProgram<T>, server_key: ServerKey) -> Self {
        Self {
            program_counter: 0,
            string_counter: 0,
            program,
            stack: Stack::new(),
            server_key,
        }
    }

    pub fn reset(&mut self) {
        self.program_counter = 0;
        self.string_counter = 0;
        self.stack = Stack::new();
    }

    pub fn run(&mut self, input: Vec<T>, checker: &impl CheckerCipherTrait) -> bool {
        let mut state = 0;
        let mut exact_match = false;

        while self.program_counter < self.program.len() {
            if state == self.program.len() {
                // End of program, return true
                return true;
            }

            let current_item = self.program.get(self.program_counter).unwrap();

            match current_item.instruction.clone() {
                CipherInstruction::CipherChar(ct) => {
                    if self.string_counter >= input.len() {
                        return false;
                    }
                    let ct_input = input[self.string_counter].clone();
                    let result = self.ct_are_equal(checker, ct_input, ct.char);
                    if !result {
                        if ct.can_repeat || ct.is_optional {
                            state = current_item.action.next;
                            self.program_counter += 1;
                        } else if self.stack.is_empty() {
                            // Failed match, backtrack to previous state
                            let prev_state = self.program_counter.saturating_sub(1);
                            let prev_item = self.program[prev_state].clone();
                            match prev_item.instruction {
                                CipherInstruction::Jump(_) => {
                                    return false;
                                }
                                _ => {
                                    state = prev_item.action.next;
                                    self.string_counter = (self.string_counter as i32
                                        + prev_item.action.offset)
                                        as usize;
                                    self.program_counter = prev_state;
                                    if exact_match {
                                        return false;
                                    }
                                }
                            }
                        } else {
                            let context = self.stack.pop().unwrap();
                            self.program_counter = context.program_counter;
                            self.string_counter = context.string_counter;
                        }
                    } else if ct.can_repeat && !ct.is_optional {
                        // char can be repeated
                        self.string_counter =
                            (self.string_counter as i32 + current_item.action.offset) as usize;
                    } else if !ct.can_repeat && ct.is_optional {
                        // char is optional
                        self.string_counter =
                            (self.string_counter as i32 + current_item.action.offset) as usize;
                        state = current_item.action.next;
                        self.program_counter += 1;
                    } else {
                        // Successful match, advance to next state
                        state = current_item.action.next;
                        self.string_counter =
                            (self.string_counter as i32 + current_item.action.offset) as usize;
                        self.program_counter += 1;
                    }
                }
                CipherInstruction::Match => {
                    // check qu'on en est à la fin de la string
                    return self.string_counter == input.len();
                }
                CipherInstruction::Start => {
                    self.program_counter += 1;
                    exact_match = true;
                }
                CipherInstruction::CipherIntervalChar(ranges) => {
                    let mut has_matched = false;
                    let ct_input = input[self.string_counter].clone();
                    for range in ranges.range.iter() {
                        if self.ct_in_range(
                            checker,
                            ct_input.clone(),
                            range.start.clone(),
                            range.end.clone(),
                        ) {
                            // we're in the right range, it matches
                            has_matched = true;
                            break;
                        }
                    }
                    if has_matched {
                        self.string_counter =
                            (self.string_counter as i32 + current_item.action.offset) as usize;
                        if !ranges.can_repeat || ranges.is_optional {
                            state = current_item.action.next;
                            self.program_counter += 1;
                        }
                    } else if !has_matched && (ranges.is_optional || ranges.can_repeat) {
                        state = current_item.action.next;
                        self.program_counter += 1;
                    } else if self.stack.is_empty() {
                        let prev_state = self.program_counter.saturating_sub(1);
                        let prev_item = self.program[prev_state].clone();
                        match prev_item.instruction {
                            CipherInstruction::Jump(_) => {
                                return false;
                            }
                            _ => {
                                state = prev_item.action.next;
                                self.string_counter =
                                    (self.string_counter as i32 + prev_item.action.offset) as usize;
                                self.program_counter = prev_state;
                                if exact_match {
                                    return false;
                                }
                            }
                        }
                    } else {
                        let context = self.stack.pop().unwrap();
                        self.program_counter = context.program_counter;
                        self.string_counter = context.string_counter;
                    }
                }
                CipherInstruction::Branch(pc) => {
                    let context = Context {
                        program_counter: pc,
                        string_counter: self.string_counter,
                    };
                    self.stack.push(context);
                    self.program_counter += 1;
                }
                CipherInstruction::Jump(pc) => {
                    self.program_counter = pc;
                }
            }
        }
        true
    }
}
