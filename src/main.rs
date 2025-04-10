use std::collections::HashMap;

mod tokens;
mod lexer;
mod nodes;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Source file required.");
        return;
    }
    let mut lexer_: lexer::Lexer;
    if let Ok(file) = std::fs::read_to_string(&args[1]) {
        lexer_ = lexer::Lexer::new(file.chars().collect());
    }
    else {
        println!("Cannot read the file.");
        return;
    }
    if let Err((l, p)) = lexer_.scan() {
        println!("Lexer scanning failed at line {} position {}", l, p);
        return;
    }
    // println!("{:#?}", lexer_.scanned);
    let mut parser_ = parser::Parser::new(lexer_.get_scanned_vec(), lexer_.get_final_pos());
    if let Err((l, p)) = parser_.parse() {
        println!("Parser parsing failed at line {} position {}", l, p);
    }
    // println!("{:#?}", parser_.parsed);
    let statements = parser_.parsed;
    let mut variables = HashMap::new();
    for s in &statements {
        if let Err(e) = s.execute(&mut variables) {
            println!("{}", e);
            return;
        }
    }
}
