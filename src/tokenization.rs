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

    fn take_between(&mut self, start: usize, end: usize) -> String {
        let mut text = String::new();
        for i in start..end {
            text.push(self.code.chars().nth(i).unwrap_or('\0').clone());
        }
        text
    }

    fn next_token(&mut self) -> Result<Token, String> {
        if self.current() == '\0' {
            return Ok(EOF);
        }

        while self.current() == ' ' || self.current() == '\n' {
            self.next();
        }

        if self.current() == '"' {
            let mut buffer = String::new();
            self.next(); // eat "
            while self.current() != '"' {

                if self.current() == '\0' {
                    return Err(format!("СТРОКА БЫЛА НЕ ЗАКОНЧЕНА, БУФФЕР БЫЛ: <\"{}\">", buffer));
                }
                if self.current() == '\\' {
                    self.next();
                    match self.current() {
                        'н' => buffer.push('\n'),
                        'т' => buffer.push('\t'),
                        '\\' => buffer.push('\\'),
                        _ => self.next() // схера эта параша съедаеть 3 символа после вообще как
                    }
                }
                else {
                    buffer.push(self.current());
                }
                self.next();
            }
            self.next(); // eat "
            return Ok(Token { view: buffer, typed: TokenType::STRING, place: self.position });
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

            return Ok(Token { view: self.take_between(start, self.position), typed: if dots == 0 { TokenType::INTEGER } else { TokenType::FLOAT }, place: self.position });
            //return Err("МНОГО ТОЧЕК ДЛЯ ЧИСЛА ТО ЧЕЛ".to_string());
        }

        if variableable(self.current()) {
            let start = self.position;
            while variableable(self.current()) {
                self.next();
            }
            let viewed: String = self.take_between(start, self.position);
            let typeded: TokenType = worder(&viewed);
            return Ok(Token { view: viewed, typed: typeded, place: self.position });
        }

        let ret = match self.current() {
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
        };
        Ok(ret)
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            match self.next_token(){
                Ok(token) => {
                    tokens.push(token.clone());
                    if token.typed == TokenType::EOF {
                        break;
                    }
                }
                Err(error) => return Err(error)
            }
        }
        Ok(tokens)
    }
}