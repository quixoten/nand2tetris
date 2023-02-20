use std::cell::Cell;
use std::io::Result;

use crate::tokens::{Token, TokenKind};

pub struct Lexer {
    chars: Vec<char>,
    pos: Cell<usize>,
    row: Cell<usize>,
    col: Cell<usize>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().filter(|&char| char != '\r').collect(),
            pos: Cell::new(0),
            row: Cell::new(1),
            col: Cell::new(1),
        }
    }

    fn row(&self) -> usize {
        self.row.get()
    }

    fn row_inc(&self) {
        self.row.replace(self.row.get() + 1);
    }

    fn col(&self) -> usize {
        self.col.get()
    }

    fn col_inc(&self) {
        self.col.replace(self.col.get() + 1);
    }

    fn col_set(&self, n: usize) {
        self.col.set(n)
    }

    fn pos(&self) -> usize {
        self.pos.get()
    }

    fn pos_inc(&self) {
        self.pos.set(self.pos.get() + 1);
    }

    fn char(&self) -> Option<&char> {
        self.look_ahead(0)
    }

    fn look_ahead(&self, n: usize) -> Option<&char> {
        self.chars.get(self.pos.get() + n)
    }

    fn consume_char(&self) {
        if let Some(&char) = self.char() { match char {
            '\n' => {
                self.row_inc();
                self.col_set(1);
            }
            _ => self.col_inc(),
        }}

        self.pos_inc();
    }

    fn consume_whitespace(&self) {
        while let Some(&char) = self.char() {
            if char.is_ascii_whitespace() {
                if char == '\n' {
                    break;
                } else {
                    self.consume_char();
                }
            } else {
                break;
            }
        }
    }

    fn consume_eol(&self) -> Result<Token> {
        let token = Ok(self.new_char_token(TokenKind::EndOfLine));
        self.consume_char();
        token
    }

    fn consume_slash(&self) -> Result<Token> {
        if let Some(&char) = self.look_ahead(1) {
            if char == '/' {
                return self.consume_comment();
            }
        }

        let token = Ok(self.new_char_token(TokenKind::Illegal));
        self.consume_char();
        token
    }

    fn consume_comment(&self) -> Result<Token> {
        let (beg_pos, beg_row, beg_col) = (self.pos(), self.row(), self.col());

        while self.char().is_some() {
            match self.look_ahead(1) {
                Some(&peek) => match peek {
                    '\n' => break,
                    _ => self.consume_char(),
                },
                None => break,
            }
        }

        let token = Ok(self.new_string_token(TokenKind::Comment, beg_pos, beg_row, beg_col));
        self.consume_char();
        token
    }

    fn consume_number(&self) -> Result<Token> {
        let (beg_pos, beg_row, beg_col) = (self.pos(), self.row(), self.col());

        while let Some(&char) = self.char() {
            if char.is_ascii_digit() {
                if let Some(&peek) = self.look_ahead(1) {
                    if peek.is_ascii_digit() {
                        self.consume_char();
                        continue;
                    }
                }
            }
            break;
        }

        let token = Ok(self.new_string_token(TokenKind::Number, beg_pos, beg_row, beg_col));
        self.consume_char();
        token
    }

    fn consume_identifier(&self) -> Result<Token> {
        let (beg_pos, beg_row, beg_col) = (self.pos(), self.row(), self.col());

        while let Some(&char) = self.look_ahead(1) {
            if char.is_ascii_whitespace() {
                break;
            }
            self.consume_char();
        }

        let token = Ok(self.new_identifier_token(beg_pos, beg_row, beg_col));
        self.consume_char();
        token
    }

    fn new_char_token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            text: &self.chars[self.pos()..=self.pos()],
            beg_row: self.row(),
            beg_col: self.col(),
            end_row: self.row(),
            end_col: self.col(),
        }
    }

    fn new_string_token(
        &self,
        kind: TokenKind,
        beg_pos: usize,
        beg_row: usize,
        beg_col: usize,
    ) -> Token {
        Token {
            kind,
            text: &self.chars[beg_pos..=self.pos()],
            beg_row,
            beg_col,
            end_row: self.row(),
            end_col: self.col(),
        }
    }

    fn new_identifier_token(&self, beg_pos: usize, beg_row: usize, beg_col: usize) -> Token {
        let text = &self.chars[beg_pos..=self.pos()];
        let identifier: &str = &String::from_iter(text);
        let kind = match identifier {
            "add" => TokenKind::Add,
            "and" => TokenKind::And,
            "argument" => TokenKind::Argument,
            "call" => TokenKind::Call,
            "constant" => TokenKind::Constant,
            "eq" => TokenKind::Eq,
            "function" => TokenKind::Function,
            "goto" => TokenKind::Goto,
            "gt" => TokenKind::Gt,
            "if-goto" => TokenKind::IfGoto,
            "label" => TokenKind::Label,
            "local" => TokenKind::Local,
            "lt" => TokenKind::Lt,
            "neg" => TokenKind::Neg,
            "not" => TokenKind::Not,
            "or" => TokenKind::Or,
            "pointer" => TokenKind::Pointer,
            "pop" => TokenKind::Pop,
            "push" => TokenKind::Push,
            "return" => TokenKind::Return,
            "static" => TokenKind::Static,
            "sub" => TokenKind::Sub,
            "temp" => TokenKind::Temp,
            "that" => TokenKind::That,
            "this" => TokenKind::This,
            _ => TokenKind::Identifier,
        };

        Token {
            kind,
            text,
            beg_row,
            beg_col,
            end_row: self.row(),
            end_col: self.col(),
        }
    }

    pub fn list_all_tokens(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Ok(token) = self.next_token() {
            match token.kind {
                TokenKind::EndOfFile => {
                    tokens.push(token);
                    break;
                }

                _ => tokens.push(token),
            }
        }

        tokens
    }

    pub fn next_token(&self) -> Result<Token> {
        let token: Result<Token>;

        self.consume_whitespace();

        match self.char() {
            Some(&char) => match char {
                '\n' => token = self.consume_eol(),
                '/' => token = self.consume_slash(),
                'a'..='z' | 'A'..='Z' => token = self.consume_identifier(),
                '0'..='9' => token = self.consume_number(),
                _ => {
                    token = Ok(self.new_char_token(TokenKind::Illegal));
                    self.consume_char();
                }
            },
            None => token = Ok(Token::empty(TokenKind::EndOfFile)),
        };

        token
    }
}
