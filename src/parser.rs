use crate::{ir::Function, lexer::Token};

pub struct Parser {
    pub functions: Vec<Function>,
    pub tokens: Vec<Token>,
    pub tok: usize
}

impl Parser {
    pub fn new() -> Self {
        Self { functions: vec![], tokens: vec![], tok: 0 }
    }

    pub fn set_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
    }

    pub fn advance(&mut self) -> &Token {
        self.tok += 1;
        &self.tokens[self.tok]
    }

    pub fn advance_and_expect_id(&mut self, expect: Token) -> Option<Token> {
        let token = self.advance();

    }

    pub fn parse(&mut self) {
        let mut tok = &self.tokens[self.tok];
        while self.tok < self.tokens.len() {
            let function_type = tok;
            let function_name = self.advance();

        }
    }
}