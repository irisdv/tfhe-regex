use regex_syntax::hir::ClassUnicodeRange;
use tfhe::shortint::ClientKey;
use tfhe_regex::EncodedCipherTrait;

#[derive(Debug, Clone)]
pub struct IntervalCharOptions {
    pub range: Vec<ClassUnicodeRange>,
    pub can_repeat: bool,
    pub is_optional: bool,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Char(u8),
    Match,                 // Anchor end
    Start,                 // Anchor start
    Repetition(u8),   // 0 to infinite repetition of a character
    OptionalChar(u8), // in case of bounded repetitions or ZeroOrOneRepetition
    IntervalChar(IntervalCharOptions),
    Branch(usize), // context to fallback
    Jump(usize),
}

#[derive(Clone)]
pub struct CiphertextRange<T> {
    pub start: T,
    pub end: T,
}

#[derive(Clone)]
pub struct CipherIntervalCharOptions<T> {
    pub range: Vec<CiphertextRange<T>>,
    pub can_repeat: bool,
    pub is_optional: bool,
}

#[derive(Clone)]
pub enum CipherInstruction<T:EncodedCipherTrait+Clone> {
    CipherChar(T),
    Match, // Anchor end
    Start, // Anchor start
    CipherRepetition(T),
    CipherOptionalChar(T),
    CipherIntervalChar(CipherIntervalCharOptions<T>),
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
pub struct CipherProgramItem<T:EncodedCipherTrait+Clone> {
    pub instruction: CipherInstruction<T>,
    pub action: Action,
}

pub type CipherProgram<T> = Vec<CipherProgramItem<T>>;

fn cipher_program_item<T:EncodedCipherTrait+Clone>(client_key: &ClientKey, program_item: &ProgramItem) -> CipherProgramItem<T> {
    let instruction: CipherInstruction<T> = match program_item.instruction.clone() {
        Instruction::Char(c) => {
            let ct = T::encrypt(client_key, c);
            CipherInstruction::CipherChar(ct)
        }
        Instruction::Match => CipherInstruction::Match,
        Instruction::Start => CipherInstruction::Start,
        Instruction::Repetition(c) => {
            let ct = T::encrypt(client_key, c);
            CipherInstruction::CipherRepetition(ct)
        }
        Instruction::OptionalChar(c) => {
            let ct = T::encrypt(client_key, c);
            CipherInstruction::CipherOptionalChar(ct)
        }
        Instruction::IntervalChar(ranges) => {
            let cipher_ranges: Vec<CiphertextRange<T>> = ranges
                .range
                .iter()
                .map(|range| {
                    let start = range.start() as u8;
                    let start_ct = T::encrypt(client_key, start);
                    let end = range.end() as u8;
                    let end_ct = T::encrypt(client_key, end);
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

pub fn cipher_program<T:EncodedCipherTrait+Clone>(client_key: &ClientKey, program: Program) -> CipherProgram<T> {
    let cipher_program = program
        .iter()
        .map(|program_item| cipher_program_item(client_key, program_item))
        .collect();
    cipher_program
}
