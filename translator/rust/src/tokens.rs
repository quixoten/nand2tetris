use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // push / pop Commands
    Push,
    Pop,

    // segments
    Local,
    Argument,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,

    // arithmetic / logical commands
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,

    // branching commands
    Label,
    Goto,
    IfGoto,

    // function commands
    Function,
    Call,
    Return,

    // other
    Number,
    Comment,
    EndOfFile,
    EndOfLine,
    Identifier,
    Illegal,
}

impl TokenKind {
    pub fn is_valid_push_segment(&self) -> bool {
        matches!(self,
            TokenKind::Local
                | TokenKind::Argument
                | TokenKind::Static
                | TokenKind::Constant
                | TokenKind::This
                | TokenKind::That
                | TokenKind::Pointer
                | TokenKind::Temp
        )
    }

    pub fn is_valid_pop_segment(&self) -> bool {
        matches!(
            self,
            TokenKind::Local
                | TokenKind::Argument
                | TokenKind::Static
                | TokenKind::This
                | TokenKind::That
                | TokenKind::Pointer
                | TokenKind::Temp
        )
    }
}

pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a [char],
    pub beg_row: usize,
    pub beg_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

impl Token<'_> {
    pub fn empty(kind: TokenKind) -> Self {
        Self {
            kind,
            text: &[],
            beg_row: 0,
            beg_col: 0,
            end_row: 0,
            end_col: 0,
        }
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Token({:?}, {:?}, {}:{}, {}:{})",
            self.kind,
            String::from_iter(self.text),
            self.beg_row,
            self.beg_col,
            self.end_row,
            self.end_col
        )
    }
}
