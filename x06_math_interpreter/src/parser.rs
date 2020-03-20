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
    pub fn lex(&mut self, source: &str) -> Option<String> {
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
                            return Some(String::from("Could not get the last value from lexeme vector."));
                        }
                    };

                    if last_elem.kind == LexemeKind::NUMBER {
                        self.lexemes.push(Lexeme{integral_value: last_elem.integral_value * 10 + match ch.to_digit(10) {
                            Some(val) => val as u64,
                            _ => {
                                return Some(format!("Inconvertible digit(?): '{0}' at position {1}.", ch, char_index));
                            }
                        }, kind: LexemeKind::NUMBER});
                    } else {
                        self.lexemes.push(last_elem);
                        self.lexemes.push(Lexeme{integral_value: match ch.to_digit(10) {
                            Some(val) => val as u64,
                            _ => {
                                return Some(format!("Inconvertible digit(?): '{0}' at position {1}.", ch, char_index));
                            }
                        }, kind: LexemeKind::NUMBER});
                    }
                } else {
                    self.lexemes.push(Lexeme{integral_value: match ch.to_digit(10) {
                        Some(val) => val as u64,
                        _ => {
                            return Some(format!("Inconvertible digit(?): '{0}' at position {1}.", ch, char_index));
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
                        return Some(format!("Unexpected char: '{0}' at position {1}.", ch, char_index));
                    }
                }

                self.lexemes.push(Lexeme{integral_value: 0, kind: lexeme_kind});
            }
        }

        return None;
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
    pub fn parse(&mut self) -> Result<ASTNode, String> {
        self.curr_index = 0;
        self.lexer.lexemes.push(Lexeme{integral_value: 0, kind: LexemeKind::Nil});

        self.curr_lexeme = self.lexer.lexemes[self.curr_index];

        return self.expr();
    }

    fn eat(&mut self, lexeme_kind: LexemeKind) -> Option<String> {
        if self.curr_lexeme.kind == lexeme_kind {
            self.curr_index += 1;
            self.curr_lexeme = self.lexer.lexemes[self.curr_index];
            None
        } else {
            Some(format!("Got unexpected token type: {}", lexeme_kind))
        }
    }

    fn factor(&mut self) -> Result<ASTNode, String>  {
        let token = self.curr_lexeme;

        if token.kind == LexemeKind::NUMBER {
            match self.eat(LexemeKind::NUMBER) {
                Some(e) => return Err(e),
                None => ()
            }

            return Ok(ASTNode{integral_value: token.integral_value as i128,
                kind: ASTNodeKind::NUMBER, left: None, right: None});
        } else if token.kind == LexemeKind::LPAREN {
            match self.eat(LexemeKind::LPAREN) {
                Some(e) => return Err(e),
                None => ()
            }

            let node = match self.expr() {
                Ok(val) => val,
                Err(e) => return Err(e)
            };

            match self.eat(LexemeKind::RPAREN) {
                Some(e) => return Err(e),
                None => ()
            }
            return Ok(node);
        } else {
            Err(format!("Unexpected token: {}, was expecting NUMBER or LPAREN.", self.curr_lexeme))
        }
    }

    fn term(&mut self) -> Result<ASTNode, String>  {
        let mut node = match self.factor() {
            Ok(val) => val,
            Err(s) => return Err(s)
        };

        while (self.curr_lexeme.kind == LexemeKind::MUL) || (self.curr_lexeme.kind == LexemeKind::DIV) {
            let token = self.curr_lexeme;

            if token.kind == LexemeKind::MUL {
                match self.eat(LexemeKind::MUL) {
                    Some(e) => return Err(e),
                    None => ()
                }
            }

            if token.kind == LexemeKind::DIV {
                match self.eat(LexemeKind::DIV) {
                    Some(e) => return Err(e),
                    None => ()
                }
            }

            node = ASTNode{
                integral_value: 0,
                kind: astnode_kind_from_lexeme_type(token.kind),
                left: Some(Box::new(node)), right: Some(Box::new(match self.factor() {
                    Ok(val) => val,
                    Err(s) => return Err(s)
                }))
            };
        }

        return Ok(node);
    }

    fn expr(&mut self) -> Result<ASTNode, String> {
        let mut node = match self.term() {
            Ok(val) => val,
            Err(s) => return Err(s)
        };

        while (self.curr_lexeme.kind == LexemeKind::PLUS) || (self.curr_lexeme.kind == LexemeKind::MINUS) {
            let token = self.curr_lexeme;

            if token.kind == LexemeKind::PLUS {
                match self.eat(LexemeKind::PLUS) {
                    Some(e) => return Err(e),
                    None => ()
                }
            }

            if token.kind == LexemeKind::MINUS {
                match self.eat(LexemeKind::MINUS) {
                    Some(e) => return Err(e),
                    None => ()
                }
            }

            node = ASTNode{
                integral_value: 0,
                kind: astnode_kind_from_lexeme_type(token.kind),
                left: Some(Box::new(node)), right: Some(Box::new(match self.term() {
                    Ok(val) => val,
                    Err(e) => return Err(e)
                }))
            };
        }

        return Ok(node);
    }
}