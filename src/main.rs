use tfhe::{shortint::{prelude::*}};

pub mod compiler;
pub mod machine;
pub mod program;
pub mod tfhe_machine;


#[cfg(test)]
mod tests;

fn fake_is_true(_ct: &Ciphertext) -> bool {
    true
}

fn main() {
    let (client_key, server_key) = gen_keys(Parameters::default());


    let program = compiler::Compiler::compile(r"^abc$");
    let program= program::cipher_program(&client_key, program);
    
    let input: Vec<Ciphertext> = "abc".chars().map(|c| client_key.encrypt(c as u64)).collect();
    
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key, client_key);
    let result = machine.run(input, fake_is_true);
    println!("Result: {}", result);
}
