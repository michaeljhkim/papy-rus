use super::tokenizer::{Token};

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    ADD,
    SUB,
    MUL,
    DIV,
    // Later you could add more: Pow, Mod, etc.
}

/*
impl Token {
    fn to_binary_op(&self) -> Option<BinaryOp> {
        match self {
            Token::PLUS => Some(BinaryOp::ADD),
            Token::MINUS => Some(BinaryOp::SUB),
            Token::STAR => Some(BinaryOp::MUL),
            Token::SLASH => Some(BinaryOp::DIV),
            _ => None,
        }
    }
}
*/

#[derive(Debug)]
pub enum Expr {
    INTEGER(i32),
    BINARY {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
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
        self.parse_addition()
    }

    // addition and subtraction
    fn parse_addition(&mut self) -> Expr {
        let mut left = self.parse_multiplication();
        while matches!(self.peek(), Token::PLUS | Token::MINUS) {
            let op = match self.advance() {
                Token::PLUS => BinaryOp::ADD,
                Token::MINUS => BinaryOp::SUB,
                _ => unreachable!(),
            };
            let right = self.parse_multiplication();
            left = Expr::BINARY { left: Box::new(left), op, right: Box::new(right) };
        }
        left
    }
    
    // multiplication and division
    fn parse_multiplication(&mut self) -> Expr {
        let mut left = self.parse_unary();
        while matches!(self.peek(), Token::STAR | Token::SLASH) {
            let op = match self.advance() {
                Token::STAR => BinaryOp::MUL,
                Token::SLASH => BinaryOp::DIV,
                _ => unreachable!(),
            };
            let right = self.parse_unary();
            left = Expr::BINARY { left: Box::new(left), op, right: Box::new(right) };
        }
        left
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek() {
            Token::PLUS => { 
                self.advance(); 
                self.parse_unary() 
            },
            Token::MINUS => { 
                self.advance(); 
                Expr::BINARY {
                    left: Box::new(Expr::INTEGER(0)),
                    op: BinaryOp::SUB,
                    right: Box::new(self.parse_unary())
                }
            },
            _ => self.parse_term()
        }
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
            Expr::BINARY { left, op, right } => {
                let l = self.eval(left);
                let r = self.eval(right);
                match op {
                    BinaryOp::ADD => l + r,
                    BinaryOp::SUB => l - r,
                    BinaryOp::MUL => l * r,
                    // be careful about division by zero
                    BinaryOp::DIV => l / r,
                }
            },
            Expr::GROUP(inner) => self.eval(inner),
        }
    }
}
