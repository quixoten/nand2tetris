//#![allow(dead_code)]

use std::{
    collections::HashMap,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
};

struct Lexer {
    bytes: Vec<u8>,
    position: usize,
    state: Box<dyn LexerState>,
    tx: SyncSender<Token>,
    rx: Receiver<Token>,
    row: usize,
    col: usize,
}

impl Lexer {
    fn from_str(s: &str) -> Self {
        let (tx, rx) = sync_channel(2);

        Self {
            bytes: Vec::from(s.as_bytes()),
            position: 0,
            state: Box::new(LexInstructionStart),
            tx,
            rx,
            row: 1,
            col: 1,
        }
    }

    fn emit(&self, token: Token) {
        self.tx.send(token).unwrap();
    }

    fn current_byte(&self) -> Option<u8> {
        if self.position < self.bytes.len() {
            Some(self.bytes[self.position])
        } else {
            None
        }
    }

    fn current_char(&self) -> Option<char> {
        match self.current_byte() {
            Some(byte) => Some(byte as char),
            _ => None,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        loop {
            if let Ok(token) = self.rx.try_recv() {
                // println!("returning token: {token:?}");
                return Some(token);
            }

            let state = std::mem::replace(&mut self.state, Box::new(LexInstructionStart));

            if let Some(next_state) = state.parse(self) {
                self.state = next_state;
                continue;
            }

            return None;
        }
    }

    fn advance(&mut self, n: usize) -> bool {
        let new_position = self.position + n;

        if new_position > self.bytes.len() {
            false
        } else {
            while self.position < new_position {
                if let Some(b'\n') = self.current_byte() {
                    self.row += 1;
                    self.col = 1;
                } else {
                    self.col += 1;
                }

                self.position += 1;
            }

            true
        }
    }

    fn accept_str(&mut self, s: &str) -> bool {
        let start = self.position;
        let end = self.position + s.len();

        if let Some(slice) = self.bytes.get(start..end) {
            if slice == s.as_bytes() {
                return self.advance(s.len());
            }
        }

        false
    }

    fn accept_any(&mut self) -> bool {
        match self.current_byte() {
            Some(_) => self.advance(1),
            _ => false,
        }
    }

    fn accept_whitespace(&mut self) -> bool {
        match self.current_byte() {
            Some(b' ' | b'\t' | b'\r' | b'\n') => self.advance(1),
            _ => false,
        }
    }

    fn accept_non_eol_whitespace(&mut self) -> bool {
        match self.current_byte() {
            Some(b' ' | b'\r' | b'\t') => self.advance(1),
            _ => false,
        }
    }

    fn accept_eol(&mut self) -> bool {
        match self.current_byte() {
            Some(b'\n') => self.advance(1),
            _ => false,
        }
    }

    fn accept_label(&mut self) -> Option<String> {
        let mut label = String::new();

        // labels can start with a letter, underscore, or dollar sign
        match self.current_byte() {
            Some(b'A'..=b'Z' | b'a'..=b'z' | b'_' | b'$') => (),
            _ => return None,
        };

        while let Some(
            byte @ b'A'..=b'Z'
            | byte @ b'a'..=b'z'
            | byte @ b'0'..=b'9'
            | byte @ b'_'
            | byte @ b'.'
            | byte @ b'$'
            | byte @ b'-',
        ) = self.current_byte()
        {
            label.push(byte as char);
            self.advance(1);
        }

        if label.len() > 0 {
            Some(label)
        } else {
            None
        }
    }

    fn accept_addr(&mut self) -> Option<usize> {
        let mut number = String::new();

        while let Some(byte @ b'0'..=b'9') = self.current_byte() {
            number.push(byte as char);
            self.advance(1);
        }

        if number.len() > 0 {
            number.parse::<usize>().ok()
        } else {
            None
        }
    }

    fn eof(&self) -> bool {
        !(self.position < self.bytes.len())
    }
}

#[derive(Debug)]
pub enum Instruction {
    Addr(AddrToken),
    Comp(DestToken, CompToken, JumpToken),
}
impl Instruction {
    fn to_binary(&self) -> Vec<u8> {
        match self {
            Instruction::Addr(addr) => match addr {
                AddrToken::Static(addr) => format!("0{:015b}\n", addr).into_bytes(),
                AddrToken::Dynamic(_) => panic!("should not call to_binary on dynamic addr"),
            },

            Instruction::Comp(dest, comp, jump) => format!(
                "111{:07b}{:03b}{:03b}\n",
                comp.code(),
                dest.code(),
                jump.code()
            )
            .into_bytes(),
        }
    }
}

#[derive(Debug)]
enum Token {
    Dest(DestToken),
    Comp(CompToken),
    Jump(JumpToken),
    Addr(AddrToken),
    Label(String),
}

#[derive(Debug)]
pub enum DestToken {
    A,
    AD,
    ADM,
    AM,
    D,
    Empty,
    M,
    MD,
}
impl DestToken {
    fn code(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::M => 1,
            Self::D => 2,
            Self::MD => 3,
            Self::A => 4,
            Self::AM => 5,
            Self::AD => 6,
            Self::ADM => 7,
        }
    }
}

