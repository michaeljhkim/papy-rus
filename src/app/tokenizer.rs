#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    INTEGER(i32),      // e.g., 42, 3.14
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

pub fn tokenize(source_str: String) -> Vec<Token> {
    let mut token_list: Vec<Token> = vec![];
    let mut source_iter = source_str.chars().peekable();
    
    let mut num_str = String::new();

    while let Some(c) = source_iter.next() {
        match c {
            '0'..='9' => {
                num_str.push_str(&c.to_string());
                let next_char: &char = source_iter.peek().unwrap_or(&' ');

                if !next_char.is_digit(10) && !num_str.is_empty() {
                    let i = num_str.parse::<i32>().unwrap();
                    token_list.push(Token::INTEGER(i));
                    num_str = "".to_string();
                }
            },
            '+' => {
                token_list.push(Token::PLUS)
            },
            '(' => {
                token_list.push(Token::LPAREN)
            },
            ')' => {
                token_list.push(Token::RPAREN)
            },
            ' ' | '\t' | '\n' => {
                continue
            },
            _ => panic!("Unexpected character: {}", c)
        }
    }
    
    return token_list;
}
