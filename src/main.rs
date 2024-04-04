use std::io;
use std::io::{Write};
mod tokenization;
use tokenization::Tokenizator;

fn main() {
    loop{
        // tokenization
        print!("> ");
        io::stdout().flush().expect("НЕ УДАЛОСЬ ДА");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("НЕ УДАЛОСЬ ПРОЧИТАТЬ КОД");
        input = input.trim().to_string();
        let mut tokenize = Tokenizator::new(input);
        match tokenize.tokenize() {
            Ok(contents) => {
                // shall proceed
                let tokens = contents;
                for token in tokens {
                    print!("{} ", token.clone().to_string());
                }
                println!();
            },
            Err(error) => println!("ОШИБКА: {}", error)
        }
    }
}
