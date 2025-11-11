#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    NUMBER(f64),      // e.g., 42, 3.14
    IDENT(String),    // optional, for variables like x, y

    // Operators
    PLUS,             // +
    MINUS,            // -
    MUL,              // *
    DIV,              // /

    // Parentheses
    LPAREN,           // (
    RPAREN,           // )

    // Assignment (optional, if you want variables)
    ASSIGN,           // =

    // End of input
    EOF,
}

/*
pub trait Tokenizer {
    fn tokenize(&self) -> Vec<Token>;
}
*/

fn tokenize(source_str: String) {
    let mut token_list: Vec<Token>;
    
    let mut num_str = String::new();

    for c in source_str.chars() {

    }

}
