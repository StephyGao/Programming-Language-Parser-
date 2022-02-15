use crate::CharStream;
use crate::Token;
use crate::TokenType;

pub struct Scanner {
    stream: CharStream,
    line_num: i32,
    char_pos: i32,
    preceded_by_id_or_constant: bool
}

pub fn is_id_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

pub fn is_num_start(c: char) -> bool {
    c.is_digit(10)
}

pub fn is_op_start(c: char) -> bool {
    match c {
        '(' | ',' | ')' | '{' | '}' | '=' | '<' | '>' | '!' | '+' | '*' | '/' | ';' => true,
        _ => false
    }
}

pub fn is_keyword(name: &str) -> bool {
    [
        "unsigned",
        "char",
        "short",
        "int",
        "long",
        "float",
        "double",
        "while",
        "if",
        "return",
        "void",
        "main",
    ].contains(&name)
}

pub fn invalid_token(text: &str, line_num: i32, char_pos: i32) -> Option<Token> {
    Some(Token::new(text.to_string(), TokenType::INVALID, line_num, char_pos))
}


impl Scanner {
    pub fn new(f: &str) -> Scanner  {
        let cs = CharStream::new(f);
        Scanner {
            stream: cs,
            line_num: 1,
            char_pos: 1,
            preceded_by_id_or_constant: false
        }
    }

    fn skip_spaces(&mut self) {
        loop {
            match self.stream.peek_next_char() {
                Some(c) => {
                    match c {
                        ' ' => {
                            self.stream.get_next_char();
                            self.char_pos += 1;
                        },
                        '\t' => {
                            self.stream.get_next_char();
                            self.char_pos += 4;
                        },
                        '\n' => {
                            self.stream.get_next_char(); 
                            self.line_num += 1;
                            self.char_pos = 1;
                        },
                        '\r' => { 
                            self.stream.get_next_char(); 
                        },
                        _ => { break; }
                    };
                },
                _ => { break; }
            }
        }
    }

    // handle keywords, variables and functions
    fn handle_identifier(&mut self) -> Option<Token> {
        self.preceded_by_id_or_constant = false;
        let mut text = String::new();
        let line_num = self.line_num;
        let char_pos = self.char_pos;
        loop {
            match self.stream.peek_next_char() {
                Some(c) if c == '_' || c.is_digit(10) || c.is_alphabetic() => {
                    self.stream.get_next_char();
                    text.push(c);
                    self.char_pos += 1;
                },
                _ => {
                    /* check if is keyword */
                    if is_keyword(&text) {
                        return Some(Token::new(text, TokenType::KEYWORD, line_num, char_pos));
                    } else {
                        self.preceded_by_id_or_constant = true;
                        return Some(Token::new(text, TokenType::VARIABLE, line_num, char_pos));
                    }
                }
            }
        }
    }

    fn handle_number(&mut self) -> Option<Token> {
        self.preceded_by_id_or_constant = false;
        let mut text = String::new();
        let line_num = self.line_num;
        let char_pos = self.char_pos;
        let mut is_float = false;
        match self.stream.peek_next_char() {
            Some(c) if c == '-' => { 
                self.stream.get_next_char();
                text.push(c);
                self.char_pos += 1
            },
            _ => { }
        }
        loop {
            match self.stream.peek_next_char() {
                Some(c) if c == '.' || c.is_digit(10) => {
                    self.stream.get_next_char();
                    text.push(c);
                    self.char_pos += 1;
                    if c == '.' {
                        if is_float {
                            return invalid_token(&text, line_num, char_pos);
                        } else {
                            is_float = true;
                        }
                    }
                },
                _ => {
                    self.preceded_by_id_or_constant = true;
                    if is_float {
                        return Some(Token::new(text, TokenType::FLOATCONSTANT, line_num, char_pos));
                    } else {
                        return Some(Token::new(text, TokenType::INTCONSTANT, line_num, char_pos));
                    }
                }
            }
        }
    }

    fn handle_operator(&mut self) -> Option<Token> {
        self.preceded_by_id_or_constant = false;
        let line_num = self.line_num;
        let char_pos = self.char_pos;
        match self.stream.peek_next_char() {
            Some(c) => {
                match c {
                    '(' | ',' | ')' | '{' | '}' | '+' | '-' | '*' | '/' | ';' => {
                        self.stream.get_next_char();
                        self.char_pos += 1;
                        Some(Token::new(c.to_string(), TokenType::OPERATOR, line_num, char_pos))
                    },
                    '=' | '<' | '>' | '!' => {
                        let mut text = String::from(c.to_string());
                        self.stream.get_next_char();
                        self.char_pos += 1;
                        match self.stream.peek_next_char() {
                            Some(c2) if c2 == '=' => {
                                self.stream.get_next_char();
                                self.char_pos += 1;
                                text.push(c2);
                            },
                            _ => { 
                                if c == '!' {
                                    return invalid_token("!", line_num, char_pos);
                                }
                            }
                        }
                        Some(Token::new(text, TokenType::OPERATOR, line_num, char_pos))
                    },
                    _ => {
                        invalid_token(&c.to_string(), line_num, char_pos)
                    }
                }
            },
            _ => None
        }
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        self.skip_spaces();
        match self.stream.peek_next_char() {
            Some(c) => { 
                match c {
                    c if is_id_start(c) => self.handle_identifier(),
                    c if is_num_start(c) => self.handle_number(),
                    c if is_op_start(c) => self.handle_operator(),
                    '-' => { 
                        match self.stream.peek_ahead_char(1) {
                            Some(c2) if c2.is_digit(10) => {
                                if !self.preceded_by_id_or_constant {
                                    self.handle_number()
                                } else {
                                    self.handle_operator()
                                }
                            },
                            _ => { self.handle_operator() }
                        }
                    },
                    _ => invalid_token(&c.to_string(), self.line_num, self.char_pos)
                }
            },
            _ => None
        }
    }
}

