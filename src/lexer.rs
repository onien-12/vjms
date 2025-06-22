use std::{fmt, fs::canonicalize};

#[derive(Debug)]
pub enum Token {
    Symbol(char),
    Id(String), // if, else
    Num(usize), // 
    Float(f64),
    Eq(),
}

pub struct Lexer {
    pub tokens: Vec<Token>,
    pub program: String,
    pub tok: usize,
    pub current_id: String
}

impl Lexer {
    pub fn new() -> Self {
        Self { 
            tokens: vec![],
            program: String::new(),
            current_id: String::new(),
            tok: 0,
        }
    }

    fn skip_until(&mut self, until: char) -> String {
        let mut c = self.program.chars().nth(self.tok).unwrap();
        let mut result = String::new();
        while c != until {
            result.push(c);
            self.tok += 1;

            if self.tok < self.program.len() {
                c = self.program.chars().nth(self.tok).unwrap();
            } else {
                break;
            }
        }

        result
    }

    fn skip_until_whitespace(&mut self) -> String {
        let mut c = self.program.chars().nth(self.tok).unwrap();
        let mut result = String::new();
        while c != ' ' && c != '\t' && c != '\n' && self.tok < self.program.len()  {
            result.push(c);
            self.tok += 1;
            if self.tok < self.program.len() {
                c = self.program.chars().nth(self.tok).unwrap();
            } else {
                break;
            }
        }

        result
    }

    fn skip_number(&mut self, is_hex: bool) -> Option<String> {
        if self.tok >= self.program.len() - 1 { return None }
        
        let mut result = String::new();
        let mut c = self.curr_char();
        while c == '0' || c == '1' || c == '2' || c == '3' ||
              c == '4' || c == '5' || c == '6' || c ==  '7' || 
              c == '8' || c == '9' || 
              (is_hex && (
                c == 'a' || c == 'b' || c == 'c' || c == 'd' || c == 'e' || c == 'f'
              )) {
            result.push(c);
            self.tok += 1;
            c = self.curr_char();
        }
        self.tok -= 1;

        Some(result)
    }

    fn curr_char(&self) -> char {
        self.program.chars().nth(self.tok).unwrap()
    }

    fn clear_current_id(&mut self) {
        if self.current_id.len() > 0 {
            self.tokens.push(Token::Id(self.current_id.clone()));
            self.current_id.clear();
        }
    }

    pub fn set_program(&mut self, program: String) {
        self.program = program;
    } 
    
    pub fn lex(&mut self) {
        let mut c = self.curr_char();
        while self.tok < self.program.len() {
            match c {
                ' ' | '\n' | '\t' => self.clear_current_id(),

                '(' | ')' | '[' | ']' | '{' | '}' | ';' => {
                    self.clear_current_id();
                    self.tokens.push(Token::Symbol(c));
                },

                '=' |  
                '-' | '+' | '*' | '/'  | '%' | 
                '!' | '|' | '&' | '^' | '~' => {
                    self.clear_current_id();
                    self.tok += 1;
                    let next_char = self.curr_char();

                    if c == '=' && next_char == '=' {
                        self.tokens.push(Token::Eq());
                    }
                },

                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if self.current_id.len() > 0 {
                        self.current_id.push(c);
                    } else {
                        let num_str = self.skip_number(false).unwrap();
                        self.tokens.push(Token::Num(num_str.parse::<usize>().unwrap()));
                    }
                },

                '"' | '\'' => {
                    self.clear_current_id();
                },

                _ => self.current_id.push(c) 
            }
            self.tok += 1;

            if self.tok >= self.program.len() { break }
            c = self.curr_char();
        }
    }
}