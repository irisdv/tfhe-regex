use regex_syntax::hir::ClassUnicodeRange;

#[derive(Debug, Clone)]
pub enum Instruction {
    Char(u8),
    Match,            // Anchor end
    Start,            // Anchor start
    Repetition(u8),   // 0 to infinite repetition of a character
    OptionalChar(u8), // in case of bounded repetitions or ZeroOrOneRepetition
    IntervalChar(Vec<ClassUnicodeRange>),
    Branch(usize), // context to fallback
    Jump(usize),
}

#[derive(Default, Clone, Debug)]
pub struct Action {
    pub next: usize,
    pub offset: i32,
}

#[derive(Debug, Clone)]
pub struct ProgramItem {
    pub instruction: Instruction,
    pub action: Action,
}

pub type Program = Vec<ProgramItem>;
