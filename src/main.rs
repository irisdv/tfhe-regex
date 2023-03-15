use machine::{Instruction, Machine, Program};
use regex_syntax::hir::{visit, Hir, HirKind, Literal, Visitor};
use regex_syntax::Parser;

mod machine;

struct ProgramFactory {
    program: Program,
}

impl Default for ProgramFactory {
    fn default() -> Self {
        Self {
            program: Vec::new(),
        }
    }
}

impl Visitor for ProgramFactory {
    type Err = ();
    type Output = Vec<Instruction>;

    fn visit_pre(&mut self, hir: &Hir) -> Result<(), Self::Err> {
        match hir.kind() {
            HirKind::Concat(_) => {}
            HirKind::Literal(literal) => match literal {
                Literal::Unicode(c) => {
                    self.program.push(Instruction::Char(*c as u8));
                }
                Literal::Byte(b) => {
                    self.program.push(Instruction::Char(*b));
                }
            },
            HirKind::Empty => {
                self.program.push(Instruction::Match);
            }
            _ => todo!(),
        }
        Ok(())
    }

    fn finish(self) -> Result<Self::Output, Self::Err> {
        Ok(self.program)
    }
}

fn main() {
    let hir = Parser::new().parse(r"abc").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    println!("{:?}", machine.run("abcc".to_string()));
}
