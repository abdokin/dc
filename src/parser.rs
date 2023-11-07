use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    pub token: Token,
    pub lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &mut Lexer<'a>) -> Self {
        Self {
            token: lexer.next_token(),
            lexer: lexer.clone(),
        }
    }
}
