use tfhe::shortint::{ciphertext::Ciphertext, ServerKey};

use crate::program::{CipherInstruction, CipherProgram};

#[derive(Default, Clone, Debug)]
struct Context {
    program_counter: usize,
    string_counter: usize,
}

type Stack = Vec<Context>;

pub struct TFHEMachine {
    program_counter: usize,
    string_counter: usize,
    program: CipherProgram,
    stack: Stack,
    server_key: ServerKey,
}

pub trait CheckerCipherTrait {
    fn is_true(&self, ct: &Ciphertext) -> bool;
}

impl TFHEMachine {
    pub fn ct_are_equal(
        &self,
        checker: &impl CheckerCipherTrait,
        ct_left: &Ciphertext,
        ct_right: &Ciphertext,
    ) -> bool {
        let ct_result = self.server_key.unchecked_equal(ct_left, ct_right);
        checker.is_true(&ct_result)
    }
    pub fn ct_in_range(
        &self,
        checker: &impl CheckerCipherTrait,
        ct_value: &Ciphertext,
        ct_start: &Ciphertext,
        ct_end: &Ciphertext,
    ) -> bool {
        let ct_greater = self.server_key.unchecked_greater_or_equal(ct_value, ct_start);
        let ct_less = self.server_key.unchecked_less_or_equal(ct_value, ct_end);
        let ct_result = self.server_key.unchecked_mul_lsb(&ct_less, &ct_greater);
        checker.is_true(&ct_result)
    }
    pub fn new(program: CipherProgram, server_key: ServerKey) -> Self {
        Self {
            program_counter: 0,
            string_counter: 0,
            program,
            stack: Stack::new(),
            server_key: server_key,
        }
    }

    pub fn reset(&mut self) {
        self.program_counter = 0;
        self.string_counter = 0;
        self.stack = Stack::new();
    }

    pub fn run(&mut self, input: Vec<Ciphertext>, checker: &impl CheckerCipherTrait) -> bool {
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
                    let result = self.ct_are_equal(checker, &ct_input, &ct);
                    if !result {
                        if self.stack.is_empty() {
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
                CipherInstruction::CipherRepetition(ct) => {
                    let ct_input = input[self.string_counter].clone();
                    let result = self.ct_are_equal(checker, &ct_input, &ct);
                    if result {
                        self.string_counter =
                            (self.string_counter as i32 + current_item.action.offset) as usize;
                    } else {
                        state = current_item.action.next;
                        self.program_counter += 1;
                    }
                }
                CipherInstruction::CipherOptionalChar(ct) => {
                    let ct_input = input[self.string_counter].clone();
                    let result = self.ct_are_equal(checker, &ct_input, &ct);
                    if result {
                        // if it matches we will go next character of the string
                        self.string_counter =
                            (self.string_counter as i32 + current_item.action.offset) as usize;
                    }
                    // if it doesn't match then we stay at the same sc but fo on pc right to the next state and next instruction
                    state = current_item.action.next;
                    self.program_counter += 1;
                }
                CipherInstruction::CipherIntervalChar(ranges) => {
                    let mut has_matched = false;
                    let ct_input = input[self.string_counter].clone();
                    for range in ranges.iter() {
                        if self.ct_in_range(checker, &ct_input, &range.start, &range.end) {
                            // we're in the right range, it matches
                            self.string_counter =
                                (self.string_counter as i32 + current_item.action.offset) as usize;
                            state = current_item.action.next;
                            self.program_counter += 1;
                            has_matched = true;
                            break;
                        }
                    }
                    if !has_matched {
                        // If we're there then we haven't match anything yet
                        let prev_state = self.program_counter.saturating_sub(1);
                        let prev_item = self.program[prev_state].clone();
                        state = prev_item.action.next;
                        self.string_counter =
                            (self.string_counter as i32 + prev_item.action.offset) as usize;
                        self.program_counter = prev_state;
                        if exact_match {
                            return false;
                        }
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
