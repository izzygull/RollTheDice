use crate::{ast_structures::{ASTNode,Value}, utils::create_binary_op_error};
use rand;
use std::cmp::max;


pub fn compute_result(root: ASTNode) -> Value {
    match root {
        ASTNode::ROLL(rolls, faces) => {
            let rolls = compute_result(*rolls);
            let faces = compute_result(*faces);
            if let (&Value::NUMBER(num_rolls), &Value::NUMBER(num_faces)) = (&rolls, &faces) {
                let mut roll: f64;
                // Want NUMBER to be f64 for greater expressiveness but roles should be ints, hence ugly conversions...
                let num_rolls = num_rolls as i64;
                let num_faces = num_faces as i64;
                let effective_faces = max(num_faces, -num_faces);
                let mut accum: f64 = 0.;
                for _ in 0..max(num_rolls,-num_rolls) {
                    roll = rand::random::<f64>();
                    accum += (((roll * effective_faces as f64) as i64) +1) as f64 ;
                }
                if (num_rolls * num_faces) < 0 {
                    accum *= -1.0;
                }
                return Value::NUMBER(accum);
            } else {
                return Value::ERROR(create_binary_op_error(&rolls, &faces));
            }
        },
        ASTNode::PLUS(a, b) => {
            let a = compute_result(*a);
            let b = compute_result(*b);
            if let (&Value::NUMBER(num_a), &Value::NUMBER(num_b)) = (&a,&b) {
                return Value::NUMBER(num_a + num_b);
            } else {
                return Value::ERROR(create_binary_op_error(&a, &b));
            }
        },
        ASTNode::MINUS(a, b) => {
            let a = compute_result(*a);
            let b = compute_result(*b);
            if let (&Value::NUMBER(num_a), &Value::NUMBER(num_b)) = (&a,&b) {
                return Value::NUMBER(num_a - num_b);
            } else {
                return Value::ERROR(create_binary_op_error(&a, &b));
            }
        },
        ASTNode::NEGATE(a) => {
            let a = compute_result(*a);
            match a {
                Value::NUMBER(num_a) => {
                    return Value::NUMBER(-num_a);
                },
                Value::ERROR(_) => {
                    return a; 
                }
            }
        },
        ASTNode::MULTIPLY(a, b) => {
            let a = compute_result(*a);
            let b = compute_result(*b);
            if let (&Value::NUMBER(num_a), &Value::NUMBER(num_b)) = (&a,&b) {
                return Value::NUMBER(num_a * num_b);
            } else {
                return Value::ERROR(create_binary_op_error(&a, &b));
            }
        },
        ASTNode::DIVIDE(a, b) => {
            let a = compute_result(*a);
            let b = compute_result(*b);
            if let (&Value::NUMBER(num_a), &Value::NUMBER(num_b)) = (&a,&b) {
                if num_b == 0.0 {
                    return Value::ERROR("Division by zero".to_string());
                }
                return Value::NUMBER(num_a / num_b);
            } else {
                return Value::ERROR(create_binary_op_error(&a, &b));
            }
        },
        ASTNode::RAISE(a, b) => {
            let a = compute_result(*a);
            let b = compute_result(*b);
            if let (&Value::NUMBER(num_a), &Value::NUMBER(num_b)) = (&a,&b) {
                return Value::NUMBER(num_a.powf(num_b));
            } else {
                return Value::ERROR(create_binary_op_error(&a, &b));
            }
        },
        ASTNode::VALUE(value) => {return value;},
        // _ => {return Value::ERROR;}
    }
}