mod parser;

use std::io::{self, Write};
use std::panic;

fn ast_visitor(root: &parser::ASTNode) -> f64 {
    if root.kind == parser::ASTNodeKind::NUMBER {
        root.integral_value as f64
    } else {
        let lhs = match &(*root).left {
            Some(expr) => expr,
            None => panic!("Empty lhs on binary operator: {}", *root),
        };

        let rhs = match &(*root).right {
            Some(expr) => expr,
            None => panic!("Empty rhs on binary operator: {}", *root),
        };

        match &root.kind {
            parser::ASTNodeKind::ADD => ast_visitor(&*lhs) + ast_visitor(&*rhs),
            parser::ASTNodeKind::SUB => ast_visitor(&*lhs) - ast_visitor(&*rhs),
            parser::ASTNodeKind::MUL => ast_visitor(&*lhs) * ast_visitor(&*rhs),
            parser::ASTNodeKind::DIV => ast_visitor(&*lhs) / ast_visitor(&*rhs),
            _ => panic!("Illegal code location!")
        }
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
        let mut lex_machine = parser::Lexer{lexemes: std::vec::Vec::new()};

        print!("\nIn [{}]: ", io_index);
        io::stdout().flush().unwrap();
    
        io::stdin().read_line(&mut input_string)
            .expect("Unable to read line");

        if input_string.contains("exit") {
            break;
        }
    
        match lex_machine.lex(input_string.as_str()) {
            Some(e) => {
                println!("LexerError: {}", e);
                io_index += 1;
                continue;
            },
            None => ()
        }
    
        let mut parser = parser::Parser{
            lexer: lex_machine,
            curr_lexeme: parser::Lexeme{
                integral_value: 0,
                kind: parser::LexemeKind::Nil
            },
            curr_index: 0
        };

        let ast = match parser.parse() {
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