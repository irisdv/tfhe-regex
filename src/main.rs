use regex_syntax::hir::{visit, Hir, HirKind, Literal, Visitor};
use regex_syntax::Parser;

#[derive(Debug)]
enum Instruction {
    Char(u8),
    Match,
}

struct ProgramFactory {
    instructions: Vec<Instruction>,
}

impl Default for ProgramFactory {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
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
                    self.instructions.push(Instruction::Char(*c as u8));
                }
                Literal::Byte(b) => {
                    self.instructions.push(Instruction::Char(*b));
                }
            },
            HirKind::Empty => {
                self.instructions.push(Instruction::Match);
            }
            _ => todo!(),
        }
        Ok(())
    }

    fn finish(self) -> Result<Self::Output, Self::Err> {
        Ok(self.instructions)
    }
}

fn main() {
    let hir = Parser::new().parse(r"abc").unwrap();
    println!("{:?}", hir);
    let result = visit(&hir, ProgramFactory::default());
    println!("{:?}", result);
}
