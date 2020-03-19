use std::fmt;

#[derive(Eq, PartialEq)]
enum LexemeKind {
    NUMBER,
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN
}

pub struct Lexeme {
    integral_value: u128,
    kind: LexemeKind
}

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self.kind {
            LexemeKind::PLUS => String::from("PLUS"),
            LexemeKind::MINUS => String::from("MINUS"),
            LexemeKind::MUL => String::from("MUL"),
            LexemeKind::DIV => String::from("DIV"),
            LexemeKind::LPAREN => format!("LPAREN {{depth: {}}}", self.integral_value),
            LexemeKind::RPAREN => format!("RPAREN {{depth: {}}}", self.integral_value),
            LexemeKind::NUMBER => format!("NUMBER {{value: {}}}", self.integral_value)
        };

        write!(f, "{}", repr.as_str())
    }
}

pub struct Lexer {
    pub lexemes: std::vec::Vec<Lexeme>
}

impl Lexer {
    pub fn lex(&mut self, source: &str) {
        let mut paren_depth = 0;

        for (char_index, ch) in source.chars().enumerate() {
            if ch.is_whitespace() {
                continue;
            }

            if ch.is_digit(10) {
                if self.lexemes.len() > 0 {
                    let last_elem : Lexeme = match self.lexemes.pop() {
                        Some(val) => val,
                        _ => {
                            panic!("Could not get the last value from lexeme vector.");
                        }
                    };

                    if last_elem.kind == LexemeKind::NUMBER {
                        self.lexemes.push(Lexeme{integral_value: last_elem.integral_value * 10 + match ch.to_digit(10) {
                            Some(val) => val as u128,
                            _ => {
                                panic!("Inconvertible digit(?): '{0}' at position {1}.", ch, char_index);
                            }
                        }, kind: LexemeKind::NUMBER});
                    } else {
                        self.lexemes.push(last_elem);
                        self.lexemes.push(Lexeme{integral_value: match ch.to_digit(10) {
                            Some(val) => val as u128,
                            _ => {
                                panic!("Inconvertible digit(?): '{0}' at position {1}.", ch, char_index);
                            }
                        }, kind: LexemeKind::NUMBER});
                    }
                } else {
                    self.lexemes.push(Lexeme{integral_value: match ch.to_digit(10) {
                        Some(val) => val as u128,
                        _ => {
                            panic!("Inconvertible digit(?): '{0}' at position {1}.", ch, char_index);
                        }
                    }, kind: LexemeKind::NUMBER});
                }
            } else if ch == '(' {
                self.lexemes.push(Lexeme{integral_value: paren_depth, kind: LexemeKind::LPAREN});
                paren_depth += 1;
            } else if ch == ')' {
                paren_depth -= 1;
                self.lexemes.push(Lexeme{integral_value: paren_depth, kind: LexemeKind::RPAREN});
            } else {
                let lexeme_kind : LexemeKind;

                match ch {
                    '+' => lexeme_kind = LexemeKind::PLUS,
                    '-' => lexeme_kind = LexemeKind::MINUS,
                    '*' => lexeme_kind = LexemeKind::MUL,
                    '/' => lexeme_kind = LexemeKind::DIV,
                    _ => {
                        panic!("Unexpected char: '{0}' at position {1}.", ch, char_index);
                    }
                }

                self.lexemes.push(Lexeme{integral_value: 0, kind: lexeme_kind});
            }
        }
    }
}

impl fmt::Display for Lexer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result : String = String::from("");

        for i in 0..self.lexemes.len() {
            result = format!("{}\n{}", result, self.lexemes[i]);
        }

        write!(f, "{}", result)
    }
}