#[derive(Debug)]
pub enum CompToken {
    Zero,
    One,
    A,
    D,
    M,
    AddA1,
    AddD1,
    AddDA,
    AddDM,
    AddM1,
    AndDA,
    AndDM,
    InvertA,
    InvertD,
    InvertM,
    Negate1,
    NegateA,
    NegateD,
    NegateM,
    OrDA,
    OrDM,
    SubA1,
    SubAD,
    SubD1,
    SubDA,
    SubDM,
    SubM1,
    SubMD,
}
impl CompToken {
    fn code(&self) -> usize {
        match self {
            Self::Zero => 0b0101010,
            Self::One => 0b0111111,
            Self::A => 0b0110000,
            Self::D => 0b0001100,
            Self::M => 0b1110000,
            Self::AddA1 => 0b0110111,
            Self::AddD1 => 0b0011111,
            Self::AddDA => 0b0000010,
            Self::AddDM => 0b1000010,
            Self::AddM1 => 0b1110111,
            Self::AndDA => 0b0000000,
            Self::AndDM => 0b1000000,
            Self::InvertA => 0b0110001,
            Self::InvertD => 0b0001101,
            Self::InvertM => 0b1110001,
            Self::Negate1 => 0b0111010,
            Self::NegateA => 0b0110011,
            Self::NegateD => 0b0001111,
            Self::NegateM => 0b1110011,
            Self::OrDA => 0b0010101,
            Self::OrDM => 0b1010101,
            Self::SubA1 => 0b0110010,
            Self::SubAD => 0b0000111,
            Self::SubD1 => 0b0001110,
            Self::SubDA => 0b0010011,
            Self::SubDM => 0b1010011,
            Self::SubM1 => 0b1110010,
            Self::SubMD => 0b1000111,
        }
    }
}

#[derive(Debug)]
pub enum JumpToken {
    JEQ,
    JGE,
    JGT,
    JLE,
    JLT,
    JMP,
    JNE,
    Empty,
}
impl JumpToken {
    fn code(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::JGT => 1,
            Self::JEQ => 2,
            Self::JGE => 3,
            Self::JLT => 4,
            Self::JNE => 5,
            Self::JLE => 6,
            Self::JMP => 7,
        }
    }
}

#[derive(Debug)]
pub enum AddrToken {
    Static(usize),
    Dynamic(String),
}

impl std::fmt::Debug for dyn LexerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.name())
    }
}

trait LexerState {
    fn parse(&self, lexer: &mut Lexer) -> Option<Box<dyn LexerState>>;
    fn name(&self) -> String {
        std::any::type_name::<Self>().into()
    }
}

struct LexInstructionStart;
impl LexerState for LexInstructionStart {
    fn parse(&self, lexer: &mut Lexer) -> Option<Box<dyn LexerState>> {
        // println!("in {}", self.name());
        match () {
            _ if lexer.accept_whitespace() => Some(Box::new(Self)),
            _ if lexer.accept_str("//") => Some(Box::new(LexComment)),
            _ if lexer.accept_str("@") => Some(Box::new(LexAddress)),
            _ if lexer.accept_str("(") => Some(Box::new(LexLabel)),
            _ if lexer.eof() => None,
            _ => Some(Box::new(LexDest)),
        }
    }
}

struct LexComment;
impl LexerState for LexComment {
    fn parse(&self, lexer: &mut Lexer) -> Option<Box<dyn LexerState>> {
        // println!("in {}", self.name());
        match () {
            _ if lexer.accept_eol() => Some(Box::new(LexInstructionStart)),
            _ if lexer.accept_any() => Some(Box::new(Self)),
            _ => None,
        }
    }
}

struct LexAddress;
impl LexerState for LexAddress {
    fn parse(&self, lexer: &mut Lexer) -> Option<Box<dyn LexerState>> {
        // println!("in {}", self.name());

        if let Some(label) = lexer.accept_label() {
            lexer.emit(Token::Addr(AddrToken::Dynamic(label)));
        } else if let Some(addr) = lexer.accept_addr() {
            lexer.emit(Token::Addr(AddrToken::Static(addr)));
        }

        Some(Box::new(LexInstructionEnd))
    }
}

