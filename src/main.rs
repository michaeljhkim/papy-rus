mod app;
use app::tokenizer::{tokenize};
use app::parser::{Parser};

fn main() {
    let test_source: String = "1 + (5 - 10) * 2".to_string();
    let tokenized = tokenize(test_source);
    println!("token_list: {:?}", tokenized);

    let mut parser_instance = Parser::new(tokenized);
    let mut ast = parser_instance.parse_expression();
    println!("AST: {:#?}", ast);

    println!("EVALUATED: {:?}", parser_instance.eval(&ast));
}
