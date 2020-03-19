mod lexer;

fn main() {
    let mut lex_machine = lexer::Lexer{lexemes: std::vec::Vec::new()};
    let mut input_string : String = String::new();

    println!("Enter expression: ");

    std::io::stdin().read_line(&mut input_string)
        .expect("Unable to read line");

    lex_machine.lex(input_string.as_str());
    println!("{}", lex_machine);
}
