use tfhe::shortint::prelude::*;
use tfhe_regex::{EncodedCipher4bits, EncodedCipherTrait};

pub mod compiler;
pub mod machine;
pub mod program;
pub mod tfhe_machine;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tfhe_machine_tests;

#[cfg(test)]
mod encoded_cipher_tests;

struct CheckerCipher {
    client_key: ClientKey,
}

impl tfhe_machine::CheckerCipherTrait for CheckerCipher {
    fn is_true(&self, ct_result: &Ciphertext) -> bool {
        self.client_key.decrypt(ct_result) != 0_u64
    }
}

fn main() {
    let (client_key, server_key) = gen_keys(Parameters::default());

    let checker = CheckerCipher {
        client_key: client_key.clone(),
    };
    let program = compiler::Compiler::compile(r"^hel(ab{2}|l{3,}o)bc$");
    let program = program::cipher_program(&client_key, program);

    let input: Vec<EncodedCipher4bits> = "helllllllobc"
        .chars()
        .map(|c| {
            EncodedCipher4bits::encrypt(&client_key, c as u8)
        })
        .collect();

    let mut machine = tfhe_machine::TFHEMachine::<EncodedCipher4bits>::new(program, server_key);
    let result = machine.run(input, &checker);
    println!("Result: {}", result);
}
