use std::fmt::format;
use std::{env, io};
use std::fs::File;
use std::io::{Read, Write};
mod tokenization;
use crate::tokenization::Tokenizator;
mod parserization;
use parserization::Parser;
mod interpretation;
use crate::interpretation::{Machine, Value};

fn read_file(path: &str) -> Result<String, String> {
    match File::open(path) {
        Ok(mut file) => {
            let mut content = String::new();
            if let Ok(_) = file.read_to_string(&mut content){
                Ok(content)
            } else {
                Err(format!("НЕ УДАЛОСЬ ПРОЧИТАТЬ ФАЙЛ {}", path))
            }
        }
        Err(_) => Err(format!("НЕ УДАЛОСЬ ОТКРЫТЬ ФАЙЛ {}", path))
    }
}

fn proceed_program(input: String){
    let mut tokenize = Tokenizator::new(input);
    match tokenize.tokenize() {
        Ok(contents) => {
            // shall proceed
            let tokens = contents;
         //   for token in tokens.clone() {
       //         print!("{} ", token.clone().to_string());
         //   }
            println!();

            let mut parser = Parser::new(tokens);
            parser.parse();
            let mut machine = Machine::new(parser.instructions);
            machine.execute();
            let errored = Value::Integer(-1);
           // println!("РЕЗУЛЬТАТ: {}", machine.stack.last().unwrap_or(&errored).to_string());
            println!("РЕЗУЛЬТАТ: {}", machine.stack.last().unwrap_or(&errored).to_string());
        },
        // shall error
        Err(error) => println!("ОШИБКА: {}", error)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2{
        match read_file(&args[1]) {
            Ok(code) => proceed_program(code),
            Err(error) => println!("ОШИБКА: {}", error)
        }
    }

    loop{
        print!("> ");
        io::stdout().flush().expect("НЕ УДАЛОСЬ ДА КАК");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("НЕ УДАЛОСЬ ПРОЧИТАТЬ КОД");
        input = input.trim().to_string();
        proceed_program(input);
    }
}
