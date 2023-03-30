use tfhe::{shortint::{prelude::*}};

pub mod compiler;
pub mod machine;
pub mod program;
pub mod tfhe_machine;


#[cfg(test)]
mod tests;

struct CheckerCipher {
    client_key: ClientKey,
}

impl tfhe_machine::CheckerCipherTrait for CheckerCipher {
    fn is_true(&self, ct: &Ciphertext) -> bool {
        self.client_key.decrypt(ct) == 1_u64
    }
}

fn main() {
    let (client_key, server_key) = gen_keys(Parameters::default());

    let checker = CheckerCipher {
        client_key: client_key.clone(),
    };
    let program = compiler::Compiler::compile(r"^hel(ab{2}|l{3,}o)bc$");
    let program= program::cipher_program(&client_key, program);
    
    let input: Vec<Ciphertext> = "helllllllobc".chars().map(|c| client_key.encrypt(c as u64)).collect();
    
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    println!("Result: {}", result);
}
