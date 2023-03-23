use regex_syntax::hir::{
    Anchor, Class, ClassUnicodeRange, Hir, HirKind, Literal, RepetitionKind, RepetitionRange,
    Visitor,
};

#[derive(Debug, Clone)]
pub enum Instruction {
    Char(u8),
    Match,            // Anchor end
    Start,            // Anchor start
    Repetition(u8),   // 0 to infinite repetition of a character
    OptionalChar(u8), // in case of bounded repetitions or ZeroOrOneRepetition
    IntervalChar(Vec<ClassUnicodeRange>),
}

#[derive(Default, Clone, Debug)]
struct Action {
    next: usize,
    offset: i32,
}

#[derive(Debug, Clone)]
pub struct ProgramItem {
    instruction: Instruction,
    action: Action,
}

pub type Program = Vec<ProgramItem>;

pub struct Machine {
    program_counter: usize,
    string_counter: usize,
    program: Program,
}

impl Machine {
    pub fn new(program: Program) -> Self {
        Self {
            program_counter: 0,
            string_counter: 0,
            program,
        }
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
                        // Failed match, backtrack to previous state
                        let prev_state = self.program_counter.saturating_sub(1);
                        let prev_item = self.program[prev_state].clone();
                        state = prev_item.action.next;
                        self.string_counter =
                            (self.string_counter as i32 + prev_item.action.offset) as usize;
                        self.program_counter = prev_state;
                        if exact_match {
                            return false;
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
            }
        }
        true
    }
}

pub struct ProgramFactory {
    program: Program,
    is_repetition: bool,
}

impl Default for ProgramFactory {
    fn default() -> Self {
        Self {
            program: Vec::new(),
            is_repetition: false,
        }
    }
}

impl Visitor for ProgramFactory {
    type Err = ();
    type Output = Vec<ProgramItem>;

    fn visit_post(&mut self, hir: &Hir) -> Result<(), Self::Err> {
        if let HirKind::Repetition(_) = hir.kind() {
            self.is_repetition = false;
        }
        Ok(())
    }

