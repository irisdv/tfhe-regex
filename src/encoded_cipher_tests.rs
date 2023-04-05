use crate::CheckerCipher;
use tfhe::shortint::{parameters::PARAM_MESSAGE_4_CARRY_4, prelude::*};
use tfhe_regex::{EncodedCipher2bits, EncodedCipher4bits, EncodedCipherTrait};

type TestEncodedCipher = EncodedCipher2bits;

fn ct_is_true(ct_result: &Ciphertext, client_key: &ClientKey) -> bool {
    client_key.decrypt(ct_result) != 0_u64
}

fn get_keys() -> Result<(ClientKey, ServerKey, CheckerCipher), String> {
    let (client_key, server_key) = gen_keys(PARAM_MESSAGE_2_CARRY_2);
    let checker = CheckerCipher {
        client_key: client_key.clone(),
    };
    Ok((client_key, server_key, checker))
}

#[test]
fn check_encrypt_decrypt() {
    let (client_key, _, _) = get_keys().unwrap();
    for value in [1_u8, 245_u8, 56_u8, 67_u8, 23_u8, 69_u8, 52_u8, 123_u8, 59_u8] {
        let cipher = TestEncodedCipher::encrypt(&client_key, value);
        let result = cipher.decrypt(&client_key);
        assert!(value == result);
    }    
}

#[test]
fn check_equal() {
    let (client_key, server_key, _) = get_keys().unwrap();
    for (left, right) in [(230_u8, 230_u8), (18_u8, 18_u8), (1_u8, 1_u8)] {
        let left = TestEncodedCipher::encrypt(&client_key, left);
        let right = TestEncodedCipher::encrypt(&client_key, right);
        let result = left.equal(&server_key, right);
        assert!(ct_is_true(&result, &client_key))
    }
}

#[test]
fn check_equal_fail() {
    let (client_key, server_key, _) = get_keys().unwrap();
    for (left, right) in [(30_u8, 21_u8), (18_u8, 28_u8), (1_u8, 0_u8)] {
        let left = TestEncodedCipher::encrypt(&client_key, left);
        let right = TestEncodedCipher::encrypt(&client_key, right);
        let result = left.equal(&server_key, right);
        assert!(!ct_is_true(&result, &client_key))
    }
}

#[test]
fn check_greater_or_equal() {
    let (client_key, server_key, _) = get_keys().unwrap();
    for (left, right) in [(240_u8, 230_u8), (230_u8, 230_u8), (1_u8, 1_u8)] {
        let left = TestEncodedCipher::encrypt(&client_key, left);
        let right = TestEncodedCipher::encrypt(&client_key, right);
        let result = left.greater_or_equal(&server_key, right);
        assert!(ct_is_true(&result, &client_key))
    }
}

#[test]
fn check_greater_or_equal_fail() {
    let (client_key, server_key, _) = get_keys().unwrap();
    for (left, right) in [(16_u8, 17_u8), (230_u8, 240_u8), (0_u8, 1_u8)] {
        let left = TestEncodedCipher::encrypt(&client_key, left);
        let right = TestEncodedCipher::encrypt(&client_key, right);
        let result = left.greater_or_equal(&server_key, right);
        assert!(!ct_is_true(&result, &client_key))
    }
}

#[test]
fn check_less_or_equal() {
    let (client_key, server_key, _) = get_keys().unwrap();
    for (left, right) in [(16_u8, 17_u8), (230_u8, 230_u8), (0_u8, 1_u8)] {
        let left = TestEncodedCipher::encrypt(&client_key, left);
        let right = TestEncodedCipher::encrypt(&client_key, right);
        let result = left.less_or_equal(&server_key, right);
        assert!(ct_is_true(&result, &client_key))
    }
}

#[test]
fn check_less_or_equal_fail() {
    let (client_key, server_key, _) = get_keys().unwrap();
    for (left, right) in [(130_u8, 30_u8), (232_u8, 231_u8), (17_u8, 1_u8)] {
        let left = TestEncodedCipher::encrypt(&client_key, left);
        let right = TestEncodedCipher::encrypt(&client_key, right);
        let result = left.less_or_equal(&server_key, right);
        assert!(!ct_is_true(&result, &client_key))
    }
}
