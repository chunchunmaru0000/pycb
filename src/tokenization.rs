#[derive(Clone, PartialEq)]
pub enum TokenType {
    EOF,
    DOT,
    SEMICOLON,
  //  QUOTE,

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
           // TokenType::QUOTE => "\"".to_string(),

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

#[derive(Clone)]
pub struct Location {
    pub line: usize,
    pub letter: usize
}

impl Location {
    pub fn new(l: usize, lett: usize) -> Location{
        Location{
            line: l,
            letter: lett
        }
    }

    pub fn to_string(&self) -> String{
        format!("<СТРОКА {}, СИМВОЛ {}>", self.line, self.letter)
    }
}

#[derive(Clone)]
pub struct Token {
    pub view : String,
    pub typed : TokenType,
    pub location : Location
}

impl Token {
    pub fn new(look: String, type_of: TokenType, loc: Location) -> Token {
        Token {
            view: look,
            typed: type_of,
            location: loc
        }
    }

    pub fn to_string(&self) -> String{
        if self.typed == TokenType::STRING{
            return  format!("<\"{}\", {}, {}>", self.view, self.typed.to_string(), self.location.to_string());
        }
        format!("<{}, {}, {}>", self.view, self.typed.to_string(), self.location.to_string())
    }
}

pub struct Tokenizator {
    code: String,
    line_of_code: usize,
    position: usize,
    start_line: usize,
    location: usize
}

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

        '\n' => false,
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
            line_of_code: 1usize,
            position: 0usize,
            start_line: 0usize,
            location: 0usize
        }
    }

    fn current(&self) -> char {
        if self.position < self.code.len() {
            return self.code.chars().nth(self.position).unwrap_or('\0');
        }
        '\0'
    }

    pub fn EOF (&self) -> Token {
        Token {
            view: String::new(),
            typed: TokenType::EOF,
            location: Location::new(self.line_of_code, self.position + 1)
        }
    }

    fn loc(&self) -> Location{
        Location::new(self.line_of_code, self.location)
    }

    fn next(&mut self) {
        self.position += 1;
        if self.current() == '\n'{
            self.next();
            self.start_line = self.position;
            self.line_of_code += 1;
        }
        self.location = self.position - self.start_line;
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
            return Ok(self.EOF());
        }

        while self.current() == ' ' {
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
                        _ => {}
                    }
                }
                else {
                    buffer.push(self.current());
                }
                self.next();
            }
            self.next(); // eat "
            return Ok(Token::new(buffer, TokenType::STRING, self.loc()));
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
            return Ok(
                Token::new(self.take_between(start, self.position), if dots == 0 { TokenType::INTEGER } else { TokenType::FLOAT }, self.loc()));
            //return Err("МНОГО ТОЧЕК ДЛЯ ЧИСЛА ТО ЧЕЛ".to_string());
        }

        if variableable(self.current()) {
            let start = self.position;
            while variableable(self.current()) {
                self.next();
            }
            let view: String = self.take_between(start, self.position);
            let typed: TokenType = worder(&view);
            return Ok(Token::new(view, typed, self.loc()));
        }

        let loc = self.loc();
        let ret = match self.current() {
            '+' => {
                self.next();
                Token::new("+".to_string(), TokenType::PLUS, loc)
            },
            '-' => {
                self.next();
                Token::new("-".to_string(), TokenType::MINUS, loc)
            },
            '*' => {
                self.next();
                Token::new("*".to_string(), TokenType::MUL, loc)
            },
            '/' => {
                self.next();
                Token::new("/".to_string(), TokenType::DIVISION, loc)
            },
            '.' => {
                self.next();
                Token::new(".".to_string(), TokenType::DOT, loc)
            },
            ';' => {
                self.next();
                Token::new(";".to_string(), TokenType::SEMICOLON, loc)
            },
            _ => {
                self.next();
                self.EOF()
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