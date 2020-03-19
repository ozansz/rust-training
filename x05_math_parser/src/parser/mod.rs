use std::fmt;
use std::boxed::Box;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum LexemeKind {
    NUMBER,
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    Nil
}

impl fmt::Display for LexemeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            LexemeKind::PLUS => String::from("PLUS"),
            LexemeKind::MINUS => String::from("MINUS"),
            LexemeKind::MUL => String::from("MUL"),
            LexemeKind::DIV => String::from("DIV"),
            LexemeKind::LPAREN => String::from("LPAREN"),
            LexemeKind::RPAREN => String::from("RPAREN"),
            LexemeKind::NUMBER => String::from("NUMBER"),
            LexemeKind::Nil => String::from("<Nil>")
        };

        write!(f, "{}", repr.as_str())
    }
}

#[derive(Clone, Copy)]
pub struct Lexeme {
    pub integral_value: u64,
    pub kind: LexemeKind
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
            LexemeKind::NUMBER => format!("NUMBER {{value: {}}}", self.integral_value),
            LexemeKind::Nil => String::from("<Nil>")
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
                            Some(val) => val as u64,
                            _ => {
                                panic!("Inconvertible digit(?): '{0}' at position {1}.", ch, char_index);
                            }
                        }, kind: LexemeKind::NUMBER});
                    } else {
                        self.lexemes.push(last_elem);
                        self.lexemes.push(Lexeme{integral_value: match ch.to_digit(10) {
                            Some(val) => val as u64,
                            _ => {
                                panic!("Inconvertible digit(?): '{0}' at position {1}.", ch, char_index);
                            }
                        }, kind: LexemeKind::NUMBER});
                    }
                } else {
                    self.lexemes.push(Lexeme{integral_value: match ch.to_digit(10) {
                        Some(val) => val as u64,
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

#[derive(Eq, PartialEq)]
pub enum ASTNodeKind {
    NUMBER,
    ADD,
    SUB,
    MUL,
    DIV
}

pub struct ASTNode {
    pub integral_value: i128,
    pub kind: ASTNodeKind,
    pub left: Option<Box<ASTNode>>,
    pub right: Option<Box<ASTNode>>
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match self.kind {
            ASTNodeKind::NUMBER => format!("NUMBER {{value: {}}}", self.integral_value),
            ASTNodeKind::ADD => String::from("BINOP(+)"),
            ASTNodeKind::SUB => String::from("BINOP(-)"),
            ASTNodeKind::MUL => String::from("BINOP(*)"),
            ASTNodeKind::DIV => String::from("BINOP(/)"),
        };

        write!(f, "{}", result.as_str())
    }
}

impl ASTNode {
    pub fn dump(&self) {
        self.dump_recursive(0);
    }

    fn dump_recursive(&self, level: i32) {
        for _i in 0..level {
            print!("  ");
        }

        println!("{}", self);
        
        match &self.left {
            Some(val) => val.dump_recursive(level + 1),
            _ => ()
        }

        match &self.right {
            Some(val) => val.dump_recursive(level + 1),
            _ => ()
        }
    }
}

pub struct Parser {
    pub lexer: Lexer,
    pub curr_lexeme: Lexeme,
    pub curr_index: usize
}

fn astnode_kind_from_lexeme_type(kind: LexemeKind) -> ASTNodeKind {
    match kind {
        LexemeKind::NUMBER => ASTNodeKind::NUMBER,
        LexemeKind::PLUS => ASTNodeKind::ADD,
        LexemeKind::MINUS => ASTNodeKind::SUB,
        LexemeKind::MUL => ASTNodeKind::MUL,
        LexemeKind::DIV => ASTNodeKind::DIV,
        _ => {
            panic!("Unexpected conversion from lexeme type: {}.", kind);
        }
    }
}

impl Parser {
    pub fn parse(&mut self) -> ASTNode {
        self.curr_index = 0;
        self.lexer.lexemes.push(Lexeme{integral_value: 0, kind: LexemeKind::Nil});

        self.curr_lexeme = self.lexer.lexemes[self.curr_index];

        return self.expr();
    }

    fn eat(&mut self, lexeme_kind: LexemeKind) {
        if self.curr_lexeme.kind == lexeme_kind {
            self.curr_index += 1;
            self.curr_lexeme = self.lexer.lexemes[self.curr_index];
        } else {
            panic!("Got unexpected token type: {}", lexeme_kind);
        }
    }

    fn factor(&mut self) -> ASTNode {
        let token = self.curr_lexeme;

        if token.kind == LexemeKind::NUMBER {
            self.eat(LexemeKind::NUMBER);
            return ASTNode{integral_value: token.integral_value as i128,
                kind: ASTNodeKind::NUMBER, left: None, right: None};
        } else if token.kind == LexemeKind::LPAREN {
            self.eat(LexemeKind::LPAREN);
            let node = self.expr();
            self.eat(LexemeKind::RPAREN);
            return node;
        } else {
            panic!("Unexpected token: {}, was expecting NUMBER or LPAREN.", self.curr_lexeme);
        }
    }

    fn term(&mut self) -> ASTNode {
        let mut node = self.factor();

        while (self.curr_lexeme.kind == LexemeKind::MUL) || (self.curr_lexeme.kind == LexemeKind::DIV) {
            let token = self.curr_lexeme;

            if token.kind == LexemeKind::MUL {
                self.eat(LexemeKind::MUL);
            }

            if token.kind == LexemeKind::DIV {
                self.eat(LexemeKind::DIV);
            }

            node = ASTNode{
                integral_value: 0,
                kind: astnode_kind_from_lexeme_type(token.kind),
                left: Some(Box::new(node)), right: Some(Box::new(self.factor()))
            };
        }

        return node;
    }

    fn expr(&mut self) -> ASTNode {
        let mut node = self.term();

        while (self.curr_lexeme.kind == LexemeKind::PLUS) || (self.curr_lexeme.kind == LexemeKind::MINUS) {
            let token = self.curr_lexeme;

            if token.kind == LexemeKind::PLUS {
                self.eat(LexemeKind::PLUS);
            }

            if token.kind == LexemeKind::MINUS {
                self.eat(LexemeKind::MINUS);
            }

            node = ASTNode{
                integral_value: 0,
                kind: astnode_kind_from_lexeme_type(token.kind),
                left: Some(Box::new(node)), right: Some(Box::new(self.term()))
            };
        }

        return node;
    }
}