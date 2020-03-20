mod parser;

use std::io::{self, Write};
use std::panic;

fn ast_visitor(root: &parser::ASTNode) -> f64 {
    match root.node {
        parser::Lexeme::Number(val) => val as f64,
        parser::Lexeme::Plus => ast_visitor(match &(*root).left {
            Some(expr) => expr,
            None => panic!("Empty lhs on binary operator: {}", *root),
        }) + ast_visitor(match &(*root).right {
            Some(expr) => expr,
            None => panic!("Empty rhs on binary operator: {}", *root),
        }),
        parser::Lexeme::Minus => ast_visitor(match &(*root).left {
            Some(expr) => expr,
            None => panic!("Empty lhs on binary operator: {}", *root),
        }) - ast_visitor(match &(*root).right {
            Some(expr) => expr,
            None => panic!("Empty rhs on binary operator: {}", *root),
        }),
        parser::Lexeme::Mul => ast_visitor(match &(*root).left {
            Some(expr) => expr,
            None => panic!("Empty lhs on binary operator: {}", *root),
        }) * ast_visitor(match &(*root).right {
            Some(expr) => expr,
            None => panic!("Empty rhs on binary operator: {}", *root),
        }),
        parser::Lexeme::Div => ast_visitor(match &(*root).left {
            Some(expr) => expr,
            None => panic!("Empty lhs on binary operator: {}", *root),
        }) / ast_visitor(match &(*root).right {
            Some(expr) => expr,
            None => panic!("Empty rhs on binary operator: {}", *root),
        }),
        _ => panic!("Illegal code location!")
    }
}

fn main() {
    let mut io_index : u32 = 1;

    println!("[i] Sazak's Basic Math Interpreter");
    println!("[i] Available operations:");
    println!("      -> expr   : term ((PLUS | MINUS) term)*");
    println!("      -> term   : factor ((MUL | DIV) factor)*");
    println!("      -> factor : INTEGER | LPAREN expr RPAREN");
    println!("\n[+] Enter 'exit' to exit the program");

    loop {
        let mut input_string : String = String::new();

        print!("\nIn [{}]: ", io_index);
        io::stdout().flush().unwrap();
    
        io::stdin().read_line(&mut input_string)
            .expect("Unable to read line");

        if input_string.contains("exit") {
            break;
        }

        let lexemes;

        match parser::Lexer::lex(input_string.as_str()) {
            Ok(val) => lexemes = val,
            Err(e) => {
                println!("LexerError: {}", e);
                io_index += 1;
                continue;
            }
        };
    
        let mut _parser = parser::Parser::new(lexemes);

        let ast = match _parser.parse() {
            Ok(val) => val,
            Err(e) => {
                println!("ParserError: {}", e);
                io_index += 1;
                continue;
            }
        };
    
        println!("Out[{}]: {}", io_index, ast_visitor(&ast));
        io_index += 1;
    }
}