// Define the possible nodes in an AST of the roll
// Maybe this would be more extensible in the form of:
// BINARY_OPERATOR(OP,A,B) rather than having unique cases for each operator
#[derive(Debug)]
pub enum ASTNode {
    // Child nodes are (num rolls, num faces)
    ROLL(Box<ASTNode>,Box<ASTNode>),
    PLUS(Box<ASTNode>,Box<ASTNode>),
    MINUS(Box<ASTNode>,Box<ASTNode>),
    NEGATE(Box<ASTNode>),
    MULTIPLY(Box<ASTNode>, Box<ASTNode>),
    DIVIDE(Box<ASTNode>, Box<ASTNode>),
    // Child nodes are base, power
    RAISE(Box<ASTNode>, Box<ASTNode>),
    VALUE(Value),
}

#[derive(Debug)]
pub enum Value {
    NUMBER(f64),
    ERROR(String)
}