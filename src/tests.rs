use crate::compiler::Compiler;
use crate::machine::Machine;

#[test]
fn simple_string() {
    let program = Compiler::compile(r"abc");
    let mut machine = Machine::new(program);
    assert!(machine.run("abc".to_string()));
    assert!(machine.run("123abc".to_string()));
    assert!(machine.run("abc123".to_string()));
    assert!(machine.run("123abc456".to_string()));
}

#[test]
fn simple_string_end_matching_should_succeed() {
    let program = Compiler::compile(r"abc$");
    let mut machine = Machine::new(program);
    assert!(machine.run("123abc".to_string()));
}

#[test]
fn simple_string_end_matching_should_fail() {
    let program = Compiler::compile(r"abc$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("123abc456".to_string()));
}

#[test]
fn simple_string_start_matching_should_succeed() {
    let program = Compiler::compile(r"^abc");
    let mut machine = Machine::new(program);
    assert!(machine.run("abc123".to_string()));
}

#[test]
fn simple_string_start_matching_should_fail() {
    let program = Compiler::compile(r"^abc");
    let mut machine = Machine::new(program);
    assert!(!machine.run("123abc".to_string()));
}

#[test]
fn simple_string_exact_matching_should_succeed() {
    let program = Compiler::compile(r"^abc$");
    let mut machine = Machine::new(program);
    assert!(machine.run("abc".to_string()));
}

#[test]
fn simple_string_exact_matching_should_fail() {
    let program = Compiler::compile(r"^abc$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("aabc".to_string()));
}

#[test]
fn simple_string_exact_matching_should_fail_2() {
    let program = Compiler::compile(r"^abc$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("abccc".to_string()));
}

#[test]
fn simple_string_one_or_more_matching_should_succeed() {
    let program = Compiler::compile(r"^ab+c$");
    let mut machine = Machine::new(program);
    assert!(machine.run("abbc".to_string()));
}

#[test]
fn simple_string_one_or_more_matching_should_succeed_2() {
    let program = Compiler::compile(r"^ab+c$");
    let mut machine = Machine::new(program);
    assert!(machine.run("abc".to_string()));
}

#[test]
fn simple_string_one_or_more_matching_should_fail() {
    let program = Compiler::compile(r"^ab+c$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("ac".to_string()));
}

#[test]
fn simple_string_zero_or_more_matching_should_succeed() {
    let program = Compiler::compile(r"^ab*c$");
    let mut machine = Machine::new(program);
    assert!(machine.run("ac".to_string()));
}

#[test]
fn simple_string_zero_or_more_matching_should_succeed_2() {
    let program = Compiler::compile(r"^ab*c$");
    let mut machine = Machine::new(program);
    assert!(machine.run("abbbc".to_string()));
}

#[test]
fn simple_string_optional_matching_should_succeed() {
    let program = Compiler::compile(r"^ab?c$");
    let mut machine = Machine::new(program);
    assert!(machine.run("abc".to_string()));
}

#[test]
fn simple_string_optional_matching_should_succeed_2() {
    let program = Compiler::compile(r"^ab?c$");
    let mut machine = Machine::new(program);
    assert!(machine.run("ac".to_string()));
}

#[test]
fn simple_string_optional_matching_should_fail() {
    let program = Compiler::compile(r"^ab?c$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("abbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_succeed() {
    let program = Compiler::compile(r"^ab{2}c$");
    let mut machine = Machine::new(program);
    assert!(machine.run("abbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_fail() {
    let program = Compiler::compile(r"^ab{2}c$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("abbbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_fail_2() {
    let program = Compiler::compile(r"^ab{2}c$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("abc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_succeed_2() {
    let program = Compiler::compile(r"^ab{3,}c$");
    let mut machine = Machine::new(program);
    assert!(machine.run("abbbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_succeed_3() {
    let program = Compiler::compile(r"^ab{3,}c$");
    let mut machine = Machine::new(program);
    assert!(machine.run("abbbbbbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_fail_3() {
    let program = Compiler::compile(r"^ab{3,}c$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("abbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_succeed_4() {
    let program = Compiler::compile(r"^ab{2,4}c$");
    let mut machine = Machine::new(program);
    assert!(machine.run("abbbbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_fail_4() {
    let program = Compiler::compile(r"^ab{2,4}c$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("abc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_fail_5() {
    let program = Compiler::compile(r"^ab{2,4}c$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("abbbbbc".to_string()));
}

#[test]
fn escaping_special_characters_should_succeed() {
    let program = Compiler::compile(r"^\.$");
    let mut machine = Machine::new(program);
    assert!(machine.run(".".to_string()));
}

#[test]
fn escaping_special_characters_should_succeed_2() {
    let program = Compiler::compile(r"^\*$");
    let mut machine = Machine::new(program);
    assert!(machine.run("*".to_string()));
}

#[test]
fn character_range_matching_should_succeed() {
    let program = Compiler::compile(r"^[abc]$");
    let mut machine = Machine::new(program);
    assert!(machine.run("a".to_string()));
}

#[test]
fn character_range_matching_should_fail() {
    let program = Compiler::compile(r"^[abc]$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("d".to_string()));
}

#[test]
fn character_range_not_matching_should_succeed() {
    let program = Compiler::compile(r"^[^ade]$");
    let mut machine = Machine::new(program);
    assert!(machine.run("b".to_string()));
}

#[test]
fn character_range_not_matching_should_fail() {
    let program = Compiler::compile(r"^[^ade]$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("a".to_string()));
}

#[test]
fn any_character_matching_should_succeed() {
    let program = Compiler::compile(r"^.$");
    let mut machine = Machine::new(program);
    assert!(machine.run("A".to_string()));
}

#[test]
fn case_insensitive_argument_should_succeed() {
    let program = Compiler::compile(r"(?i)^abc$");
    let mut machine = Machine::new(program);
    assert!(machine.run("ABC".to_string()));
}

#[test]
fn alternation_should_succeed() {
    let program = Compiler::compile(r"0a|bcd$");
    let mut machine = Machine::new(program);
    assert!(machine.run("0a".to_string()));
    machine.reset();
    assert!(machine.run("bcd".to_string()));
}

#[test]
fn alternation_should_succeed_2() {
    let program = Compiler::compile(r"a(bc|ed)42$");
    let mut machine = Machine::new(program);
    assert!(machine.run("abc42".to_string()));
    machine.reset();
    assert!(machine.run("aed42".to_string()));
}

#[test]
fn alternation_should_fail() {
    let program = Compiler::compile(r"0a|bcd$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("0b".to_string()));
    machine.reset();
    assert!(!machine.run("bce".to_string()));
}

#[test]
fn alternation_should_fail_2() {
    let program = Compiler::compile(r"a(bc|ed)42$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("abd42".to_string()));
    machine.reset();
    assert!(!machine.run("abed42".to_string()));
}

#[test]
fn alternation_string_numbered_matching_should_succeed() {
    let program = Compiler::compile(r"^hel(ab{2}|l{3,}o)bc$");
    let mut machine = Machine::new(program);
    assert!(machine.run("helabbbc".to_string()));
    machine.reset();
    assert!(machine.run("helllllllobc".to_string()));
}

#[test]
fn alternation_string_numbered_matching_should_fail() {
    let program = Compiler::compile(r"^hel(ab{2}|l{3,}o)bc$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("helabbc".to_string()));
    machine.reset();
    assert!(!machine.run("helllobc".to_string()));
}

#[test]
fn repetition_with_range_should_succeed() {
    let program = Compiler::compile(r"^01[b-e]{4}56$");
    let mut machine = Machine::new(program);
    assert!(machine.run("01bbbb56".to_string()));
    machine.reset();
    assert!(machine.run("01bcde56".to_string()));
}

#[test]
fn repetition_with_range_should_fail() {
    let program = Compiler::compile(r"^01[b-e]{4}56$");
    let mut machine = Machine::new(program);
    assert!(!machine.run("01bb56".to_string()));
    machine.reset();
    assert!(!machine.run("01bcfg56".to_string()));
}

#[test]
fn repetition_with_range_should_succeed_1() {
    let program = Compiler::compile(r"^hel(a[b-e]{2}|[l-n]{3,}o)bc$");
    let mut machine = Machine::new(program);
    assert!(machine.run("helacdbc".to_string()));
    machine.reset();
    assert!(machine.run("hellllobc".to_string()));
}
