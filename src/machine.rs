use crate::program::{Instruction, Program};

#[derive(Default, Clone, Debug)]
struct Context {
    program_counter: usize,
    string_counter: usize,
}

type Stack = Vec<Context>;

pub struct Machine {
    program_counter: usize,
    string_counter: usize,
    program: Program,
    stack: Stack,
}

impl Machine {
    pub fn new(program: Program) -> Self {
        Self {
            program_counter: 0,
            string_counter: 0,
            program,
            stack: Stack::new(),
        }
    }

    pub fn reset(&mut self) {
        self.program_counter = 0;
        self.string_counter = 0;
        self.stack = Stack::new();
    }

    pub fn run(&mut self, input: String) -> bool {
        let mut state = 0;
        let mut exact_match = false;

        while self.program_counter < self.program.len() {
            if state == self.program.len() {
                // End of program, return true
                return true;
            }

            let current_item = self.program.get(self.program_counter).unwrap();

            match current_item.instruction.clone() {
                Instruction::Char(c) => {
                    if self.string_counter >= input.len() {
                        return false;
                    }
                    let result = input.as_bytes()[self.string_counter] == c;
                    if !result {
                        if self.stack.is_empty() {
                            // Failed match, backtrack to previous state
                            let prev_state = self.program_counter.saturating_sub(1);
                            let prev_item = self.program[prev_state].clone();
                            match prev_item.instruction {
                                Instruction::Jump(_) => {
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
                Instruction::Match => {
                    // check qu'on en est à la fin de la string
                    return self.string_counter == input.len();
                }
                Instruction::Start => {
                    self.program_counter += 1;
                    exact_match = true;
                }
                Instruction::Repetition(c) => {
                    let result = input.as_bytes()[self.string_counter] == c;
                    if result {
                        self.string_counter =
                            (self.string_counter as i32 + current_item.action.offset) as usize;
                    } else {
                        state = current_item.action.next;
                        self.program_counter += 1;
                    }
                }
                Instruction::OptionalChar(c) => {
                    let result = input.as_bytes()[self.string_counter] == c;
                    if result {
                        // if it matches we will go next character of the string
                        self.string_counter =
                            (self.string_counter as i32 + current_item.action.offset) as usize;
                    }
                    // if it doesn't match then we stay at the same sc but fo on pc right to the next state and next instruction
                    state = current_item.action.next;
                    self.program_counter += 1;
                }
                Instruction::IntervalChar(ranges) => {
                    let mut has_matched = false;
                    let result = input.as_bytes()[self.string_counter];
                    for range in ranges.iter() {
                        if range.start() as u8 <= result && result <= range.end() as u8 {
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
                Instruction::Branch(pc) => {
                    let context = Context {
                        program_counter: pc,
                        string_counter: self.string_counter,
                    };
                    self.stack.push(context);
                    self.program_counter += 1;
                }
                Instruction::Jump(pc) => {
                    self.program_counter = pc;
                }
            }
        }
        true
    }
}

