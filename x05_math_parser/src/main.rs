mod parser;

fn main() {
    let mut lex_machine = parser::Lexer{lexemes: std::vec::Vec::new()};

    let mut input_string : String = String::new();

    println!("Enter expression: ");

    std::io::stdin().read_line(&mut input_string)
        .expect("Unable to read line");

    lex_machine.lex(input_string.as_str());

    let mut parser = parser::Parser{lexer: lex_machine, curr_lexeme: parser::Lexeme{integral_value: 0, kind: parser::LexemeKind::Nil}, curr_index: 0};
    let ast = parser.parse();

    ast.dump();
}
