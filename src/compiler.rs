use regex_syntax::hir::{
    visit, Anchor, Class, Hir, HirKind, Literal, RepetitionKind, RepetitionRange, Visitor,
};
use regex_syntax::Parser;

use crate::program::{Action, Instruction, Program, ProgramItem};

pub struct Compiler {}

impl Compiler {
    pub fn compile(pattern: &str) -> Program {
        let hir = Parser::new().parse(pattern).unwrap();
        visit(&hir, ProgramFactory::default()).unwrap()
    }
}

struct ProgramFactory {
    program: Program,
    is_repetition: bool,
    branch_counter: usize,
    jump_counter: usize,
}

impl Default for ProgramFactory {
    fn default() -> Self {
        Self {
            program: Vec::new(),
            is_repetition: false,
            branch_counter: 0,
            jump_counter: 0,
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
        if let HirKind::Alternation(_) = hir.kind() {
            self.program[self.jump_counter].instruction = Instruction::Jump(self.program.len());
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
            HirKind::Alternation(_) => {
                self.program.push(ProgramItem {
                    instruction: Instruction::Branch(0),
                    action: Action {
                        next: self.program.len() + 1, // unused value
                        offset: 0,                    // unused value
                    },
                });
                self.branch_counter = self.program.len() - 1;
            }
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

    fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
        self.program.push(ProgramItem {
            instruction: Instruction::Jump(0),
            action: Action { next: 0, offset: 0 },
        });
        self.jump_counter = self.program.len() - 1;

        self.program[self.branch_counter].instruction = Instruction::Branch(self.program.len());
        Ok(())
    }
    fn finish(self) -> Result<Self::Output, Self::Err> {
        Ok(self.program)
    }
}
