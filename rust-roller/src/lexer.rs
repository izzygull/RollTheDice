use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("d")]
    D,
    #[token("+")]
    PLUS,
    #[token("-")]
    HYPHEN,
    #[token("*")]
    STAR,
    #[token("/")]
    FSLASH,
    #[token("^")]
    CARROT,
    #[token("(")]
    LPAREN,
    #[token(")")]
    RPAREN,
    #[regex(r"([0-9]*\.[0-9]+|[0-9]+)", |lex| lex.slice().parse::<f64>().unwrap())]
    NUMBER(f64),
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"//.*\n", logos::skip)]
    ERROR
}


#[test]
fn print_roll() {
    let mut lex = Token::lexer("+-/.34*^()d1.523 1 4-3");
    while let Some(token) =  lex.next(){
        println!("{:?}", token)
    }
}