use std::collections::HashMap;

use crate::lexer::Token;
use crate::ast_structures::{ASTNode,Value};
use crate::utils::get_parens_map;

fn get_token_precedence(token: &Token) -> i64 {
    match token {
        Token::PLUS | Token::HYPHEN => return 500,
        Token::STAR | Token::FSLASH => return 400,
        Token::CARROT => return 300,
        Token::D => return 100,
        Token::NUMBER(_) => return 0,
        _ => return -1
    }
}

fn build_ast_rec(tokens: &Vec<Token>, parens_map: &HashMap<usize,usize>, left:usize, right:usize) -> ASTNode {
    if right - left < 1 {
        return ASTNode::VALUE(Value::ERROR("Empty or less than empty expr".to_string()));
    } else if right - left == 1 {
        match tokens[left] {
            Token::NUMBER(num) => {
                return  ASTNode::VALUE(Value::NUMBER(num));
            },
            _ => {
                return ASTNode::VALUE(Value::ERROR(format!("Token at ind {} is its own expr, but not a value!", left)));
            }
        }
    } else {
        // Handle case where expression is wholly parenthesized
        if let Token::LPAREN = &tokens[left] {
            if *parens_map.get(&left).unwrap() == right-1 {
                return build_ast_rec(tokens, parens_map, left+1, right-1);
            }
        }
        let mut max_ind:usize = 0;
        let mut max_precedence: i64 = -1;
        // need to check for stuff in parens
        // while left and right are matching parens, move left and right in by one... (a)+(b)
        // Find token with max precedence and min paren depth
        let mut i = left;
        while i < right {
            let token = &tokens[i];
            match token {
                Token::LPAREN => {
                    i = *parens_map.get(&i).unwrap()+1;
                    continue;
                },
                Token::RPAREN => {
                    return ASTNode::VALUE(Value::ERROR(format!("I somehow tried to parse a right paren at ind {}", i)));
                },
                _ => {
                    if let Token::HYPHEN = token {
                        if i == left {
                            max_precedence = 1;
                            max_ind = i;
                        } else if let Token:: RPAREN | Token::NUMBER(_) = tokens[i-1] {
                            let precedence = get_token_precedence(&token);
                            if precedence >= max_precedence {
                                max_ind = i;
                                max_precedence = precedence;
                            }
                        }
                    }
                    else {
                        let precedence = get_token_precedence(&token);
                        if precedence >= max_precedence {
                            max_ind = i;
                            max_precedence = precedence;
                        }
                    }
                }
            }
            i += 1;
        }
        if max_precedence == -1 {
            return ASTNode::VALUE(Value::ERROR("Couldn't find an operator out of parens".to_string()));
        }
        match tokens[max_ind] {
            Token::HYPHEN => {
                if max_ind == left {
                    return ASTNode::MINUS(Box::<ASTNode>::new(ASTNode::VALUE(Value::NUMBER(0.))), Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, left+1, right)));
                } else {
                    return ASTNode::MINUS(Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, left, max_ind)), Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, max_ind+1, right)));
                }
            },
            Token::PLUS => {
                return ASTNode::PLUS(Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, left, max_ind)), Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, max_ind+1, right)));
            },
            Token::STAR => {
                return ASTNode::MULTIPLY(Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, left, max_ind)), Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, max_ind+1, right)));
            },
            Token::FSLASH => {
                return ASTNode::DIVIDE(Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, left, max_ind)), Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, max_ind+1, right)));
            },
            Token::D => {
                while max_ind > left {
                    if let Token::D = tokens[max_ind -1] {
                        max_ind -= 1;
                    }
                    else {
                        break;
                    }
                }
                if max_ind == left {
                    return ASTNode::ROLL(Box::<ASTNode>::new(ASTNode::VALUE(Value::NUMBER(1.))), Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, left+1, right)));
                } else {
                    return ASTNode::ROLL(Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, left, max_ind)), Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, max_ind+1, right)));
                }
            },
            Token::CARROT => {
                return ASTNode::RAISE(Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, left, max_ind)), Box::<ASTNode>::new(build_ast_rec(tokens, parens_map, max_ind+1, right)));
            },
            _ => {
                return ASTNode::VALUE(Value::ERROR("Something else went wrong".to_string()));
            }
        }
    }
}

pub fn build_ast(tokens:&Vec<Token>) -> ASTNode {
    let parens_map_or_err = get_parens_map(tokens);
    match parens_map_or_err {
        Ok(parens_map) => {
            return build_ast_rec(tokens, &parens_map, 0, tokens.len());
        },
        Err(()) => {
            return ASTNode::VALUE(Value::ERROR("Unbalanced parens".to_string()));
        }
    }
}

