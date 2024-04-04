use crate::tokenization::{EOF, Token};

#[derive(Clone, PartialEq)]
pub enum Instruction {
    PUSH,
    POP,

    PLUS,
    MINUS,
    MUL,
    DIVISION
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokenы: Vec<Token>) -> Parser {
        Parser {
            tokens: tokenы,
            position: 0
        }
    }

    pub fn current(&mut self) -> &Token {
        if self.position < self.tokens.len(){
            return &self.tokens[self.position];
        }
        &EOF
    }

    pub fn parse(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        while true {
            let cur = self.current();

        }
        instructions
    }
}