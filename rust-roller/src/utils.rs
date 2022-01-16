use std::collections::HashMap;

use logos::Logos;

use crate::{lexer::Token, ast_structures::Value};

// Combine error values to create a meaningful error
pub fn create_binary_op_error(val_1:&Value, val_2:&Value) -> String {
    let mut ret = "".to_string();
    match val_1 {
        Value::ERROR(s) => {
            ret.push_str(&s);
        },
        _ => ()
    }
    match val_2 {
        Value::ERROR(s) => {
            if ret.len() > 0 {
                ret.push_str(" and ");
            }
            ret.push_str(&s);
        },
        _ => ()
    }
    ret
}

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

// Function to get a map from each open parenthesis to each close parenthesis.
// As currently written, not extensible to other 
pub fn get_parens_map(tokens_vec: &Vec<Token>) -> Result<HashMap<usize,usize>, ()> {
    let mut parens_map = HashMap::<usize,usize>::new();
    let mut stack = Vec::<usize>::new(); 
    for (i, token) in tokens_vec.iter().enumerate() {
        match token {
            Token::LPAREN => stack.push(i),
            Token::RPAREN => {
                let open_ind_or_none = stack.pop();
                match open_ind_or_none {
                    Some(open_ind) => {parens_map.insert(open_ind, i);},
                    None => return Err(())
                }
            },
            _ => ()
        }
    }
    if stack.is_empty() {
        return Result::Ok(parens_map);
    }
    Result::Err(())
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