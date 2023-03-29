use regex_syntax::{hir::visit, Parser};

use crate::machine::{Machine, ProgramFactory};

#[test]
fn simple_string() {
    let hir = Parser::new().parse(r"abc").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("abc".to_string()));
    assert!(machine.run("123abc".to_string()));
    assert!(machine.run("abc123".to_string()));
    assert!(machine.run("123abc456".to_string()));
}

#[test]
fn simple_string_end_matching_should_succeed() {
    let hir = Parser::new().parse(r"abc$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("123abc".to_string()));
}

#[test]
fn simple_string_end_matching_should_fail() {
    let hir = Parser::new().parse(r"abc$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(!machine.run("123abc456".to_string()));
}

#[test]
fn simple_string_start_matching_should_succeed() {
    let hir = Parser::new().parse(r"^abc").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("abc123".to_string()));
}

#[test]
fn simple_string_start_matching_should_fail() {
    let hir = Parser::new().parse(r"^abc").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(!machine.run("123abc".to_string()));
}

#[test]
fn simple_string_exact_matching_should_succeed() {
    let hir = Parser::new().parse(r"^abc$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("abc".to_string()));
}

#[test]
fn simple_string_exact_matching_should_fail() {
    let hir = Parser::new().parse(r"^abc$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(!machine.run("aabc".to_string()));
}

#[test]
fn simple_string_exact_matching_should_fail_2() {
    let hir = Parser::new().parse(r"^abc$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(!machine.run("abccc".to_string()));
}

#[test]
fn simple_string_one_or_more_matching_should_succeed() {
    let hir = Parser::new().parse(r"^ab+c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("abbc".to_string()));
}

#[test]
fn simple_string_one_or_more_matching_should_succeed_2() {
    let hir = Parser::new().parse(r"^ab+c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("abc".to_string()));
}

#[test]
fn simple_string_one_or_more_matching_should_fail() {
    let hir = Parser::new().parse(r"^ab+c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(!machine.run("ac".to_string()));
}

#[test]
fn simple_string_zero_or_more_matching_should_succeed() {
    let hir = Parser::new().parse(r"^ab*c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("ac".to_string()));
}

#[test]
fn simple_string_zero_or_more_matching_should_succeed_2() {
    let hir = Parser::new().parse(r"^ab*c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("abbbc".to_string()));
}

#[test]
fn simple_string_optional_matching_should_succeed() {
    let hir = Parser::new().parse(r"^ab?c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("abc".to_string()));
}

#[test]
fn simple_string_optional_matching_should_succeed_2() {
    let hir = Parser::new().parse(r"^ab?c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("ac".to_string()));
}

#[test]
fn simple_string_optional_matching_should_fail() {
    let hir = Parser::new().parse(r"^ab?c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(!machine.run("abbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_succeed() {
    let hir = Parser::new().parse(r"^ab{2}c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("abbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_fail() {
    let hir = Parser::new().parse(r"^ab{2}c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(!machine.run("abbbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_fail_2() {
    let hir = Parser::new().parse(r"^ab{2}c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(!machine.run("abc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_succeed_2() {
    let hir = Parser::new().parse(r"^ab{3,}c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("abbbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_succeed_3() {
    let hir = Parser::new().parse(r"^ab{3,}c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("abbbbbbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_fail_3() {
    let hir = Parser::new().parse(r"^ab{3,}c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(!machine.run("abbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_succeed_4() {
    let hir = Parser::new().parse(r"^ab{2,4}c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("abbbbc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_fail_4() {
    let hir = Parser::new().parse(r"^ab{2,4}c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(!machine.run("abc".to_string()));
}

#[test]
fn simple_string_numbered_matching_should_fail_5() {
    let hir = Parser::new().parse(r"^ab{2,4}c$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(!machine.run("abbbbbc".to_string()));
}

#[test]
fn escaping_special_characters_should_succeed() {
    let hir = Parser::new().parse(r"^\.$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run(".".to_string()));
}

#[test]
fn escaping_special_characters_should_succeed_2() {
    let hir = Parser::new().parse(r"^\*$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("*".to_string()));
}

#[test]
fn character_range_matching_should_succeed() {
    let hir = Parser::new().parse(r"^[abc]$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("a".to_string()));
}

#[test]
fn character_range_matching_should_fail() {
    let hir = Parser::new().parse(r"^[abc]$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(!machine.run("d".to_string()));
}

#[test]
fn character_range_not_matching_should_succeed() {
    let hir = Parser::new().parse(r"^[^ade]$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("b".to_string()));
}

#[test]
fn character_range_not_matching_should_fail() {
    let hir = Parser::new().parse(r"^[^ade]$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(!machine.run("a".to_string()));
}

#[test]
fn any_character_matching_should_succeed() {
    let hir = Parser::new().parse(r"^.$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("A".to_string()));
}

#[test]
fn case_insensitive_argument_should_succeed() {
    let hir = Parser::new().parse(r"(?i)^abc$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("ABC".to_string()));
}

#[test]
fn alternation_should_succeed() {
    let hir = Parser::new().parse(r"0a|bcd$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("0a".to_string()));
}

#[test]
fn alternation_should_succeed_2() {
    let hir = Parser::new().parse(r"0a|bcd$").unwrap();
    let program = visit(&hir, ProgramFactory::default()).unwrap();
    let mut machine = Machine::new(program);
    assert!(machine.run("bcd".to_string()));
}