    fn visit_pre(&mut self, hir: &Hir) -> Result<(), Self::Err> {
        let mut start = 0;

        match hir.kind() {
            HirKind::Concat(_) => {}
            HirKind::Literal(literal) => {
                if !self.is_repetition {
                    match literal {
                        Literal::Unicode(c) => {
                            self.program.push(ProgramItem {
                                instruction: Instruction::Char(*c as u8),
                                action: Action {
                                    next: self.program.len() + 1 + start,
                                    offset: 1,
                                },
                            });
                        }
                        Literal::Byte(b) => {
                            self.program.push(ProgramItem {
                                instruction: Instruction::Char(*b),
                                action: Action {
                                    next: self.program.len() + 1 + start,
                                    offset: 1,
                                },
                            });
                        }
                    }
                }
            }
            HirKind::Empty => {
                self.program.push(ProgramItem {
                    instruction: Instruction::Match,
                    action: Action {
                        next: self.program.len() + 1 + start,
                        offset: 1,
                    },
                });
            }
            HirKind::Anchor(anchor) => match anchor {
                Anchor::StartText => {
                    start = self.program.len();
                    self.program.push(ProgramItem {
                        instruction: Instruction::Start,
                        action: Action {
                            next: start + 1,
                            offset: 0,
                        },
                    });
                }
                Anchor::EndText => {
                    self.program.push(ProgramItem {
                        instruction: Instruction::Match,
                        action: Action {
                            next: self.program.len() + 1,
                            offset: 0,
                        },
                    });
                }
                Anchor::StartLine => todo!(),
                Anchor::EndLine => todo!(),
            },
            HirKind::Alternation(_) => todo!(),
            HirKind::Repetition(repetition) => {
                self.is_repetition = true;
                match repetition.kind.clone() {
                    RepetitionKind::OneOrMore => {
                        if let HirKind::Literal(literal) = repetition.hir.kind() {
                            let (instruction, repetition) = match literal {
                                Literal::Unicode(c) => (
                                    Instruction::Char(*c as u8),
                                    Instruction::Repetition(*c as u8),
                                ),
                                Literal::Byte(b) => {
                                    (Instruction::Char(*b), Instruction::Repetition(*b))
                                }
                            };
                            self.program.push(ProgramItem {
                                instruction,
                                action: Action {
                                    next: self.program.len() + 1 + start,
                                    offset: 1,
                                },
                            });
                            self.program.push(ProgramItem {
                                instruction: repetition,
                                action: Action {
                                    next: self.program.len() + 1 + start,
                                    offset: 1,
                                },
                            });
                        }
                    }
                    RepetitionKind::ZeroOrMore => {
                        if let HirKind::Literal(literal) = repetition.hir.kind() {
                            let instruction = match literal {
                                Literal::Unicode(c) => Instruction::Repetition(*c as u8),
                                Literal::Byte(b) => Instruction::Repetition(*b),
                            };
                            self.program.push(ProgramItem {
                                instruction,
                                action: Action {
                                    next: self.program.len() + 1 + start,
                                    offset: 1,
                                },
                            });
                        }
                    }
                    RepetitionKind::ZeroOrOne => {
                        if let HirKind::Literal(literal) = repetition.hir.kind() {
                            let instruction = match literal {
                                Literal::Unicode(c) => Instruction::OptionalChar(*c as u8),
                                Literal::Byte(b) => Instruction::OptionalChar(*b),
                            };
                            self.program.push(ProgramItem {
                                instruction,
                                action: Action {
                                    next: self.program.len() + 1 + start,
                                    offset: 1,
                                },
                            });
                        }
                    }
                    RepetitionKind::Range(range) => match range {
                        RepetitionRange::Exactly(n) => {
                            if let HirKind::Literal(literal) = repetition.hir.kind() {
                                let instruction = match literal {
                                    Literal::Unicode(c) => Instruction::Char(*c as u8),
                                    Literal::Byte(b) => Instruction::Char(*b),
                                };

                                for _i in 0..n {
                                    self.program.push(ProgramItem {
                                        instruction: instruction.clone(),
                                        action: Action {
                                            next: self.program.len() + 1 + start,
                                            offset: 1,
                                        },
                                    });
                                }
                            }
                        }
                        RepetitionRange::AtLeast(n) => {
                            if let HirKind::Literal(literal) = repetition.hir.kind() {
                                let (instruction, repetition) = match literal {
                                    Literal::Unicode(c) => (
                                        Instruction::Char(*c as u8),
                                        Instruction::Repetition(*c as u8),
                                    ),
                                    Literal::Byte(b) => {
                                        (Instruction::Char(*b), Instruction::Repetition(*b))
                                    }
                                };

                                for _i in 0..n {
                                    self.program.push(ProgramItem {
                                        instruction: instruction.clone(),
                                        action: Action {
                                            next: self.program.len() + 1 + start,
                                            offset: 1,
                                        },
                                    });
                                }
                                self.program.push(ProgramItem {
                                    instruction: repetition,
                                    action: Action {
                                        next: self.program.len() + 1 + start,
                                        offset: 1,
                                    },
                                });
                            }
                        }
                        RepetitionRange::Bounded(m, n) => {
                            if let HirKind::Literal(literal) = repetition.hir.kind() {
                                let (instruction, optional_char) = match literal {
                                    Literal::Unicode(c) => (
                                        Instruction::Char(*c as u8),
                                        Instruction::OptionalChar(*c as u8),
                                    ),
                                    Literal::Byte(b) => {
                                        (Instruction::Char(*b), Instruction::OptionalChar(*b))
                                    }
                                };
                                for _i in 0..m {
                                    self.program.push(ProgramItem {
                                        instruction: instruction.clone(),
                                        action: Action {
                                            next: self.program.len() + 1 + start,
                                            offset: 1,
                                        },
                                    });
                                }

                                for _i in 0..(n - m) {
                                    self.program.push(ProgramItem {
                                        instruction: optional_char.clone(),
                                        action: Action {
                                            next: self.program.len() + 1 + start,
                                            offset: 1,
                                        },
                                    });
                                }
                            }
                        }
                    },
                }
            }
            HirKind::Class(class) => match class {
                Class::Unicode(set) => {
                    let range_chars = set.ranges().to_owned();
                    self.program.push(ProgramItem {
                        instruction: Instruction::IntervalChar(range_chars),
                        action: Action {
                            next: self.program.len() + 1 + start,
                            offset: 1,
                        },
                    })
                }
                Class::Bytes(_) => todo!(),
            },
            HirKind::Group(_) => {}
            HirKind::WordBoundary(_) => {}
        }
        Ok(())
    }

    fn finish(self) -> Result<Self::Output, Self::Err> {
        Ok(self.program)
    }
}
