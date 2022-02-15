use crate::Token;
use crate::TokenType;
use crate::Scanner;
use std::fs::File;
use std::io::Write;

pub struct Parser {
    scanner: Scanner,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(f: &str) -> Parser {
        let scan = Scanner::new(f);
        Parser {
            scanner: scan,
            tokens: Vec::new()
        }
    }

    pub fn parse(&mut self) {
        loop {
            match self.scanner.get_next_token() {
                Some(mut token) => {
                    self.tokens.push(token);
                },
                _ => { break; }
            }
        }
    }

    pub fn check(&mut self) -> &str {
        self.parse();
        match self.check_program(0) {
            Some(x) => {
                "OK!"
            },
            _ => {
                "Panic"
            }
        }
    }

    pub fn write_html(&mut self) {
        let mut file = File::create("output.xhtml").expect("Failed to create file");
        file.write_all("<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1
        -transitional.dtd\">\n".as_bytes()).expect("write failed");
        file.write_all("<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\">\n".as_bytes()).expect("write failed");
        file.write_all("<head>\n<title>\nX Formatted file</title>\n</head>\n<body bgcolor=\"navy\" text=\"yellow\" link=\"yellow\" vlink=\"yellow\">".as_bytes()).expect("write failed");
        file.write_all("<font face=\"Courier New\">\n".as_bytes()).expect("write failed");

        let mut space_num = 0;
        for i in 0..self.tokens.len() {
            // println!("{} token_type: {}",self.tokens[i].get_text(), self.tokens[i].get_type().as_str());
            let color : &str;
            let mut text = self.tokens[i].get_text();
            match self.tokens[i].get_type(){
                TokenType::FLOATCONSTANT|TokenType::INTCONSTANT => {
                    color = "aqua";
                    let format = format!("<font color=\"{}\"><b>{}</b></font>", color, text);
                    file.write_all(format.as_bytes()).expect("write failed");
                }

                TokenType::FUNCTION => {
                    color = "orange";
                    let format = format!("<font color=\"{}\">{}</font>", color, text);
                    file.write_all(format.as_bytes()).expect("write failed");
                }

                TokenType::VARIABLE => {
                    color = "yellow";
                    let format = format!("<font color=\"{}\">{}</font>", color, text);
                    file.write_all(format.as_bytes()).expect("write failed");
                }

                TokenType::OPERATOR | TokenType::KEYWORD => {
                    color = "white";
                    let format = format!("<font color=\"{}\"><b>{}</b></font>", color, text);
                    file.write_all(format.as_bytes()).expect("write failed");
                }

                _ => {
                    color = "white";
                    let format = format!("<font color=\"{}\">{}</font>", color, text);
                    file.write_all(format.as_bytes()).expect("write failed");
                }
            }

            if i + 1 < self.tokens.len() {
                if self.tokens[i+1].get_line_number() > self.tokens[i].get_line_number() { //new line
                    file.write_all("<br />\n".as_bytes()).expect("write failed");
                    space_num = self.tokens[i+1].get_char_pos() - 1; //char starts from 1
                }

                else {
                    space_num = self.tokens[i+1].get_char_pos() - self.tokens[i].get_char_pos() - (self.tokens[i].get_text().to_string().len() as i32);//space in between characters
                }

                for j in 0..space_num {
                    file.write_all("&nbsp;".as_bytes()).expect("write failed"); //space ahead
                }
            }
        }
    }

    pub fn check_program(&mut self, mut pos: usize) -> Option<usize> {
        loop {
            match self.check_declaration(pos) {
                Some(next_pos) => { pos = next_pos; },
                _ => { break; }
            }
        }
        match self.check_main_declaration(pos) {
            Some(next_pos) => { pos = next_pos; },
            _ => { return None; }
        }
        loop {
            match self.check_function_definition(pos) {
                Some(next_pos) => { pos = next_pos; },
                _ => { break; }
            }
        }
        return Some(pos);
    }