struct LexLabel;
impl LexerState for LexLabel {
    fn parse(&self, lexer: &mut Lexer) -> Option<Box<dyn LexerState>> {
        // println!("in {}", self.name());
        if let Some(label) = lexer.accept_label() {
            lexer.emit(Token::Label(label));
            Some(Box::new(LexLabelEnd))
        } else {
            panic!(
                "invalid character in label at line {}, col {}",
                lexer.row, lexer.col
            );
        }
    }
}

struct LexLabelEnd;
impl LexerState for LexLabelEnd {
    fn parse(&self, lexer: &mut Lexer) -> Option<Box<dyn LexerState>> {
        // println!("in {}", self.name());
        match () {
            _ if lexer.accept_str(")") => Some(Box::new(LexInstructionEnd)),
            _ => panic!(
                "invalid character in label at line {}, col {}",
                lexer.row, lexer.col
            ),
        }
    }
}

struct LexDest;
impl LexerState for LexDest {
    fn parse(&self, lexer: &mut Lexer) -> Option<Box<dyn LexerState>> {
        // println!("in {}", self.name());

        match () {
            _ if lexer.accept_str("A=") => lexer.emit(Token::Dest(DestToken::A)),
            _ if lexer.accept_str("AD=") => lexer.emit(Token::Dest(DestToken::AD)),
            _ if lexer.accept_str("ADM=") => lexer.emit(Token::Dest(DestToken::ADM)),
            _ if lexer.accept_str("AM=") => lexer.emit(Token::Dest(DestToken::AM)),
            _ if lexer.accept_str("D=") => lexer.emit(Token::Dest(DestToken::D)),
            _ if lexer.accept_str("MD=") => lexer.emit(Token::Dest(DestToken::MD)),
            _ if lexer.accept_str("M=") => lexer.emit(Token::Dest(DestToken::M)),
            _ => lexer.emit(Token::Dest(DestToken::Empty)),
        };

        Some(Box::new(LexComp))
    }
}

struct LexComp;
impl LexerState for LexComp {
    fn parse(&self, lexer: &mut Lexer) -> Option<Box<dyn LexerState>> {
        // println!("in {}", self.name());
        match () {
            _ if lexer.accept_str("A+1") => lexer.emit(Token::Comp(CompToken::AddA1)),
            _ if lexer.accept_str("D+1") => lexer.emit(Token::Comp(CompToken::AddD1)),
            _ if lexer.accept_str("D+A") => lexer.emit(Token::Comp(CompToken::AddDA)),
            _ if lexer.accept_str("D+M") => lexer.emit(Token::Comp(CompToken::AddDM)),
            _ if lexer.accept_str("M+1") => lexer.emit(Token::Comp(CompToken::AddM1)),
            _ if lexer.accept_str("D+A") => lexer.emit(Token::Comp(CompToken::AddDM)),
            _ if lexer.accept_str("A-1") => lexer.emit(Token::Comp(CompToken::SubA1)),
            _ if lexer.accept_str("A-D") => lexer.emit(Token::Comp(CompToken::SubAD)),
            _ if lexer.accept_str("D-1") => lexer.emit(Token::Comp(CompToken::SubD1)),
            _ if lexer.accept_str("D-A") => lexer.emit(Token::Comp(CompToken::SubDA)),
            _ if lexer.accept_str("D-M") => lexer.emit(Token::Comp(CompToken::SubDM)),
            _ if lexer.accept_str("M-1") => lexer.emit(Token::Comp(CompToken::SubM1)),
            _ if lexer.accept_str("M-D") => lexer.emit(Token::Comp(CompToken::SubMD)),
            _ if lexer.accept_str("D&A") => lexer.emit(Token::Comp(CompToken::AndDA)),
            _ if lexer.accept_str("D&M") => lexer.emit(Token::Comp(CompToken::AndDM)),
            _ if lexer.accept_str("D|A") => lexer.emit(Token::Comp(CompToken::OrDA)),
            _ if lexer.accept_str("D|M") => lexer.emit(Token::Comp(CompToken::OrDM)),
            _ if lexer.accept_str("!A") => lexer.emit(Token::Comp(CompToken::InvertA)),
            _ if lexer.accept_str("!D") => lexer.emit(Token::Comp(CompToken::InvertD)),
            _ if lexer.accept_str("!M") => lexer.emit(Token::Comp(CompToken::InvertM)),
            _ if lexer.accept_str("-1") => lexer.emit(Token::Comp(CompToken::Negate1)),
            _ if lexer.accept_str("-A") => lexer.emit(Token::Comp(CompToken::NegateA)),
            _ if lexer.accept_str("-D") => lexer.emit(Token::Comp(CompToken::NegateD)),
            _ if lexer.accept_str("-M") => lexer.emit(Token::Comp(CompToken::NegateM)),
            _ if lexer.accept_str("0") => lexer.emit(Token::Comp(CompToken::Zero)),
            _ if lexer.accept_str("1") => lexer.emit(Token::Comp(CompToken::One)),
            _ if lexer.accept_str("A") => lexer.emit(Token::Comp(CompToken::A)),
            _ if lexer.accept_str("D") => lexer.emit(Token::Comp(CompToken::D)),
            _ if lexer.accept_str("M") => lexer.emit(Token::Comp(CompToken::M)),

            _ => panic!(
                "invalid computation starting at line {}, col {}",
                lexer.row, lexer.col
            ),
        }

        Some(Box::new(LexAfterComp))
    }
}

