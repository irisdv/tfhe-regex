use tfhe::shortint::prelude::*;

pub mod compiler;
pub mod machine;
pub mod program;
pub mod tfhe_machine;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tfhe_machine_tests;

struct CheckerCipher {
    client_key: ClientKey,
}

impl tfhe_machine::CheckerCipherTrait for CheckerCipher {
    fn is_true(&self, ct_lower: &Ciphertext, ct_upper: &Ciphertext) -> bool {
        self.client_key.decrypt(ct_lower) == 1_u64 && self.client_key.decrypt(ct_upper) == 1_u64
    }
}

fn main() {
    let (client_key, server_key) = gen_keys(Parameters::default());

    let checker = CheckerCipher {
        client_key: client_key.clone(),
    };
    let program = compiler::Compiler::compile(r"^hel(ab{2}|l{3,}o)bc$");
    let program = program::cipher_program(&client_key, program);

    let input: Vec<[Ciphertext; 2]> = "helllllllobc"
        .chars()
        .map(|c| {
            let lower = client_key.encrypt(((c as u8) & 0x0F) as u64);
            let upper = client_key.encrypt((((c as u8) >> 4) & 0x0F) as u64);
            [lower, upper]
        })
        .collect();

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    println!("Result: {}", result);
}
