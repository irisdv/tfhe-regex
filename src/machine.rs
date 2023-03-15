#[derive(Debug)]
pub enum Instruction {
    Char(u8),
    Match,
}

pub type Program = Vec<Instruction>;

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
        while self.program_counter < self.program.len() {
            let instruction = self.program.get(self.program_counter).unwrap();
            self.program_counter += 1;
            match instruction {
                Instruction::Char(c) => {
                    if self.string_counter >= input.len() {
                        return false;
                    }
                    let result = input.as_bytes()[self.string_counter] == *c;
                    self.string_counter += 1;
                    if !result {
                        return false;
                    }
                }
                Instruction::Match => {}
            }
        }
        true
    }
}