struct LexAfterComp;
impl LexerState for LexAfterComp {
    fn parse(&self, lexer: &mut Lexer) -> Option<Box<dyn LexerState>> {
        // println!("in {}", self.name());
        match () {
            _ if lexer.accept_str(";") => Some(Box::new(LexJump)),
            _ => {
                lexer.emit(Token::Jump(JumpToken::Empty));
                Some(Box::new(LexInstructionEnd))
            }
        }
    }
}

struct LexJump;
impl LexerState for LexJump {
    fn parse(&self, lexer: &mut Lexer) -> Option<Box<dyn LexerState>> {
        // println!("in {}", self.name());
        match () {
            _ if lexer.accept_str("JGT") => lexer.emit(Token::Jump(JumpToken::JGT)),
            _ if lexer.accept_str("JEQ") => lexer.emit(Token::Jump(JumpToken::JEQ)),
            _ if lexer.accept_str("JGE") => lexer.emit(Token::Jump(JumpToken::JGE)),
            _ if lexer.accept_str("JLT") => lexer.emit(Token::Jump(JumpToken::JLT)),
            _ if lexer.accept_str("JNE") => lexer.emit(Token::Jump(JumpToken::JNE)),
            _ if lexer.accept_str("JLE") => lexer.emit(Token::Jump(JumpToken::JLE)),
            _ if lexer.accept_str("JMP") => lexer.emit(Token::Jump(JumpToken::JMP)),
            _ => panic!(
                "invalid jump starting at line {}, col {}",
                lexer.row, lexer.col
            ),
        }

        Some(Box::new(LexInstructionEnd))
    }
}

struct LexInstructionEnd;
impl LexerState for LexInstructionEnd {
    fn parse(&self, lexer: &mut Lexer) -> Option<Box<dyn LexerState>> {
        // println!("in {}", self.name());
        match () {
            _ if lexer.accept_str("//") => Some(Box::new(LexComment)),
            _ if lexer.accept_non_eol_whitespace() => Some(Box::new(Self)),
            _ if lexer.accept_eol() => Some(Box::new(LexInstructionStart)),
            _ if lexer.eof() => None,
            _ => panic!(
                "unexpected character after instruction: {:?} at line {}, col {}",
                lexer.current_char(),
                lexer.row,
                lexer.col
            ),
        }
    }
}

mod parser {
    use super::*;

    pub struct Parser {
        lexer: Lexer,
        symbols: HashMap<String, usize>,
        instructions: Vec<Instruction>,
    }
    impl Parser {
        pub fn from_str(s: &str) -> Self {
            Self {
                lexer: Lexer::from_str(s),
                symbols: HashMap::new(),
                instructions: vec![],
            }
            .add_default_symbols()
            .first_pass()
            .second_pass()
        }