    pub fn check_declaration(&mut self, mut pos: usize) -> Option<usize> {
        match self.check_declaration_type(pos) {
            Some(next_pos) => {
                pos = next_pos;
                match self.check_variable_declaration(pos) {
                    Some(next_pos) => {
                        self.tokens[pos - 1].set_type(TokenType::VARIABLE);
                        pos = next_pos;
                        return Some(pos);
                    },
                    _ => { }
                }
                match self.check_function_declaration(next_pos) {
                    Some(next_pos) => {
                        self.tokens[pos - 1].set_type(TokenType::FUNCTION);
                        pos = next_pos;
                        return Some(pos);
                    },
                    _ => { }
                }

                return None;
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_main_declaration(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) {
            Some(token) => {
                if token.get_text() == "void" {
                    pos += 1;
                    match self.tokens.get(pos) {
                        Some(token) => {
                            if token.get_text() == "main" {
                                pos += 1;
                                match self.tokens.get(pos) {
                                    Some(token) => {
                                        if token.get_text() == "(" {
                                            pos += 1;
                                            match self.tokens.get(pos) {
                                                Some(token) => {
                                                    if token.get_text() == ")" {
                                                        pos += 1;
                                                        match self.check_block(pos) {
                                                            Some(next_pos) => {
                                                                return Some(next_pos);
                                                            },
                                                            _ => { return None; }
                                                        }
                                                    } else {
                                                        return None;
                                                    }
                                                },
                                                _ => { return None; }
                                            }
                                        } else {
                                            return None;
                                        }
                                    },
                                    _ => { return None; }
                                }
                            } else {
                                return None;
                            }
                        },
                        _ => { return None; }
                    }
                } else {
                    return None;
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_function_definition(&mut self, mut pos: usize) -> Option<usize> {
        match self.check_declaration_type(pos) {
            Some(next_pos) => {
                match self.check_parameter_block(next_pos) {
                    Some(next_pos2) => {
                        match self.check_block(next_pos2) {
                            Some(next_pos3) => {
                                self.tokens[next_pos - 1].set_type(TokenType::FUNCTION);
                                // println!("//////entered token should be ( :{}:::::type: {}", self.tokens[next_pos - 1].get_text(),self.tokens[next_pos - 1].get_type().as_str());
                                return Some(next_pos3);
                            },
                            _ => { return None; }
                        }
                    },
                    _ => { return None; }
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_declaration_type(&mut self, mut pos: usize) -> Option<usize> {
        match self.check_data_type(pos) {
            Some(next_pos) => {
                pos = next_pos;
                match self.tokens.get(pos) {
                    Some(token) => {
                        if token.is_identifier() {
                            pos += 1;
                            return Some(pos);
                        } else {
                            return None;
                        }
                    },
                    _ => { return None; }
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_variable_declaration(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) {
            Some(token) => {
                if token.get_text() == "=" {
                    pos += 1;
                    match self.check_constant(pos) {
                        Some(next_pos) => {
                            pos = next_pos;
                        },
                        _ => { return None; }
                    }
                }
                match self.tokens.get(pos) {
                    Some(token) => {
                        if token.get_text() == ";" {
                            pos += 1;
                            return Some(pos);
                        } else {
                            return None;
                        }
                    },
                    _ => { return None; }
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_function_declaration(&mut self, mut pos: usize) -> Option<usize> {
        match self.check_parameter_block(pos) {
            Some(next_pos) => {
                pos = next_pos;
                match self.tokens.get(pos) {
                    Some(token) => {
                        if token.get_text() == ";" {
                            pos += 1;
                            return Some(pos);
                        } else {
                            return None;
                        }
                    },
                    _ => { return None; }
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_block(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) {
            Some(token) => {
                if token.get_text() == "{" {
                    pos += 1;
                    loop {
                        match self.check_declaration(pos) { 
                            Some(next_pos) => { pos = next_pos; }, //if check 成功了，返回新的pos
                            _ => { break; } //如果不是declaration，退出loop
                        }
                    }
                    loop {
                        match self.check_statement(pos) {
                            Some(next_pos) => { pos = next_pos; },
                            _ => { break; }
                        }
                    }
                    loop {
                        match self.check_function_definition(pos) {
                            Some(next_pos) => { pos = next_pos; },
                            _ => { break; }
                        }
                    }
                    match self.tokens.get(pos) {
                        Some(token) => {
                            if token.get_text() == "}" {
                                pos += 1;
                                return Some(pos);
                            } else {
                                return None;
                            }
                        },
                        _ => { return None; }
                    }
                } else {
                    return None;
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_parameter_block(&mut self, mut pos: usize) -> Option<usize> {
        let last_token_pos = pos - 1;
        // println!("//////entered token should be ( :{}:::::position: {}", self.tokens[pos].get_text(), pos);
        match self.tokens.get(pos) {
            Some(token) => {
                if token.get_text() == "(" { //是不是 （
                    pos += 1;
                    match self.check_parameter(pos) { //有没有param
                        Some(next_pos) => {
                            pos = next_pos;
                            loop {
                                match self.tokens.get(pos) {
                                    Some(token) => {
                                        if token.get_text() == "," {
                                            pos += 1;
                                            match self.check_parameter(pos) {//如果有param
                                                Some(next_pos) => { pos = next_pos; },
                                                _ => { return None; }//如果没有param，rule失败。,后面必须有param
                                            }
                                        } else {
                                            break;//没有, 退出loop
                                        }
                                    },
                                    _ => { return None; }
                                }
                            }
                        },
                        _ => { } //没有param就继续往下
                    }
                    match self.tokens.get(pos) {
                        Some(token) => {
                            if token.get_text() == ")" { //有没有)
                                if self.tokens[last_token_pos].is_identifier() { //if last from param block is an identifier

                                    self.tokens[last_token_pos].set_type(TokenType::FUNCTION); //then the identifier is a function

                                }

                                pos += 1;
                                return Some(pos);//成功了就返回新的pos
                            } else {
                                return None; //不然结束
                            }
                        },
                        _ => { return None; }
                    }
                } else {
                    return None;
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_data_type(&mut self, pos: usize) -> Option<usize> {
        match self.check_integer_type(pos) {
            Some(next_pos) => { return Some(next_pos); },
            _ => { }
        }
        match self.check_float_type(pos) {
            Some(next_pos) => { return Some(next_pos); },
            _ => { }
        }
        None
    }

    pub fn check_constant(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) {
            Some(token) => {
                if token.is_constant() {
                    pos += 1;
                    return Some(pos);
                } else {
                    return None;
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_statement(&mut self, mut pos: usize) -> Option<usize> {
        match self.check_assignment(pos) {
            Some(next_pos) => { return Some(next_pos); },
            _ => { }
        }
        match self.check_while_loop(pos) {
            Some(next_pos) => { return Some(next_pos); },
            _ => { }
        }
        match self.check_if_statement(pos) {
            Some(next_pos) => { return Some(next_pos); },
            _ => { }
        }
        match self.check_return_statement(pos) {
            Some(next_pos) => { return Some(next_pos); },
            _ => { }
        }
        match self.check_expression(pos) {
            Some(next_pos) => {
                pos = next_pos;
                match self.tokens.get(pos) {
                    Some(token) => {
                        if token.get_text() == ";" {
                            pos += 1;
                            return Some(pos);
                        }
                    },
                    _ => { return None; }
                }
            },
            _ => { }
        }
        None
    }

    pub fn check_parameter(&mut self, mut pos: usize) -> Option<usize> {
        match self.check_data_type(pos) {
            Some(next_pos) => {
                pos = next_pos;
                match self.tokens.get(pos) {
                    Some(token) => {
                        if token.is_identifier() {
                            pos += 1;
                            return Some(pos);
                        } else {
                            return None;
                        }
                    },
                    _ => { return None; }
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_integer_type(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) {
            Some(token) => {
                if token.get_text() == "unsigned" {
                    pos += 1;
                }
            },
            _ => { return None; }
        }
        match self.tokens.get(pos) {
            Some(token) => {
                match token.get_text() {
                    "char" | "short" | "int" | "long" => {
                        pos += 1;
                        return Some(pos);
                    },
                    _ => { return None; }
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_float_type(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) {
            Some(token) => {
                match token.get_text() {
                    "float" | "double" => {
                        pos += 1;
                        return Some(pos);
                    },
                    _ => { }
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_assignment(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) {
            Some(token) => {
                if token.is_identifier() {
                    pos += 1;
                    match self.tokens.get(pos) {
                        Some(token) => {
                            if token.get_text() == "=" {
                                pos += 1;
                                loop {
                                    match self.tokens.get(pos) {
                                        Some(token) => {
                                            if token.is_identifier() {
                                                match self.tokens.get(pos + 1) {
                                                    Some(token) => {
                                                        if token.get_text() == "=" {
                                                            pos += 2;
                                                        } else {
                                                            break;
                                                        }
                                                    },
                                                    _ => { return None; }
                                                }
                                            } else {
                                                break;
                                            }
                                        },
                                        _ => { return None; }
                                    }
                                }
                                match self.check_expression(pos) {
                                    Some(next_pos) => { pos = next_pos; },
                                    _ => { return None; }
                                }
                                match self.tokens.get(pos) {
                                    Some(token) => {
                                        if token.get_text() == ";" {
                                            pos += 1;
                                            return Some(pos);
                                        } else {
                                            return None;
                                        }
                                    },
                                    _ => { return None; }
                                }
                            } else {
                                return None;
                            }
                        },
                        _ => { return None; }
                    }
                } else {
                    return None;
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_while_loop(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) {
            Some(token) => {
                if token.get_text() == "while" {
                    pos += 1;
                    match self.tokens.get(pos) {
                        Some(token) => {
                            if token.get_text() == "(" {
                                pos += 1;
                                match self.check_expression(pos) {
                                    Some(next_pos) => {
                                        pos = next_pos;
                                        match self.tokens.get(next_pos) {
                                            Some(token) => {
                                                if token.get_text() == ")" {
                                                    pos += 1;
                                                    match self.check_block(pos) {
                                                        Some(next_pos) => {
                                                            return Some(next_pos);
                                                        },
                                                        _ => { return None; }
                                                    }
                                                } else {
                                                    return None;
                                                }
                                            },
                                            _ => { return None; }
                                        }
                                    },
                                    _ => { return None; }
                                }
                            } else {
                                return None;
                            }
                        },
                        _ => { return None; }
                    }
                } else {
                    return None;
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_if_statement(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) {
            Some(token) => {
                if token.get_text() == "if" {
                    pos += 1;
                    match self.tokens.get(pos) {
                        Some(token) => {
                            if token.get_text() == "(" {
                                pos += 1;
                                match self.check_expression(pos) {
                                    Some(next_pos) => {
                                        pos = next_pos;
                                        match self.tokens.get(pos) {
                                            Some(token) => {
                                                if token.get_text() == ")" {
                                                    pos += 1;
                                                    match self.check_block(pos) {
                                                        Some(next_pos) => {
                                                            return Some(next_pos);
                                                        },
                                                        _ => { return None; }
                                                    }
                                                } else {
                                                    return None;
                                                }
                                            },
                                            _ => { return None; }
                                        }
                                    },
                                    _ => { return None; }
                                }
                            } else {
                                return None;
                            }
                        },
                        _ => { return None; }
                    }
                } else {
                    return None;
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_return_statement(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) {
            Some(token) => {
                if token.get_text() == "return" {
                    pos += 1;
                    match self.check_expression(pos) {
                        Some(next_pos) => {
                            pos = next_pos;
                            match self.tokens.get(pos) {
                                Some(token) => {
                                    if token.get_text() == ";" {
                                        pos += 1;
                                        return Some(pos);
                                    } else {
                                        return None;
                                    }
                                },
                                _ => { return None; }
                            }
                        },
                        _ => { return None; }
                    }
                } else {
                    return None;
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_expression(&mut self, mut pos: usize) -> Option<usize> {
        match self.check_simple_expression(pos) {
            Some(next_pos) => {
                pos = next_pos;
                match self.check_relation_operator(pos) {
                    Some(next_pos) => {
                        pos = next_pos;
                        match self.check_simple_expression(pos) {
                            Some(next_pos) => { return Some(next_pos); },
                            _ => { return None; }
                        }
                    },
                    _ => { }
                }
                return Some(pos);
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_simple_expression(&mut self, mut pos: usize) -> Option<usize> {
        match self.check_term(pos) {
            Some(next_pos) => {
                pos = next_pos;
                loop {
                    match self.check_add_operator(pos) {
                        Some(next_pos) => {
                            pos = next_pos;
                            match self.check_term(pos) {
                                Some(next_pos) => { pos = next_pos; },
                                _ => { return None; }
                            }
                        },
                        _ => { break; }
                    }
                }
                return Some(pos);
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_term(&mut self, mut pos: usize) -> Option<usize> {
        match self.check_factor(pos) {
            Some(next_pos) => {
                pos = next_pos;
                loop {
                    match self.check_mult_operator(pos) {
                        Some(next_pos) => {
                            pos = next_pos;
                            match self.check_factor(pos) {
                                Some(next_pos) => { pos = next_pos; },
                                _ => { return None; }
                            }
                        },
                        _ => { break; }
                    }
                }
                return Some(pos);
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_factor(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) { //(expression)
            Some(token) => {
                if token.get_text() == "(" {
                    pos += 1;
                    match self.check_expression(pos) {
                        Some(next_pos) => {//如果成功
                            pos = next_pos;
                            match self.tokens.get(pos) {//拿下一个
                                Some(token) => {
                                    if token.get_text() == ")" {//如果是）
                                        pos += 1;
                                        return Some(pos);//返回新的
                                    }
                                },
                                _ => { return None; } //没有")"
                            }
                        },
                        _ => { return None; } //不成功expression
                    }
                }
            },
            _ => { return None; } //没有 （
        }
        
        match self.check_constant(pos) { //constant
            Some(next_pos) => {
                return Some(next_pos);
            },
            _ => { }
        }

        let mut is_function = false;
        let mut last_pos = pos;
        match self.tokens.get(pos) { //identifier
            Some(token) => {
                if token.is_identifier() {
                    pos += 1;
                    match self.tokens.get(pos) {
                        Some(token) => {
                            if token.get_text() == "(" {
                                is_function = true;
                                pos += 1;
                                match self.check_expression(pos) {
                                    Some(next_pos) => {
                                        pos = next_pos;
                                        loop {
                                            match self.tokens.get(pos) {
                                                Some(token) => {
                                                    if token.get_text() == "," {
                                                        pos += 1;
                                                        match self.check_expression(pos) {
                                                            Some(next_pos) => { pos = next_pos; },
                                                            _ => { return None; }
                                                        }
                                                    } else { break; }
                                                },
                                                _ => { return None; }
                                            }
                                        }
                                        match self.tokens.get(pos) {
                                            Some(token) => {
                                                if token.get_text() == ")" {
                                                    pos += 1;
                                                } else {
                                                    return None;
                                                }
                                            },
                                            _ => { return None; }
                                        }
                                    },
                                    _ => { return None; }
                                }
                            }
                        }, 
                        _ => { return None; }
                    }
                    if is_function{
                        self.tokens[last_pos].set_type(TokenType::FUNCTION);
                    }
                    return Some(pos); //没有后面(部分，但也成功了
                }
            },
            _ => { return None; }// 不是identifier就none了
        }
        None
    }

    pub fn check_relation_operator(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) {
            Some(token) => {
                match token.get_text() {
                    "==" | "<" | ">" | "<=" | ">=" | "!=" => { 
                        pos += 1;
                        return Some(pos); 
                    },
                    _ => { return None; }
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_add_operator(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) {
            Some(token) => {
                match token.get_text() {
                    "+" | "-" => { 
                        pos += 1;
                        return Some(pos); 
                    },
                    _ => { return None; }
                }
            },
            _ => { return None; }
        }
        None
    }

    pub fn check_mult_operator(&mut self, mut pos: usize) -> Option<usize> {
        match self.tokens.get(pos) {
            Some(token) => {
                match token.get_text() {
                    "*" | "/" => {
                        pos += 1;
                        return Some(pos); 
                    },
                    _ => { return None; }
                }
            },
            _ => { return None; }
        }
        None
    }
}

