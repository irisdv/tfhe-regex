use crate::{
    compiler, program,
    tfhe_machine::{self},
    CheckerCipher,
};
use tfhe::shortint::prelude::*;
use tfhe_regex::convert_str_to_cts;

fn get_keys() -> Result<(ClientKey, ServerKey, CheckerCipher), String> {
    let (client_key, server_key) = gen_keys(Parameters::default());
    let checker = CheckerCipher {
        client_key: client_key.clone(),
    };
    Ok((client_key, server_key, checker))
}

#[test]
fn simple_string() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"abc");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("123abc456", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn simple_string_end_matching_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"abc$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("123abc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn simple_string_end_matching_should_fail() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"abc$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("123abc456", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn simple_string_start_matching_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^abc");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abc123", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn simple_string_start_matching_should_fail() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^abc");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("123abc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn simple_string_exact_matching_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^abc$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn simple_string_exact_matching_should_fail() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^abc$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("aabc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn simple_string_exact_matching_should_fail_2() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^abc$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abccc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn simple_string_one_or_more_matching_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^ab+c$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abbc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn simple_string_one_or_more_matching_should_succeed_2() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^ab+c$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn simple_string_one_or_more_matching_should_fail() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^ab+c$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("ac", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn simple_string_zero_or_more_matching_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^ab*c$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("ac", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn simple_string_zero_or_more_matching_should_succeed_2() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^ab*c$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abbbc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn simple_string_optional_matching_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^ab?c$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program.clone(), server_key.clone());
    let result = machine.run(input, &checker);
    assert!(result);

    machine.reset();

    let input = convert_str_to_cts("ac", &client_key);
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn simple_string_optional_matching_should_fail() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^ab?c$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abbc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn simple_string_numbered_matching_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^ab{2}c$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abbc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn simple_string_numbered_matching_should_fail() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^ab{2}c$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abbbc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program.clone(), server_key.clone());
    let result = machine.run(input, &checker);
    assert!(!result);

    machine.reset();

    let input = convert_str_to_cts("abc", &client_key);
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn simple_string_numbered_matching_should_succeed_2() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^ab{3,}c$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abbbc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program.clone(), server_key.clone());
    let result = machine.run(input, &checker);
    assert!(result);

    machine.reset();

    let input = convert_str_to_cts("abbbbbbc", &client_key);
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn simple_string_numbered_matching_should_fail_2() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^ab{3,}c$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abbc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn simple_string_numbered_matching_should_succeed_3() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^ab{2,4}c$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abbbbc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn simple_string_numbered_matching_should_fail_3() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^ab{2,4}c$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program.clone(), server_key.clone());
    let result = machine.run(input, &checker);
    assert!(!result);

    machine.reset();

    let input = convert_str_to_cts("abbbbbc", &client_key);
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn escaping_special_characters_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^\.$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts(".", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn escaping_special_characters_should_succeed_2() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^\*$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("*", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn character_range_matching_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^[abc]$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("a", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn character_range_matching_should_fail() {
    // let program = Compiler::compile(r"^[abc]$");
    // let mut machine = Machine::new(program);
    // assert!(!machine.run("d".to_string()));
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^[abc]$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("d", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn character_range_not_matching_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^[^ade]$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("b", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn character_range_not_matching_should_fail() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^[^ade]$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("a", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn any_character_matching_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^.$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("A", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn case_insensitive_argument_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"(?i)^abc$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("ABC", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn alternation_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"0a|bcd$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("0a", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program.clone(), server_key.clone());
    let result = machine.run(input, &checker);
    assert!(result);

    machine.reset();

    let input = convert_str_to_cts("bcd", &client_key);
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn alternation_should_succeed_2() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"a(bc|ed)42$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abc42", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program.clone(), server_key.clone());
    let result = machine.run(input, &checker);
    assert!(result);

    machine.reset();

    let input = convert_str_to_cts("aed42", &client_key);
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn alternation_should_fail() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"0a|bcd$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("0b", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program.clone(), server_key.clone());
    let result = machine.run(input, &checker);
    assert!(!result);

    machine.reset();

    let input = convert_str_to_cts("bce", &client_key);
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn alternation_should_fail_2() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"a(bc|ed)42$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("abd42", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program.clone(), server_key.clone());
    let result = machine.run(input, &checker);
    assert!(!result);

    machine.reset();

    let input = convert_str_to_cts("abed42", &client_key);
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn alternation_string_numbered_matching_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^hel(ab{2}|l{3,}o)bc$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("helabbbc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program.clone(), server_key.clone());
    let result = machine.run(input, &checker);
    assert!(result);

    machine.reset();

    let input = convert_str_to_cts("helllllllobc", &client_key);
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn alternation_string_numbered_matching_should_fail() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^hel(ab{2}|l{3,}o)bc$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("helabbc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program.clone(), server_key.clone());
    let result = machine.run(input, &checker);
    assert!(!result);

    machine.reset();

    let input = convert_str_to_cts("helllobc", &client_key);
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn repetition_with_range_should_succeed() {
    let (client_key, server_key, checker) = get_keys().unwrap();

    let program = compiler::Compiler::compile(r"^01[b-e]{4}56$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("01bbbb56", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program.clone(), server_key.clone());
    let result = machine.run(input, &checker);
    assert!(result);

    machine.reset();

    let input = convert_str_to_cts("01bcde56", &client_key);
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}

#[test]
fn repetition_with_range_should_fail() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^01[b-e]{4}56$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("01bb56", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program.clone(), server_key.clone());
    let result = machine.run(input, &checker);
    assert!(!result);

    machine.reset();

    let input = convert_str_to_cts("01bcfg56", &client_key);
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(!result);
}

#[test]
fn repetition_with_range_should_succeed_1() {
    let (client_key, server_key, checker) = get_keys().unwrap();
    let program = compiler::Compiler::compile(r"^hel(a[b-e]{2}|[l-n]{3,}o)bc$");
    let program = program::cipher_program(&client_key, program);

    let input = convert_str_to_cts("helacdbc", &client_key);

    let mut machine = tfhe_machine::TFHEMachine::new(program.clone(), server_key.clone());
    let result = machine.run(input, &checker);
    assert!(result);

    machine.reset();

    let input = convert_str_to_cts("hellllobc", &client_key);
    let mut machine = tfhe_machine::TFHEMachine::new(program, server_key);
    let result = machine.run(input, &checker);
    assert!(result);
}
