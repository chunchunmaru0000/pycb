use std::io;
use std::io::Write;
mod tokenization;
use tokenization::Tokenizator;

fn main() {
    loop{
        print!("> ");
        io::stdout().flush().expect("НЕ УДАЛОСЬ ДА");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("НЕ УДАЛОСЬ ПРОЧИТАТЬ КОД");
        let mut tokenizator = Tokenizator::new(input);
        let tokens = tokenizator.tokenize();
        for token in tokens {
            print!("{} ", token.to_string());
        }
        println!();
    }
}
