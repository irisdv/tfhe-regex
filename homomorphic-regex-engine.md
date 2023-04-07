---
title: "Building an homomorphic regex engine using TFHE-rs"
thumbnail:
authors:
  - user:
    guest: true
---

# Building an homomorphic regex engine using TFHE-rs

Regular expressions, commonly referred to as regex, are a powerful tool for searching and manipulating text. However, this process requires having access to unencrypted text, which, in certain cases, can cause privacy concerns.

Homomorphic encryption is a type of encryption that allows computations to be performed on encrypted data without requiring decryption first. This means that data can be kept private even while it's being used in computations.

Combining homomorphic encryption and regex engines opens up new possibilities for secure text search and manipulation. For example, a homomorphic regex engine could be used to search for patterns in encrypted data without revealing their contents.

This tutorial uses the [TFHE-rs library](https://github.com/zama-ai/tfhe-rs) library, a pure Rust implementation of the Fully Homomorphic Encryption over the Torus (TFHE) scheme for boolean and integers FHE arithmetics. TFHE-rs is based on several crates, for our engine we'll use the shortint crate which allow homomorphic computation over short integers whose size do not exceed 4 bits.

The tutorial covers :

- how to build a finite state machine to analyze regular expressions
- how to setup and use the shortint TFHE-rs library
- ...

## Setup the environment

First we'll start by creating a new Rust project called `tfhe-regex` by running:

```
cargo new tfhe-regex
```

For this project we'll use two packages:

- [TFHE-rs library shortint crate](https://github.com/zama-ai/tfhe-rs)
- [regex-syntax crate](https://docs.rs/regex-syntax) : a regular expression parser.

To install `TFHE-rs` you can check the [Getting started](https://github.com/zama-ai/tfhe-rs#getting-started) section of the lib to add the right dependency depending on your operating system.

At first we'll focus on developing a simple implementation of the regex engine, and then we'll transform it so it can operate on encrypted data using tfhe-rs.

## Building a Regex Engine

A regex engine works by first parsing a regular expression into a finite state machine, which is a mathematical model used to describe the behavior of a system. The finite state machine then processes the input text, character by character, by moving through the various states of the machine.

For example, consider the regular expression "ab+c". This regex matches any string that starts with "a", followed by one or more "b"s, and ends with "c". The finite state machine for this regex would look something like this:

```
    a      b        c
-> (q0) -> (q1) -> (q2) ->
```

The arrow denotes a transition from one state to another, and the labels on the transitions indicate which character triggers that transition.

To match a string against this regex, the regex engine would start at the initial state, and then for each character in the input string, it would follow the appropriate transition to the next state. If the engine reaches the final state after processing the entire input string, then the string is a match. Otherwise, it's not a match.

### Implement a simple state machine

We'll start by implementating a simple state machine for a simple test case: `r("abc")` which should return true if `abc` is present in a string.

We'll have a compiler which will convert the regular expression into a program, a sequence of instructions, that will be executed when we compare a given input string.

Let's declare an Instruction enum. This enum defines two instructions: `Char` and `Match`. The `Char` instruction is used to match a character in the input string, while the `Match` instruction is used to signify the end of the input string.

```
#[derive(Debug)]
pub enum Instruction {
    Char(u8),
    Match,
}
```

Next, we have the `Program` type, which is just a vector of `Instruction`. This is the program that the machine will execute.

```
pub type Program = Vec<Instruction>;
```

Let's start by creating a `ProgramFactory` struct. It's a visitor that generates the program from the high-level intermediate representation (HIR) of the regex. It is implemented using the Visitor trait from the `regex_syntax` crate. It keeps track of the program it is generating as it walks the HIR, and returns the final program when it is done.

```
#[derive(Default)]
pub struct ProgramFactory {
    program: Program,
}
```

The `visit_pre` function is called for each node in the HIR. In this implementation, it only handles literals and empty nodes. For literals, it generates a `Char` instruction for each character in the literal. For empty nodes, it generates a Match instruction. When all nodes have been run, it calls the `finish` function and returns the `Program`.

```
impl Visitor for ProgramFactory {
    type Err = ();
    type Output = Vec<Instruction>;

    fn visit_pre(&mut self, hir: &Hir) -> Result<(), Self::Err> {
        match hir.kind() {
            HirKind::Literal(literal) => match literal {
                Literal::Unicode(c) => {
                    self.program.push(Instruction::Char(*c as u8));
                }
                Literal::Byte(b) => {
                    self.program.push(Instruction::Char(*b));
                }
            },
            HirKind::Empty => {
                self.program.push(Instruction::Match);
            }
            _ => todo!(),
        }
        Ok(())
    }

    fn finish(self) -> Result<Self::Output, Self::Err> {
        Ok(self.program)
    }
}
```

Now that we have built the Program let's create a `Machine` struct that represents the state machine that will execute the program. It keeps track of the `program_counter` (where we are in the program), the `string_counter` (where we are in the string), and the content of the program itself that is to say a list of instructions.

```
pub struct Machine {
    program_counter: usize,
    string_counter: usize,
    program: Program,
}
```

Let's move to the implementation. We need to implement a function that will run the machine on a given input string. It loops through the program, executing each instruction until it reaches the end of the program.

```
impl Machine {
    pub fn new(program: Program) -> Self {
        Self {
            program_counter: 0,
            string_counter: 0,
            program,
        }
    }

    pub fn run(&mut self, input: String) -> bool {
        while self.program_counter < self.program.len() {
            let instruction = self.program.get(self.program_counter).unwrap();
            self.program_counter += 1;
            match instruction {
                Instruction::Char(c) => {
                    if self.string_counter >= input.len() {
                        return false;
                    }
                    let result = input.as_bytes()[self.string_counter] == *c;
                    self.string_counter += 1;
                    if !result {
                        return false;
                    }
                }
                Instruction::Match => {}
            }
        }
        true
    }
}
```

We have a first implementation. Let's test it. Create a main.rs file. We first use the `Parser` imported from the `regex_syntax` crate to parse the regular expression "abc" into a HIR of the regular expression syntax. We then use the `visit` function to convert the HIR into a program that can be executed by the virtual machine. Finally we call the `run` method of the machine with the input string we'd like to compare, here "abcc". It should return a boolean that indicates whether the input string matches the regular expression.

```
use machine::Machine;
use regex_syntax::hir::visit;
use regex_syntax::Parser;

use crate::machine::ProgramFactory;

mod machine;


fn main() {
  let hir = Parser::new().parse(r"abc").unwrap();
  let program = visit(&hir, ProgramFactory::default()).unwrap();
  let mut machine = Machine::new(program);
  println!("{:?}", machine.run("abcc".to_string()));
}
```

Now that we have a first implementation of a state machine for a simple test case, let's expand it so it can handle more features.

#### A more complex Regex state machine

We started using two simple Instructions : `Char` and `Match`.

To have a full implementation of a regex engine, we will need additional instructions.

- Char: a character that can be repeated or optional.
- IntervalChar: a range of characters that can be repeated or optional
- Start: instruction to specify if the beginning of the string should match exactly
- Match: instruction to specify that we can stop the program if it has matched exactly
- Branch: in case of alternative matching, specify multiple possible paths that the program will test one after the other
- Jump: to set the program counter to a new value. It will be used in case of alternations

Each instruction will maintain an action struct where `next` represent the next state of the program to go to and `offset` .... In case we need to jump to another instruction or go back in the strings those two variables will help us achieve this.

```
pub struct Action {
    pub next: usize,
    pub offset: i32,
}
```

....

You can check the full implementation of all of these instructions [here](https://#).

## Homomorphic Regex Engine
