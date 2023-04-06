use tfhe::shortint::{Ciphertext, ClientKey, ServerKey};

pub trait EncodedCipherTrait {
    fn encrypt(client_key: &ClientKey, c: u8) -> Self;
    fn decrypt(self, client_key: &ClientKey) -> u8;

    fn equal(self, server_key: &ServerKey, rhs: Self) -> Ciphertext;
    fn greater_or_equal(self, server_key: &ServerKey, rhs: Self) -> Ciphertext;
    fn less_or_equal(self, server_key: &ServerKey, rhs: Self) -> Ciphertext;
}

pub fn convert_str_to_cts<T:EncodedCipherTrait>(input: &str, client_key: &ClientKey) -> Vec<T> {
    input
    .bytes()
        .map(|c| {
            T::encrypt(client_key, c)
        })
        .collect()
}


#[derive(Clone)]
pub struct EncodedCipher4bits {
    upper: Ciphertext,
    lower: Ciphertext,
}

impl EncodedCipherTrait for EncodedCipher4bits {
    fn encrypt(client_key: &ClientKey, c: u8) -> Self {
        let upper = client_key.encrypt(((c >> 4) & 0x0F) as u64);
        let lower = client_key.encrypt((c & 0x0F) as u64);
        EncodedCipher4bits {
            upper: upper,
            lower: lower,
        }
    }

    fn decrypt(self, client_key: &ClientKey) -> u8 {
        let upper = client_key.decrypt(&self.upper) as u8;
        let lower = client_key.decrypt(&self.lower) as u8;
        ((upper & 0x0F) << 4) | (lower)
    }

    fn equal(self, server_key: &ServerKey, rhs: Self) -> Ciphertext {
        let equal_lower = server_key.unchecked_equal(&self.lower, &rhs.lower);
        let equal_upper = server_key.unchecked_equal(&self.upper, &rhs.upper);
        server_key.unchecked_mul_lsb(&equal_lower, &equal_upper)
    }

    fn greater_or_equal(self, server_key: &ServerKey, rhs: Self) -> Ciphertext {
        let result_upper = server_key.unchecked_greater(&self.upper, &rhs.upper);
        let equal_upper = server_key.unchecked_equal(&self.upper, &rhs.upper);
        let result_lower = server_key.unchecked_greater_or_equal(&self.lower, &rhs.lower);
        let result = server_key.unchecked_mul_lsb(&equal_upper, &&result_lower);
        server_key.unchecked_add(&result_upper, &result)
    }

    fn less_or_equal(self, server_key: &ServerKey, rhs: Self) -> Ciphertext {
        let result_upper = server_key.unchecked_less(&self.upper, &rhs.upper);
        let equal_upper = server_key.unchecked_equal(&self.upper, &rhs.upper);
        let result_lower = server_key.unchecked_less_or_equal(&self.lower, &rhs.lower);

        let result = server_key.unchecked_mul_lsb(&equal_upper, &result_lower);
        server_key.unchecked_add(&result_upper, &result)
    }
}

#[derive(Clone)]
pub struct EncodedCipher2bits {
    // MSB - LSB
    // i | j | k | l
    i: Ciphertext,
    j: Ciphertext,
    k: Ciphertext,
    l: Ciphertext,
}

impl EncodedCipherTrait for EncodedCipher2bits {
    fn encrypt(client_key: &ClientKey, c: u8) -> Self {
        let i = client_key.encrypt(((c >> 6) & 0x03) as u64);
        let j = client_key.encrypt(((c >> 4) & 0x03) as u64);
        let k = client_key.encrypt(((c >> 2) & 0x03) as u64);
        let l = client_key.encrypt((c & 0x3) as u64);
        EncodedCipher2bits { i: i, j: j, k: k, l: l }
    }

    fn decrypt(self, client_key: &ClientKey) -> u8 {
        let i = client_key.decrypt(&self.i) as u8;
        let j = client_key.decrypt(&self.j) as u8;
        let k = client_key.decrypt(&self.k) as u8;
        let l = client_key.decrypt(&self.l) as u8;
        ((i & 0x03) << 6) | ((j & 0x03) << 4) | ((k & 0x03) << 2) | (l & 0x03)
    }

    fn equal(self, server_key: &ServerKey, rhs: Self) -> Ciphertext {
        let result_i = server_key.unchecked_equal(&self.i, &rhs.i);
        let result_j = server_key.unchecked_equal(&self.j, &rhs.j);
        let result_k = server_key.unchecked_equal(&self.k, &rhs.k);
        let result_l = server_key.unchecked_equal(&self.l, &rhs.l);
        let result_upper = server_key.unchecked_mul_lsb(&result_i, &result_j);
        let result_lower = server_key.unchecked_mul_lsb(&result_k, &result_l);
        server_key.unchecked_mul_lsb(&result_upper, &result_lower)
    }

    fn greater_or_equal(self, server_key: &ServerKey, rhs: Self) -> Ciphertext {
        // (Ai > Bi) + (Ai == Bi) *
        //          (Aj > Bj) + (Aj == Bj) *
        //                  (Ak > Bk) + (Ak == Bk) *
        //                          (Al >= Bl) 
        let result_i = server_key.unchecked_greater(&self.i, &rhs.i);
        let result_i_equal = server_key.unchecked_equal(&self.i, &rhs.i);
        let result_j = server_key.unchecked_greater(&self.j, &rhs.j);
        let result_j_equal = server_key.unchecked_equal(&self.j, &rhs.j);
        let result_k = server_key.unchecked_greater(&self.k, &rhs.k);
        let result_k_equal = server_key.unchecked_equal(&self.k, &rhs.k);
        let result_l = server_key.unchecked_greater_or_equal(&self.l, &rhs.l);

        let result = server_key.unchecked_mul_lsb(&result_k_equal, &result_l);
        let result = server_key.unchecked_add(&result_k, &result);
        let result = server_key.unchecked_mul_lsb(&result_j_equal, &result);
        let result = server_key.unchecked_add(&result_j, &result);
        let result = server_key.smart_scalar_greater_or_equal(&result, 1_u8);
        let result = server_key.unchecked_mul_lsb(&result_i_equal, &result);
        server_key.unchecked_add(&result_i, &result)
    }

    fn less_or_equal(self, server_key: &ServerKey, rhs: Self) -> Ciphertext {
        // (Ai < Bi) + (Ai == Bi) *
        //          (Aj < Bj) + (Aj == Bj) *
        //                  (Ak < Bk) + (Ak == Bk) *
        //                          (Al <= Bl) 
        let result_i = server_key.unchecked_less(&self.i, &rhs.i);
        let result_i_equal = server_key.unchecked_equal(&self.i, &rhs.i);
        let result_j = server_key.unchecked_less(&self.j, &rhs.j);
        let result_j_equal = server_key.unchecked_equal(&self.j, &rhs.j);
        let result_k = server_key.unchecked_less(&self.k, &rhs.k);
        let result_k_equal = server_key.unchecked_equal(&self.k, &rhs.k);
        let result_l = server_key.unchecked_less_or_equal(&self.l, &rhs.l);

        let result = server_key.unchecked_mul_lsb(&result_k_equal, &result_l);
        let result = server_key.unchecked_add(&result_k, &result);
        let result = server_key.unchecked_mul_lsb(&result_j_equal, &result);
        let result = server_key.unchecked_add(&result_j, &result);
        let result = server_key.smart_scalar_greater_or_equal(&result, 1_u8);
        let result = server_key.unchecked_mul_lsb(&result_i_equal, &result);
        server_key.unchecked_add(&result_i, &result)
    }
}