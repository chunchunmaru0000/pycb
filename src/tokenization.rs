use std::io;

#[derive(Clone, PartialEq)]
pub enum TokenType {
    EOF,
    DOT,
    SEMICOLON,
    QUOTE,

    INTEGER,
    FLOAT,
    STRING,
    VARIABLE,

    RETURN,

    PLUS,
    MINUS,
    MUL,
    DIVISION,
}

impl TokenType {
    pub fn to_string(&self) -> String {
        match self {
            TokenType::EOF => "КОНЕЦ ФАЙЛА".to_string(),
            TokenType::DOT => ".".to_string(),
            TokenType::SEMICOLON => ";".to_string(),
            TokenType::QUOTE => "\"".to_string(),

            TokenType::INTEGER => "ЧИСЛО".to_string(),
            TokenType::FLOAT => "ТОЧКА".to_string(),
            TokenType::STRING => "СТРОКА".to_string(),
            TokenType::VARIABLE => "ПЕРЕМЕННАЯ".to_string(),

            TokenType::RETURN => "ВОЗДАТЬ".to_string(),

            TokenType::PLUS => "+".to_string(),
            TokenType::MINUS => "-".to_string(),
            TokenType::MUL => "*".to_string(),
            TokenType::DIVISION => "/".to_string(),
            //_ => "НЕ СУЩЕСТВУЮЩИЙ ТОКЕН ПОЛАГАЮ".to_string()
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Token {
    view : String,
    typed : TokenType,
    place : usize
}

impl Token {
    pub fn to_string(&self) -> String{
        if self.typed == TokenType::STRING{
            return  format!("<\"{}\", {}, {}>", self.view, self.typed.to_string(), self.place);
        }
        format!("<{}, {}, {}>", self.view, self.typed.to_string(), self.place)
    }
}

pub struct Tokenizator {
    code: String,
    position: usize,
}

pub const EOF : Token = Token {
    view: String::new(),
    typed: TokenType::EOF,
    place: 0usize
};

pub fn variableable(c: char) -> bool {
    match c {
        '.' => false,
        ';' => false,
        ',' => false,
        '"' => false,
        '+' => false,
        '-' => false,
        '*' => false,
        '/' => false,
        ' ' => false,

        '\0' => false,
        _ => true
    }
}

pub fn worder(view: &str) -> TokenType{
    match view {
        "воздать" => TokenType::RETURN,
        _ => TokenType::VARIABLE
    }
}

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

        while self.current() == ' ' || self.current() == '\n' {
            self.next();
        }

        if self.current() == '"' {
            let mut buffer = String::new();
            self.next(); // eat "
            while self.current() != '"' {
                self.next();
                if self.current() == '\0' {
                    panic!("СТРОКА БЫЛА НЕ ЗАКОНЧЕННА, БУФФЕР БЫЛ: <{}>", buffer);
                }
                if self.current() == '\\' {
                    self.next();
                    match self.current() {
                        'н' => buffer.push('\n'),
                        'т' => buffer.push('\t'),
                        '\\' => buffer.push('\\'),
                        _ => self.next()
                    }
                }
                else {
                    buffer.push(self.current());
                }
            }
            self.next(); // eat "
            return Token { view: buffer, typed: TokenType::STRING, place: self.position };
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
                return Token { view: self.code[start..self.position].to_string(), typed: TokenType::INTEGER, place: self.position };
            }
            if dots == 1 {
                return Token { view: self.code[start..self.position].to_string(), typed: TokenType::FLOAT, place: self.position };
            }
            panic!("МНОГО ТОЧЕК ДЛЯ ЧИСЛА ТО ЧЕЛ");
        }

        if variableable(self.current()) {
            let start = self.position;
            while variableable(self.current()) {
                self.next();
            }
            let viewed: String = self.code[start..self.position].to_string();
            let typeded: TokenType = worder(&viewed);
            return Token { view: viewed, typed: typeded, place: self.position };
        }

        match self.current() {
            '+' => {
                self.next();
                Token { view: String::from("+"), typed: TokenType::PLUS, place: self.position - 1 }
            },
            '-' => {
                self.next();
                Token { view: String::from("-"), typed: TokenType::MINUS, place: self.position - 1 }
            },
            '*' => {
                self.next();
                Token { view: String::from("*"), typed: TokenType::MUL, place: self.position - 1 }
            },
            '/' => {
                self.next();
                Token { view: String::from("/"), typed: TokenType::DIVISION, place: self.position - 1 }
            },
            '.' => {
                self.next();
                Token { view: String::from("."), typed: TokenType::DOT, place: self.position - 1 }
            },
            ';' => {
                self.next();
                Token { view: String::from(";"), typed: TokenType::SEMICOLON, place: self.position - 1 }
            },
            _ => {
                self.next();
                EOF
            } //panic!("{}", format!("НЕИЗВЕСТНЫЙ СИМВОЛ {}", self.current()))
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, io::Error> {
        let mut tokens = Vec::new();
        loop {
            match self.next_token(){
                Ok(token) => {
                    tokens.push(token);
                    if token.typed == TokenType::EOF {
                        break;
                    }
                }
                Err(error) => return Err(error)
            }
            //let token = self.next_token();

        }
        Ok(tokens)
    }
}