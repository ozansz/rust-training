use std::fmt;
use std::vec;
use std::boxed::Box;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Lexeme {
    Number(i128),
    Plus,
    Minus,
    Mul,
    Div,
    Lparen(u32),
    Rparen(u32),
    Nil
}

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lexeme::Number(val) => write!(f, "Number({})", val),
            Lexeme::Plus => write!(f, "Plus(+)"),
            Lexeme::Minus => write!(f, "Minus(-)"),
            Lexeme::Mul => write!(f, "Mul(*)"),
            Lexeme::Div => write!(f, "Div(/)"),
            Lexeme::Lparen(level) => write!(f, "Lparen({})", level),
            Lexeme::Rparen(level) => write!(f, "Rparen({})", level),
            Lexeme::Nil => write!(f, "<Nil>",)
        }
    }
}

pub struct Lexer {}

impl Lexer {
    pub fn lex(source: &str) -> Result<Vec<Lexeme>, String> {
        let mut lexemes = vec::Vec::new();
        let mut paren_depth = 0;

        for (char_index, ch) in source.chars().enumerate() {
            if ch.is_whitespace() {
                continue;
            }

            if ch.is_digit(10) {
                if lexemes.len() > 0 {
                    let last_elem : Lexeme = match lexemes.pop() {
                        Some(val) => val,
                        _ => {
                            return Err(String::from("Could not get the last value from lexeme vector."));
                        }
                    };

                    match last_elem {
                        Lexeme::Number(val) => lexemes.push(Lexeme::Number(val * 10 + match ch.to_digit(10) {
                            Some(val) => val as i128,
                            _ => {
                                return Err(format!("Inconvertible digit(?): '{0}' at position {1}.", ch, char_index));
                            }}
                        )),
                        _ => {
                            lexemes.push(last_elem);
                            lexemes.push(Lexeme::Number(match ch.to_digit(10) {
                                Some(val) => val as i128,
                                _ => {
                                    return Err(format!("Inconvertible digit(?): '{0}' at position {1}.", ch, char_index));
                                }
                            }));
                        }
                    }
                } else {
                    lexemes.push(Lexeme::Number(match ch.to_digit(10) {
                        Some(val) => val as i128,
                        _ => {
                            return Err(format!("Inconvertible digit(?): '{0}' at position {1}.", ch, char_index));
                        }
                    }));
                }
            } else if ch == '(' {
                lexemes.push(Lexeme::Lparen(paren_depth));
                paren_depth += 1;
            } else if ch == ')' {
                paren_depth -= 1;
                lexemes.push(Lexeme::Rparen(paren_depth));
            } else {
                match ch {
                    '+' => lexemes.push(Lexeme::Plus),
                    '-' => lexemes.push(Lexeme::Minus),
                    '*' => lexemes.push(Lexeme::Mul),
                    '/' => lexemes.push(Lexeme::Div),
                    _ => {
                        return Err(format!("Unexpected char: '{0}' at position {1}.", ch, char_index));
                    }
                }
            }
        }

        return Ok(lexemes);
    }
}

pub struct ASTNode {
    pub node: Lexeme,
    pub left: Option<Box<ASTNode>>,
    pub right: Option<Box<ASTNode>>
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match write!(f, "{} (", self.node) {
            Ok(_) => (),
            Err(e) => return Err(e)
        };

        match &(*self).left {
            Some(val) => print!("{}", val),
            None => print!("(), ")
        };

        match &(*self).left {
            Some(val) => print!("{}", val),
            None => print!("()")
        };

        write!(f, ")")
    }
}

pub struct Parser {
    pub lexemes: Vec<Lexeme>,
    pub curr_lexeme: Lexeme,
    pub curr_index: usize
}

impl Parser {
    pub fn new(lexemes: Vec<Lexeme>) -> Parser {
        Parser{
            lexemes: lexemes,
            curr_lexeme: Lexeme::Nil,
            curr_index: 0
        }
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        self.curr_index = 0;
        self.lexemes.push(Lexeme::Nil);

        self.curr_lexeme = self.lexemes[self.curr_index];

        return self.expr();
    }

    fn eat(&mut self, lexeme: Lexeme) -> Option<String> {
        if self.curr_lexeme == lexeme {
            self.curr_index += 1;
            self.curr_lexeme = self.lexemes[self.curr_index];
            None
        } else {
            Some(format!("Got unexpected token type: {}", lexeme))
        }
    }

    fn factor(&mut self) -> Result<ASTNode, String>  {
        let token = self.curr_lexeme;

        match token {
            Lexeme::Number(val) => {
                match self.eat(Lexeme::Number(val)) {
                    Some(e) => return Err(e),
                    None => ()
                };
    
                Ok(ASTNode{node: Lexeme::Number(val), left: None, right: None})
            },
            Lexeme::Lparen(level) => {
                match self.eat(Lexeme::Lparen(level)) {
                    Some(e) => return Err(e),
                    None => ()
                }
    
                let node = match self.expr() {
                    Ok(val) => val,
                    Err(e) => return Err(e)
                };
    
                match self.eat(Lexeme::Rparen(level)) {
                    Some(e) => return Err(e),
                    None => ()
                };

                Ok(node)
            },
            _ => Err(format!("Unexpected token: {}, was expecting NUMBER or LPAREN.", self.curr_lexeme))
        }
    }

    fn term(&mut self) -> Result<ASTNode, String>  {
        let mut node = match self.factor() {
            Ok(val) => val,
            Err(s) => return Err(s)
        };

        loop {
            let token = self.curr_lexeme;

            match token {
                Lexeme::Mul => match self.eat(Lexeme::Mul) {
                    Some(e) => return Err(e),
                    None => ()
                },
                Lexeme::Div => match self.eat(Lexeme::Div) {
                    Some(e) => return Err(e),
                    None => ()
                },
                _ => break
            }

            node = ASTNode{
                node: token.clone(),
                left: Some(Box::new(node)),
                right: Some(Box::new(match self.factor() {
                    Ok(val) => val,
                    Err(s) => return Err(s)
                }))
            };
        }

        Ok(node)
    }

    fn expr(&mut self) -> Result<ASTNode, String> {
        let mut node = match self.term() {
            Ok(val) => val,
            Err(s) => return Err(s)
        };

        loop {
            let token = self.curr_lexeme;

            match token {
                Lexeme::Plus => match self.eat(Lexeme::Plus) {
                    Some(e) => return Err(e),
                    None => ()
                },
                Lexeme::Minus => match self.eat(Lexeme::Minus) {
                    Some(e) => return Err(e),
                    None => ()
                },
                _ => break
            }

            node = ASTNode{
                node: token.clone(),
                left: Some(Box::new(node)),
                right: Some(Box::new(match self.term() {
                    Ok(val) => val,
                    Err(s) => return Err(s)
                }))
            };
        }

        Ok(node)
    }
}