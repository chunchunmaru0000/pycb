use crate::tokenization::{Token, TokenType};

#[derive(Clone, PartialEq)]
pub enum Instruction {

    PushStr(String),
    PopStr(String),

    PushInt(i64),
    PopInt(i64),

    PushFloat(f64),
    PopFloat(f64),

    Plus,
    Minus,
    Mul,
    Division
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    pub instructions: Vec<Instruction>
}

impl Parser {
    pub fn new(tokenы: Vec<Token>) -> Parser {
        Parser {
            tokens: tokenы,
            position: 0,
            instructions: Vec::<Instruction>::new()
        }
    }

    pub fn current(&mut self) -> &Token {
        if self.position < self.tokens.len(){
            return &self.tokens[self.position];
        }
        &self.tokens.last().unwrap()
    }

    pub fn primary(&mut self) -> Instruction {
        let ret: Instruction;
        match self.current().typed {
            TokenType::INTEGER => {
                ret = Instruction::PushInt(self.current().view.parse::<i64>().unwrap());
                //self.instructions.push(Instruction::PushInt(self.current().view.parse::<i64>().unwrap()));
                self.position += 1;
            },
            TokenType::STRING => {
                ret = Instruction::PushStr(self.current().view.clone());
                //self.instructions.push(Instruction::PushStr(self.current().view.clone()));
                self.position += 1;
            },
            TokenType::FLOAT => {
                ret = Instruction::PushFloat(self.current().view.parse::<f64>().unwrap());
               // self.instructions.push(Instruction::PushFloat(self.current().view.parse::<f64>().unwrap()));
                self.position += 1;
            },
            _ => unreachable!()
        }
        ret
    }

    pub fn mul_and_these(&mut self) -> Instruction {
        let res = self.primary();

        res
    }

    pub fn plus_minus(&mut self) {
        let mut res = self.primary();
        self.instructions.push(res);
        while self.current().typed == TokenType::PLUS {
            self.position += 1;
            res = self.primary();
            self.instructions.push(res);
            self.instructions.push(Instruction::Plus);
        }
    }

    pub fn parse(&mut self) {
        loop {
            match self.current().typed {
                TokenType::INTEGER => self.plus_minus(),
                TokenType::FLOAT => self.plus_minus(),
                TokenType::STRING => self.plus_minus(),
                _ => break
            }
            // match self.current().typed {
            //     TokenType::PLUS => {
            //         instructions.push(Instruction::Plus);
            //         self.position += 1;
            //     },
            //     TokenType::MINUS => {
            //         instructions.push(Instruction::Minus);
            //         self.position += 1;
            //     },
            //     TokenType::MUL => {
            //         instructions.push(Instruction::Mul);
            //         self.position += 1;
            //     },
            //     TokenType::DIVISION => {
            //         instructions.push(Instruction::Division);
            //         self.position += 1;
            //     },
            //
            //     TokenType::INTEGER => {
            //         instructions.push(Instruction::PushInt(self.current().view.parse::<i64>().unwrap()));
            //         self.position += 1;
            //     },
            //     TokenType::STRING => {
            //         instructions.push(Instruction::PushStr(self.current().view.clone()));
            //         self.position += 1;
            //     },
            //     TokenType::FLOAT => {
            //         instructions.push(Instruction::PushFloat(self.current().view.parse::<f64>().unwrap()));
            //         self.position += 1;
            //     },
            //
            //     _ => break
            // }
        }
    }
}