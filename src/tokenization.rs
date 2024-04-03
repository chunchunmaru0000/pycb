use std::cmp::PartialEq;

pub enum TokenType {
    EOF,

    INT32,

    DOTED32,

    PLUS,
    MINUS,
    MUL,
    DIVISION,
}

impl TokenType {
    pub fn to_string(&self) -> String {
        match self {
            TokenType::EOF => "КОНЕЦ ФАЙЛА".to_string(),

            TokenType::INT32 => "ЧИСЛО32".to_string(),
            TokenType::DOTED32 => "ТОЧКА32".to_string(),

            TokenType::PLUS => "+".to_string(),
            TokenType::MINUS => "-".to_string(),
            TokenType::MUL => "*".to_string(),
            TokenType::DIVISION => "/".to_string(),

            //_ => "НЕ СУЩЕСТВУЮЩИЙ ТОКЕН ПОЛАГАЮ".to_string()
        }
    }

    pub fn clone(&self) -> TokenType {
        match self {
            TokenType::EOF => TokenType::EOF,

            TokenType::INT32 => TokenType::INT32,
            TokenType::DOTED32 => TokenType::DOTED32,

            TokenType::PLUS => TokenType::PLUS,
            TokenType::MINUS => TokenType::MINUS,
            TokenType::MUL => TokenType::MUL,
            TokenType::DIVISION => TokenType::DIVISION,

            //_ => panic!()
        }
    }
}

pub struct Token {
    view : String,
    typed : TokenType,
    place : usize
}

impl Token {
    pub fn to_string(&self) -> String{
        format!("<{}, {}, {}>", self.view, self.typed.to_string(), self.place)
    }

    pub fn clone (&self) -> Token {
        Token {
            view: self.view.clone(),
            typed: self.typed.clone(),
            place: self.place.clone()
        }
    }
}

pub struct Tokenizator {
    code: String,
    position: usize,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TokenType::EOF, TokenType::EOF) => true,
            (TokenType::INT32, TokenType::INT32) => true,
            (TokenType::DOTED32, TokenType::DOTED32) => true,
            (TokenType::PLUS, TokenType::PLUS) => true,
            (TokenType::MINUS, TokenType::MINUS) => true,
            (TokenType::MUL, TokenType::MUL) => true,
            (TokenType::DIVISION, TokenType::DIVISION) => true,
            _ => false
        }
    }
}

pub const EOF : Token = Token {
    view: String::new(),
    typed: TokenType::EOF,
    place: 0usize
};

impl Tokenizator {
    pub fn new(code_from: String) -> Tokenizator {
        Tokenizator {
            code: code_from,
            position : 0usize,
        }
    }

    fn current(&mut self) -> char {
        if self.position < self.code.len() {
            return self.code.chars().nth(self.position).unwrap_or('?');
        }
        '\0'
    }

    fn next(&mut self) {
        self.position += 1
    }

    fn next_token(&mut self) -> Token {
        if self.current() == '\0' {
            return EOF;
        }
        while self.current() == ' ' {
            self.next();
        }
        if self.current().is_digit(10){
            let start = self.position;
            let mut dots = 0u8;

            while self.current().is_digit(10) || self.current() == '.' {
                if self.current() == '.' {
                    dots += 1;
                }
                if dots > 1 {
                    dots -= 1;
                    break;
                }
                self.next();
            }

            if dots == 0 {
                return Token {
                    view: self.code[start..self.position].to_string(),
                    typed: TokenType::INT32,
                    place: self.position
                };
            }
            if dots == 1 {
                return Token {
                    view: self.code[start..self.position].to_string(),
                    typed: TokenType::DOTED32,
                    place: self.position
                };
            }
            panic!("МНОГО ТОЧЕК ДЛЯ ЧИСЛА ТО ЧЕЛ");
        }
        match self.current() {
            '+' => {
                self.next();
                Token {
                    view: String::from("+"),
                    typed: TokenType::PLUS,
                    place: self.position - 1
                }
            },
            '-' => {
                self.next();
                Token {
                    view: String::from("-"),
                    typed: TokenType::MINUS,
                    place: self.position - 1
                }
            },
            '*' => {
                self.next();
                Token {
                    view: String::from("*"),
                    typed: TokenType::MUL,
                    place: self.position - 1
                }
            },
            '/' => {
                self.next();
                Token {
                    view: String::from("/"),
                    typed: TokenType::DIVISION,
                    place: self.position - 1
                }
            },
            _ => {
                self.next();
                EOF
            } //panic!("{}", format!("НЕИЗВЕСТНЫЙ СИМВОЛ {}", self.current()))
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            tokens.push(token.clone());
            if token.typed == TokenType::EOF {
                break;
            }
        }
        tokens
    }
}