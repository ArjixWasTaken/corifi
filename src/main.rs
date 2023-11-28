use std::env;

mod lexer;
mod parser;
mod optimizer;
mod compiler;

fn main() {
    let input = env::args().collect::<Vec<_>>();
    if input.len() < 2 {
        println!("arg[1] is missing");
        return;
    }
    let _ = lexer::lex(input.iter().nth(1).unwrap());
}
