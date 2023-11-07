use std::str::Chars;

use crate::report_error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum KeywordType {
    Fn,
    Return,
    Print,
    If,
    ElseIf,
    Else,
    Let,
}

fn identify_keyword(word: &str) -> Option<KeywordType> {
    match word {
        "fn" => Some(KeywordType::Fn),
        "return" => Some(KeywordType::Return),
        "print" => Some(KeywordType::Print),
        "let" => Some(KeywordType::Let),
        "if" => Some(KeywordType::If),
        "else" => Some(KeywordType::Else),
        "else if" => Some(KeywordType::ElseIf),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Keyword(KeywordType),
    Identifier(String),
    IntegerLiteral(i64),
    StringLiteral(String),
    LeftBrace,
    RightBrace,
    RightBracket,
    LeftBracket,
    LessThan,
    GreaterThan,
    Colon,
    LeftParenthesis,
    RightParenthesis,
    Semicolon,
    Comma,
    Assignment,
    Minus,
    Plus,
    Multiplication,
    EndOfFile,
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    src: String,
    character_list: Chars<'a>,
    index: usize,
    file_name: String,
    row: usize,
    col: usize,
    size: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str, name: &'a str) -> Self {
        Self {
            character_list: src.chars(),
            src: src.to_string(),
            index: 0,
            col: 1,
            row: 1,
            file_name: name.to_owned(),
            size: src.len(),
        }
    }
    fn next(&mut self) -> Option<char> {
        if self.index < self.size {
            self.index += 1;
            self.col += 1;
            self.character_list.next()
        } else {
            None
        }
    }
    fn current(&mut self) -> Option<char> {
        self.src.clone().chars().nth(self.index)
    }
    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                if c == '\n' {
                    self.row += 1;
                    self.col = 0;
                }
                self.next();
            } else {
                break;
            }
        }
    }
    fn lax_number(&mut self, c: char) -> Token {
        let mut number = String::new();
        number.push(c);
        while let Some(next_char) = self.current() {
            if next_char.is_numeric() {
                number.push(next_char);
                self.next();
            } else {
                break;
            }
        }
        let parsed_number = match number.parse::<i64>() {
            Ok(value) => value,
            Err(e) => {
                report_error!(e, self.file_name, self.row, self.col)
            }
        };
        Token::IntegerLiteral(parsed_number)
    }
    fn lax_identifier(&mut self, c: char) -> Token {
        let mut identifier = String::new();
        identifier.push(c);
        while let Some(next_char) = self.current() {
            if next_char.is_alphanumeric() {
                identifier.push(next_char);
                self.next();
            } else {
                break;
            }
        }
        if let Some(keyword) = identify_keyword(&identifier) {
            Token::Keyword(keyword)
        } else {
            Token::Identifier(identifier)
        }
    }
    fn lax_string_literal(&mut self, c: char) -> Token {
        let mut identifier = String::new();
        while let Some(next_char) = self.current() {
            if next_char != '"' {
                identifier.push(next_char);
                self.next();
            } else {
                self.next();
                break;
            }
        }
        Token::StringLiteral(identifier)
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();
        match self.next() {
            Some(c) => match c {
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                '(' => Token::LeftParenthesis,
                ')' => Token::RightParenthesis,
                '[' => Token::LeftBracket,
                ']' => Token::RightBracket,
                ',' => Token::Comma,
                ';' => Token::Semicolon,
                ':' => Token::Colon,
                '=' => Token::Assignment,
                '>' => Token::GreaterThan,
                '<' => Token::LessThan,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Multiplication,
                x if x.is_numeric() => self.lax_number(c),
                x if x.is_alphabetic() => self.lax_identifier(c),
                '"' => self.lax_string_literal(c),
                '\0' => Token::EndOfFile,
                x => {
                    report_error!(
                        format!("Unexpected token {x}"),
                        self.file_name,
                        self.row,
                        self.col
                    )
                }
            },
            None => Token::EndOfFile,
        }
    }
}
