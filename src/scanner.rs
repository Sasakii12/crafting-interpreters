use crate::tokens::{Token, TokenType, Literal};
struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self{
        Scanner {source, tokens: Vec::new(), start: 0, current: 0, line: 0}
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            _ => todo!()
        }
    }

    fn advance(&mut self) -> char{
        self.current += 1;
        self.source.chars().nth(self.current).unwrap()
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        loop {
            if self.is_at_end() {
                self.start = self.current;
                todo!()
            }

            self.tokens.push(Token::new(TokenType::EOF, String::from(""), Literal::Nil, self.line));
            return self.tokens
        }
    }

    fn add_token(&mut self,data_type: TokenType, literal: Literal) {
        let text = self.source.get(self.start..self.current).unwrap().to_string();
        self.tokens.push(Token::new(data_type, text, literal, self.line));
    }

}
