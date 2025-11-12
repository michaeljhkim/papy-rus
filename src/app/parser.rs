use super::tokenizer::{Token};

#[derive(Debug)]
pub enum Expr {
    INTEGER(i32),
    ADD(Box<Expr>, Box<Expr>),
    GROUP(Box<Expr>),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { 
            tokens, 
            pos: 0 
        }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> &Token {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        self.tokens.get(self.pos - 1).unwrap_or(&Token::EOF)
    }

    pub fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_term();

        while matches!(self.peek(), Token::PLUS) {
            self.advance();     // consume '+'
            let right = self.parse_term();
            left = Expr::ADD(Box::new(left), Box::new(right));
        }

        return left;
    }

    fn parse_term(&mut self) -> Expr {
        match self.advance() {
            Token::INTEGER(n) => Expr::INTEGER(*n),
            Token::LPAREN => {
                let expr = self.parse_expression(); // parse until there 

                match self.advance() {
                    Token::RPAREN => Expr::GROUP(Box::new(expr)),
                    _ => panic!("Expected ')'"),
                }
            }
            _ => panic!("Expected number or '('"),
        }
    }

    pub fn eval(&mut self, expr: &Expr) -> i32 {
        match expr {
            Expr::INTEGER(n) => *n,
            Expr::ADD(a, b) => self.eval(a) + self.eval(b),
            Expr::GROUP(inner) => self.eval(inner),
        }
    }
}
