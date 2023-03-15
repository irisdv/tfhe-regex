use machine::Machine;
use regex_syntax::hir::visit;
use regex_syntax::Parser;

use crate::machine::ProgramFactory;

mod machine;
#[cfg(test)]
mod tests;

fn main() {
    let hir = Parser::new().parse(r"abc").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    println!("{:?}", machine.run("abcc".to_string()));
}
