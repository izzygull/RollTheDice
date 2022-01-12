use logos::Logos;

use crate::lexer::Token;

pub fn check_brackets(tokens_vec: &Vec<Token>) -> bool {
    let mut stack: Vec<&Token> = Vec::new();
    for token in tokens_vec {
        match token {
            Token::LPAREN => {stack.push(token);},
            Token::RPAREN => {match stack.pop() {
                    Some(Token::LPAREN) => (),
                    _ => return false
                }
            },
            _ => ()
        }
    }
    stack.is_empty()
}

#[test]
fn test_empty() {
    let lex = Token::lexer("");
    let tokens_vec = lex.collect::<Vec<Token>>();
    assert!(check_brackets(&tokens_vec));
}

#[test]
fn test_one_pair() {
    let lex = Token::lexer("()");
    let tokens_vec = lex.collect::<Vec<Token>>();
    assert!(check_brackets(&tokens_vec));
}

#[test]
fn test_two_pair() {
    let lex = Token::lexer("()()");
    let tokens_vec = lex.collect::<Vec<Token>>();
    assert!(check_brackets(&tokens_vec));
}

#[test]
fn test_nest() {
    let lex = Token::lexer("(())");
    let tokens_vec = lex.collect::<Vec<Token>>();
    assert!(check_brackets(&tokens_vec));
}

#[test]
fn test_too_closed() {
    let lex = Token::lexer("())(()");
    let tokens_vec = lex.collect::<Vec<Token>>();
    assert!(!check_brackets(&tokens_vec));
}

#[test]
fn test_too_opened() {
    let lex = Token::lexer("((()())");
    let tokens_vec = lex.collect::<Vec<Token>>();
    assert!(!check_brackets(&tokens_vec));
}

#[test]
fn test_complex() {
    let lex = Token::lexer("(dd)23+34(*/(+-+)23.34(+-34*)^-^)234");
    let tokens_vec = lex.collect::<Vec<Token>>();
    assert!(check_brackets(&tokens_vec));
}