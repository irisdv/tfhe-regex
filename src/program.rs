use regex_syntax::hir::ClassUnicodeRange;
use tfhe::shortint::{ciphertext::Ciphertext, ClientKey};

#[derive(Debug, Clone)]
pub struct IntervalCharOptions {
    pub range: Vec<ClassUnicodeRange>,
    pub can_repeat: bool,
    pub is_optional: bool,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Char(u8),
    Match,            // Anchor end
    Start,            // Anchor start
    Repetition(u8),   // 0 to infinite repetition of a character
    OptionalChar(u8), // in case of bounded repetitions or ZeroOrOneRepetition
    IntervalChar(IntervalCharOptions),
    Branch(usize), // context to fallback
    Jump(usize),
}

#[derive(Clone)]
pub struct CiphertextRange {
    pub start: Ciphertext,
    pub end: Ciphertext,
}

#[derive(Clone)]
pub enum CipherInstruction {
    CipherChar(Ciphertext),
    Match, // Anchor end
    Start, // Anchor start
    CipherRepetition(Ciphertext),
    CipherOptionalChar(Ciphertext),
    CipherIntervalChar(Vec<CiphertextRange>),
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

#[derive(Clone)]
pub struct CipherProgramItem {
    pub instruction: CipherInstruction,
    pub action: Action,
}

pub type CipherProgram = Vec<CipherProgramItem>;

fn cipher_program_item(client_key: &ClientKey, program_item: &ProgramItem) -> CipherProgramItem {
    let instruction: CipherInstruction = match program_item.instruction.clone() {
        Instruction::Char(c) => {
            let ct = client_key.encrypt(c as u64);
            CipherInstruction::CipherChar(ct)
        }
        Instruction::Match => CipherInstruction::Match,
        Instruction::Start => CipherInstruction::Start,
        Instruction::Repetition(c) => {
            let ct = client_key.encrypt(c as u64);
            CipherInstruction::CipherRepetition(ct)
        }
        Instruction::OptionalChar(c) => {
            let ct = client_key.encrypt(c as u64);
            CipherInstruction::CipherOptionalChar(ct)
        }
        Instruction::IntervalChar(ranges) => {
            let cipher_ranges = ranges
                .range
                .iter()
                .map(|range| {
                    let start = client_key.encrypt(range.start() as u64);
                    let end = client_key.encrypt(range.end() as u64);
                    CiphertextRange {
                        start: start,
                        end: end,
                    }
                })
                .collect();
            CipherInstruction::CipherIntervalChar(cipher_ranges)
        }
        Instruction::Branch(pc) => CipherInstruction::Branch(pc),
        Instruction::Jump(pc) => CipherInstruction::Jump(pc),
    };
    CipherProgramItem {
        instruction: instruction,
        action: program_item.action.clone(),
    }
}

pub fn cipher_program(client_key: &ClientKey, program: Program) -> CipherProgram {
    let cipher_program = program
        .iter()
        .map(|program_item| cipher_program_item(client_key, program_item))
        .collect();
    cipher_program
}
