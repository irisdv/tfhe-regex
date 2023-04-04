use regex_syntax::hir::ClassUnicodeRange;
use tfhe::shortint::{ciphertext::Ciphertext, ClientKey};
use tfhe_regex::convert_char;

#[derive(Debug, Clone)]
pub struct IntervalCharOptions {
    pub range: Vec<ClassUnicodeRange>,
    pub can_repeat: bool,
    pub is_optional: bool,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Char([u8; 2]),
    Match,                 // Anchor end
    Start,                 // Anchor start
    Repetition([u8; 2]),   // 0 to infinite repetition of a character
    OptionalChar([u8; 2]), // in case of bounded repetitions or ZeroOrOneRepetition
    IntervalChar(IntervalCharOptions),
    Branch(usize), // context to fallback
    Jump(usize),
}

#[derive(Clone)]
pub struct CiphertextRange {
    pub start: [Ciphertext; 2],
    pub end: [Ciphertext; 2],
}

#[derive(Clone)]
pub struct CipherIntervalCharOptions {
    pub range: Vec<CiphertextRange>,
    pub can_repeat: bool,
    pub is_optional: bool,
}

#[derive(Clone)]
pub enum CipherInstruction {
    CipherChar([Ciphertext; 2]),
    Match, // Anchor end
    Start, // Anchor start
    CipherRepetition([Ciphertext; 2]),
    CipherOptionalChar([Ciphertext; 2]),
    CipherIntervalChar(CipherIntervalCharOptions),
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
            let ct: [Ciphertext; 2] = [
                client_key.encrypt(c[0] as u64),
                client_key.encrypt(c[1] as u64),
            ];
            CipherInstruction::CipherChar(ct)
        }
        Instruction::Match => CipherInstruction::Match,
        Instruction::Start => CipherInstruction::Start,
        Instruction::Repetition(c) => {
            let ct: [Ciphertext; 2] = [
                client_key.encrypt(c[0] as u64),
                client_key.encrypt(c[1] as u64),
            ];
            CipherInstruction::CipherRepetition(ct)
        }
        Instruction::OptionalChar(c) => {
            let ct: [Ciphertext; 2] = [
                client_key.encrypt(c[0] as u64),
                client_key.encrypt(c[1] as u64),
            ];
            CipherInstruction::CipherOptionalChar(ct)
        }
        Instruction::IntervalChar(ranges) => {
            let cipher_ranges: Vec<CiphertextRange> = ranges
                .range
                .iter()
                .map(|range| {
                    let start = convert_char(range.start() as u8);
                    let start_ct = [
                        client_key.encrypt(start[0] as u64),
                        client_key.encrypt(start[1] as u64),
                    ];
                    let end = convert_char(range.end() as u8);
                    let end_ct = [
                        client_key.encrypt(end[0] as u64),
                        client_key.encrypt(end[1] as u64),
                    ];
                    CiphertextRange {
                        start: start_ct,
                        end: end_ct,
                    }
                })
                .collect();
            CipherInstruction::CipherIntervalChar(CipherIntervalCharOptions {
                range: cipher_ranges,
                can_repeat: ranges.can_repeat,
                is_optional: ranges.is_optional,
            })
        }
        Instruction::Branch(pc) => CipherInstruction::Branch(pc),
        Instruction::Jump(pc) => CipherInstruction::Jump(pc),
    };
    CipherProgramItem {
        instruction,
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
