pub mod compiler;
pub mod machine;
pub mod program;

#[cfg(test)]
mod tests;

fn main() {
    let program = compiler::Compiler::compile(r"abc");

    let mut machine = machine::Machine::new(program);
    println!("{:?}", machine.run("abcc".to_string()));
}
