use std::fmt;

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
}

impl Lexer {
    pub fn new() -> Self {
        Self { 
            tokens: vec![],
            program: String::new(),
            tok: 0
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

    fn skip_whitespace(&mut self) {
        if self.tok >= self.program.len() - 1 { return }
        let mut c = self.program.chars().nth(self.tok).unwrap();
        while c == '\n' || c == '\t' || c == ' ' {
            self.tok += 1;
            c = self.program.chars().nth(self.tok).unwrap();
        }
    }

    fn curr_char(&self) -> char {
        self.program.chars().nth(self.tok).unwrap()
    }

    pub fn set_program(&mut self, program: String) {
        self.program = program;
    } 
    
    pub fn lex(&mut self) {
        let mut id = self.skip_until_whitespace();
        self.tokens.push(Token::Id(id.clone()));
        self.skip_whitespace();
        id = self.skip_until('(');
        self.tokens.push(Token::Id(id.clone()));
        self.tokens.push(Token::Symbol('('));
        self.skip_until(')');
        self.tokens.push(Token::Symbol(')'));
        self.skip_whitespace();
    }
}