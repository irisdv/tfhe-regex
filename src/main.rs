use regex_syntax::hir::{visit, Hir, Visitor};
use regex_syntax::Parser;

struct MyVisitor;

impl Visitor for MyVisitor {
    type Err = ();
    type Output = ();

    fn visit_pre(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
        println!("visit_pre");
        println!("{:?}", _hir);
        Ok(())
    }

    fn start(&mut self) {
        println!("start");
    }

    fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
        println!("visit_alternation_in");
        Ok(())
    }

    fn visit_post(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
        println!("visit_post");
        println!("{:?}", _hir);
        Ok(())
    }

    fn finish(self) -> Result<Self::Output, Self::Err> {
        println!("finish");
        Ok(())
    }
}

fn main() {
    let hir = Parser::new().parse(r"[^abcf]+").unwrap();
    let result = visit(&hir, MyVisitor {});
    println!("{:?}", result);
}
