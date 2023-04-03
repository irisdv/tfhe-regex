use tfhe::shortint::{Ciphertext, ClientKey};

pub fn convert_char(char: u8) -> [u8; 2] {
    [char & 0x0F, (char >> 4) & 0x0F]
}

pub fn convert_str_to_cts(input: &str, client_key: &ClientKey) -> Vec<[Ciphertext; 2]> {
    input
        .chars()
        .map(|c| {
            let lower = client_key.encrypt((c as u8 & 0x0F) as u64);
            let upper = client_key.encrypt(((c as u8 >> 4) & 0x0F) as u64);
            [lower, upper]
        })
        .collect()
}
