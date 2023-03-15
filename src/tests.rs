use regex_syntax::{hir::visit, Parser};

use crate::machine::{Machine, ProgramFactory};

#[test]
fn simple_string() {
    let hir = Parser::new().parse(r"abc").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("abc".to_string()));
}
