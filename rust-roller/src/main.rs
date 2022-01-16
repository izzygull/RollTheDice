mod lexer;
mod parser;
mod utils;
mod ast_structures;
mod interpreter;

use logos::Logos;
use std::io;

use crate::lexer::Token;

fn main() {
    println!("Welcome to the dice roller, version 0.🦀.0!");
    let mut buffer = String::new();
    let stdin = io::stdin(); 
    loop {
        let res = stdin.read_line(&mut buffer);
        match res {
            Ok(_ipt_bytes) => (),
            Err(error) => println!("Error: {}", error)
        }
        let tokens = lexer::Token::lexer(&buffer).collect::<Vec<lexer::Token>>();
        if tokens.contains(&Token::ERROR) {
            println!("Unrecognized character");
            continue;
        }
        buffer.clear();
        let ast = parser::build_ast(&tokens);
        let val = interpreter::compute_result(ast);
        match val {
            ast_structures::Value::NUMBER(result) => println!("{}", result),
            ast_structures::Value::ERROR(s) => println!("{}", s)
        }
    }
}