        fn add_default_symbols(mut self) -> Self {
            self.symbols.insert("SP".to_string(), 0);
            self.symbols.insert("LCL".to_string(), 1);
            self.symbols.insert("ARG".to_string(), 2);
            self.symbols.insert("THIS".to_string(), 3);
            self.symbols.insert("THAT".to_string(), 4);
            self.symbols.insert("R0".to_string(), 0);
            self.symbols.insert("R1".to_string(), 1);
            self.symbols.insert("R2".to_string(), 2);
            self.symbols.insert("R3".to_string(), 3);
            self.symbols.insert("R4".to_string(), 4);
            self.symbols.insert("R5".to_string(), 5);
            self.symbols.insert("R6".to_string(), 6);
            self.symbols.insert("R7".to_string(), 7);
            self.symbols.insert("R8".to_string(), 8);
            self.symbols.insert("R9".to_string(), 9);
            self.symbols.insert("R10".to_string(), 10);
            self.symbols.insert("R11".to_string(), 11);
            self.symbols.insert("R12".to_string(), 12);
            self.symbols.insert("R13".to_string(), 13);
            self.symbols.insert("R14".to_string(), 14);
            self.symbols.insert("R15".to_string(), 15);
            self.symbols.insert("SCREEN".to_string(), 16384);
            self.symbols.insert("KBD".to_string(), 24576);
            self
        }

        fn first_pass(mut self) -> Self {
            loop {
                match self.lexer.next_token() {
                    Some(Token::Label(label)) => {
                        if let Some(_) = self.symbols.get(&label) {
                            panic!("the ({label}) label is defined twice");
                        } else {
                            self.symbols.insert(label, self.instructions.len());
                        }
                    }

                    Some(Token::Addr(token @ AddrToken::Dynamic(_))) => {
                        self.instructions.push(Instruction::Addr(token));
                    }

                    Some(Token::Addr(addr)) => self.instructions.push(Instruction::Addr(addr)),

                    Some(Token::Dest(dest)) => {
                        let comp = self.lexer.next_token();
                        let jump = self.lexer.next_token();

                        if let Some(Token::Comp(comp)) = comp {
                            if let Some(Token::Jump(jump)) = jump {
                                self.instructions.push(Instruction::Comp(dest, comp, jump));
                            } else {
                                panic!("expected jump token after {comp:?} but got {jump:?}");
                            }
                        } else {
                            panic!("expected comp token after {dest:?} but got {comp:?}");
                        }
                    }
                    Some(token) => {
                        panic!("unexpected token {token:?}");
                    }
                    None => break,
                }
            }

            self
        }

        fn second_pass(mut self) -> Self {
            let mut next_dynamic_address = 15;

            self.instructions = self
                .instructions
                .into_iter()
                .map(|instruction| match instruction {
                    Instruction::Addr(AddrToken::Dynamic(label)) => {
                        if let Some(&value) = self.symbols.get(&label) {
                            Instruction::Addr(AddrToken::Static(value))
                        } else {
                            next_dynamic_address += 1;
                            self.symbols.insert(label, next_dynamic_address);
                            Instruction::Addr(AddrToken::Static(next_dynamic_address))
                        }
                    }
                    _ => instruction,
                })
                .collect();

            self
        }
    }

    impl IntoIterator for Parser {
        type Item = Instruction;
        type IntoIter = std::vec::IntoIter<Self::Item>;
        fn into_iter(self) -> Self::IntoIter {
            self.instructions.into_iter()
        }
    }
}

use parser::Parser;

struct Generator<'a> {
    dest: Box<dyn std::io::Write>,
    source: &'a str,
}
impl<'a> Generator<'a> {
    fn from_str(dest: Box<dyn std::io::Write>, source: &'a str) -> Generator<'a> {
        Self { dest, source }
    }

    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let instructions = Parser::from_str(self.source).into_iter();

        for instruction in instructions {
            self.dest.write_all(instruction.to_binary().as_slice())?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("usage: {} <source>.asm", args[0])
    }

    let src_path = std::path::Path::new(&args[1]);
    let stem = src_path.file_stem().unwrap();
    let parent = src_path.parent().unwrap();
    let dst_path = parent.join(format!("{}.hack", stem.to_str().unwrap()));
    let src = std::fs::read_to_string(src_path)?;
    let dst = std::fs::File::create(dst_path.clone())?;

    let mut generator = Generator::from_str(Box::new(dst), src.as_str());

    eprintln!(
        "assembling {} into {}",
        src_path.to_string_lossy(),
        dst_path.to_string_lossy()
    );

    generator.run()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplest_instruction() -> Result<(), String> {
        let mut lex = Lexer::from_str("0");
        let dest = lex.next_token();
        let comp = lex.next_token();
        let jump = lex.next_token();
        let eof = lex.next_token();
        let actual = (dest, comp, jump, eof);

        match actual {
            (Some(Token::Dest(DestToken::Empty)), Some(Token::Comp(CompToken::Zero)), Some(Token::Jump(JumpToken::Empty)), None) => Ok(()),
            _ => Err(format!("expected (Some(Dest(Empty)), Some(Comp(Zero)), Some(Jump(Empty)), None) but got {:?}", actual)),
        }
    }
}
