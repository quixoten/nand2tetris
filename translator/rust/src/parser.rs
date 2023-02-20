use core::cell::Cell;
use std::path::PathBuf;

use crate::emitter::Emitter;
use crate::lexer::Lexer;
use crate::tokens::{Token, TokenKind};

pub const SP: usize = 0;
pub const LCL: usize = 1;
pub const ARG: usize = 2;
pub const THIS: usize = 3;
pub const THAT: usize = 4;

pub struct Parser<'a> {
    position: Cell<usize>,
    tokens: Vec<Token<'a>>,
    emitter: &'a Emitter,
}

impl<'a> Parser<'a> {
    fn look_ahead(&self, n: usize) -> &Token {
        &self.tokens[self.position.get() + n]
    }

    fn look_back(&self, n: usize) -> &Token {
        &self.tokens[self.position.get() - n]
    }

    fn current_token(&self) -> &Token {
        self.look_ahead(0)
    }

    fn current_token_kind(&self) -> TokenKind {
        self.current_token().kind.clone()
    }

    fn consume_token(&self) {
        //println!("consuming token: {:?}", self.current_token());
        self.position.set(self.position.get() + 1);
    }

    fn consume_comment_tokens(&self) {
        while let TokenKind::Comment = self.current_token_kind() {
            self.consume_token();
        }
    }

    fn consume_eol_and_comment_tokens(&self) {
        while let TokenKind::EndOfLine | TokenKind::Comment = self.current_token_kind() {
            self.consume_token();
        }
    }

    fn consume_eol_or_panic(&self) {
        let prev_token = self.look_back(1);

        // ignore comments at the end of the line
        self.consume_comment_tokens();

        let token = self.current_token();

        if token.kind != TokenKind::EndOfLine {
            panic!(
                "expected a newline token after {:?} but got {:?}",
                prev_token, token
            );
        }

        self.consume_token();
    }

    fn generate_argument_push(&self, number: usize) {
        self.emitter.emit_segment_push(number, ARG)
    }

    fn generate_argument_pop(&self, number: usize) {
        self.emitter.emit_segment_pop(number, ARG);
    }

    fn generate_constant_push(&self, number: usize) {
        self.emitter.emit_constant_push(number);
    }

    fn generate_local_push(&self, number: usize) {
        self.emitter.emit_segment_push(number, LCL)
    }

    fn generate_local_pop(&self, number: usize) {
        self.emitter.emit_segment_pop(number, LCL);
    }

    fn generate_pointer_push(&self, number: usize) {
        if number == 0 {
            self.emitter.emit_pointer_push(number, THIS);
        } else if number == 1 {
            self.emitter.emit_pointer_push(number, THAT);
        } else {
            panic!(
                "{number} is not a valid number for the 'push pointer' instruction: {:?}",
                self.current_token()
            );
        }
    }

    fn generate_pointer_pop(&self, number: usize) {
        if number == 0 {
            self.emitter.emit_pointer_pop(number, THIS);
        } else if number == 1 {
            self.emitter.emit_pointer_pop(number, THAT);
        } else {
            panic!(
                "{number} is not a valid number for the 'pop pointer' instruction: {:?}",
                self.current_token()
            );
        }
    }

    fn generate_static_push(&self, number: usize) {
        self.emitter.emit_static_push(number);
    }

    fn generate_static_pop(&self, number: usize) {
        self.emitter.emit_static_pop(number);
    }

    fn generate_temp_push(&self, number: usize) {
        self.emitter.emit_temp_push(number);
    }

    fn generate_temp_pop(&self, number: usize) {
        self.emitter.emit_temp_pop(number);
    }

    fn generate_that_push(&self, number: usize) {
        self.emitter.emit_segment_push(number, THAT)
    }

    fn generate_that_pop(&self, number: usize) {
        self.emitter.emit_segment_pop(number, THAT);
    }

    fn generate_this_push(&self, number: usize) {
        self.emitter.emit_segment_push(number, THIS)
    }

    fn generate_this_pop(&self, number: usize) {
        self.emitter.emit_segment_pop(number, THIS);
    }

    fn generate_push(&self) {
        let push = &self.current_token();
        let segment = &self.look_ahead(1);
        let number = &self.look_ahead(2);

        if !segment.kind.is_valid_push_segment() {
            panic!(
                "expected a segment token after {:?} but got {:?}",
                push, segment
            );
        }

        if number.kind != TokenKind::Number {
            panic!(
                "expected a number token after {:?} but got {:?}",
                segment, number
            );
        }

        let number_str = String::from_iter(number.text);
        let number_usize: usize = number_str.parse().unwrap_or_else(|_| {
            panic!(
                "{} is an invalid location to pop from: {:?}",
                number_str, number
            )
        });
        match segment.kind {
            TokenKind::Argument => self.generate_argument_push(number_usize),
            TokenKind::Constant => self.generate_constant_push(number_usize),
            TokenKind::Local => self.generate_local_push(number_usize),
            TokenKind::Static => self.generate_static_push(number_usize),
            TokenKind::Temp => self.generate_temp_push(number_usize),
            TokenKind::That => self.generate_that_push(number_usize),
            TokenKind::This => self.generate_this_push(number_usize),
            TokenKind::Pointer => self.generate_pointer_push(number_usize),
            _ => panic!("do not know how to build segment kind {:?}", segment.kind),
        }

        self.consume_token(); // push
        self.consume_token(); // constant
        self.consume_token(); // <number>
        self.consume_eol_or_panic();
    }

    fn generate_pop(&self) {
        let pop = &self.current_token();
        let segment = &self.look_ahead(1);
        let number = &self.look_ahead(2);

        if !segment.kind.is_valid_pop_segment() {
            panic!(
                "expected a segment token after {:?} but got {:?}",
                pop, segment
            );
        }

        if number.kind != TokenKind::Number {
            panic!(
                "expected a number token after {:?} but got {:?}",
                segment, number
            );
        }

        let number_str = String::from_iter(number.text);
        let number_usize: usize = number_str.parse().unwrap_or_else(|_| {
            panic!(
                "{} is an invalid location to pop from: {:?}",
                number_str, number
            )
        });

        match segment.kind {
            TokenKind::Argument => self.generate_argument_pop(number_usize),
            TokenKind::Local => self.generate_local_pop(number_usize),
            TokenKind::Pointer => self.generate_pointer_pop(number_usize),
            TokenKind::Static => self.generate_static_pop(number_usize),
            TokenKind::Temp => self.generate_temp_pop(number_usize),
            TokenKind::That => self.generate_that_pop(number_usize),
            TokenKind::This => self.generate_this_pop(number_usize),
            _ => panic!("do not know how to build segment kind {:?}", segment.kind),
        }

        self.consume_token(); // pop
        self.consume_token(); // constant
        self.consume_token(); // <number>
        self.consume_eol_or_panic();
    }

    fn generate_add(&self) {
        self.emitter.emit_add();
        self.consume_token();
        self.consume_eol_or_panic();
    }

    fn generate_eq(&self) {
        self.emitter.emit_eq();
        self.consume_token();
        self.consume_eol_or_panic();
    }

    fn generate_gt(&self) {
        self.emitter.emit_gt();
        self.consume_token();
        self.consume_eol_or_panic();
    }

    fn generate_lt(&self) {
        self.emitter.emit_lt();
        self.consume_token();
        self.consume_eol_or_panic();
    }

    fn generate_and(&self) {
        self.emitter.emit_and();
        self.consume_token();
        self.consume_eol_or_panic();
    }

    fn generate_or(&self) {
        self.emitter.emit_or();
        self.consume_token();
        self.consume_eol_or_panic();
    }

    fn generate_neg(&self) {
        self.emitter.emit_neg();
        self.consume_token();
        self.consume_eol_or_panic();
    }

    fn generate_not(&self) {
        self.emitter.emit_not();
        self.consume_token();
        self.consume_eol_or_panic();
    }

    fn generate_sub(&self) {
        self.emitter.emit_sub();
        self.consume_token();
        self.consume_eol_or_panic();
    }

    fn generate_label(&self) {
        let label = &self.current_token();
        let label_name = &self.look_ahead(1);

        if label_name.kind != TokenKind::Identifier {
            panic!(
                "expected an identifier after {:?} but found {:?}",
                label, label_name
            );
        }

        let label_name = String::from_iter(label_name.text);
        self.emitter.emit_label(label_name);

        self.consume_token(); // label
        self.consume_token(); // label name
        self.consume_eol_or_panic();
    }

    fn generate_goto(&self) {
        let goto = &self.current_token();
        let label_name = &self.look_ahead(1);

        if label_name.kind != TokenKind::Identifier {
            panic!(
                "expected an identifier after {:?} but found {:?}",
                goto, label_name
            );
        }

        let label_name_text = String::from_iter(label_name.text);
        self.emitter.emit_goto(label_name_text);

        self.consume_token(); // goto
        self.consume_token(); // label name
        self.consume_eol_or_panic();
    }

    fn generate_ifgoto(&self) {
        let ifgoto = &self.current_token();
        let label_name = &self.look_ahead(1);

        if label_name.kind != TokenKind::Identifier {
            panic!(
                "expected an identifier after {:?} but found {:?}",
                ifgoto, label_name
            );
        }

        let label_name = String::from_iter(label_name.text);
        self.emitter.emit_ifgoto(label_name);

        self.consume_token(); // if-goto
        self.consume_token(); // label name
        self.consume_eol_or_panic();
    }

    fn generate_function(&self) {
        let name = &self.look_ahead(1);
        let n_vars = &self.look_ahead(2);

        if name.kind != TokenKind::Identifier {
            panic!(
                "expected an identifier after \"function\" but found {:?}",
                name
            );
        }

        let name = String::from_iter(name.text);

        if n_vars.kind != TokenKind::Number {
            panic!(
                "expected a number after \"function {name}\" but found {:?}",
                n_vars
            );
        }

        let n_vars_text = String::from_iter(n_vars.text);
        let n_vars_usize: usize = n_vars_text.parse().unwrap_or_else(|_| {
            panic!("Cannot parse the variable count in \"function {name} {n_vars_text}\"")
        });

        self.emitter.emit_function(name.clone(), n_vars_usize);
        self.emitter.set_func_name(name);

        self.consume_token(); // function
        self.consume_token(); // function name
        self.consume_token(); // n vars
        self.consume_eol_or_panic();
    }

    fn generate_call(&self) {
        //let call = &self.current_token();
        let function = &self.look_ahead(1);
        let n_args = &self.look_ahead(2);

        if function.kind != TokenKind::Identifier {
            panic!(
                "expected an identifier after \"function\" but found {:?}",
                function
            );
        }

        let function_name = String::from_iter(function.text);

        if n_args.kind != TokenKind::Number {
            panic!(
                "expected an identifier after \"function {function_name}\" but found {:?}",
                n_args
            );
        }

        let n_args_text = String::from_iter(n_args.text);
        let n_args_usize: usize = n_args_text.parse().unwrap_or_else(|_| {
            panic!(
                "Cannot parse the arg count in \"function {function_name} {n_args_text}\": {:?}",
                n_args
            )
        });

        self.emitter.emit_call(function_name, n_args_usize);


        self.consume_token(); // call
        self.consume_token(); // function name
        self.consume_token(); // arg count
        self.consume_eol_or_panic();
    }

    fn generate_return(&self) {
        self.emitter.emit_return();
        self.consume_token();
        self.consume_eol_or_panic();
    }

    pub fn generate_assembly(&self) {
        // ignore comments and newlines at the start of the file
        self.consume_eol_and_comment_tokens();

        loop {
            match self.current_token_kind() {
                TokenKind::Add => self.generate_add(),
                TokenKind::And => self.generate_and(),
                TokenKind::Eq => self.generate_eq(),
                TokenKind::Goto => self.generate_goto(),
                TokenKind::Gt => self.generate_gt(),
                TokenKind::IfGoto => self.generate_ifgoto(),
                TokenKind::Function => self.generate_function(),
                TokenKind::Call => self.generate_call(),
                TokenKind::Return => self.generate_return(),
                TokenKind::Label => self.generate_label(),
                TokenKind::Lt => self.generate_lt(),
                TokenKind::Neg => self.generate_neg(),
                TokenKind::Not => self.generate_not(),
                TokenKind::Or => self.generate_or(),
                TokenKind::Pop => self.generate_pop(),
                TokenKind::Push => self.generate_push(),
                TokenKind::Sub => self.generate_sub(),
                TokenKind::Comment => self.consume_token(),
                TokenKind::EndOfLine => self.consume_token(),
                TokenKind::EndOfFile => break,
                _ => panic!("Unsupported token {:?}", self.current_token()),
            }
        }
    }

    fn parse_file(source_path: PathBuf, emitter: &mut Emitter) {
        let source = std::fs::read_to_string(&source_path).unwrap();
        let file_name = source_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let lexer = Lexer::new(&source);
        let tokens = lexer.list_all_tokens();
        let parser = Parser {
            emitter,
            position: Cell::new(0),
            tokens,
        };
        emitter.set_file_name(file_name);
        parser.generate_assembly();
    }

    fn parse_dir(source_dir: PathBuf, emitter: &mut Emitter) {
        emitter.emit_bootstrap();

        for path in source_dir.read_dir().unwrap() {
            let path = path.unwrap().path();

            if path.is_file() && path.extension().unwrap() == "vm" {
                Self::parse_file(path, emitter);
            }
        }
    }

    pub fn parse(source_path: PathBuf) {
        if source_path.is_dir() {
            let source_stem = source_path.file_name().unwrap().to_str().unwrap().to_string();
            let dest_path = source_path.join(source_stem).with_extension("asm");
            let mut emitter = Emitter::new(dest_path);

            Self::parse_dir(source_path, &mut emitter);
        } else if source_path.extension().unwrap() == "vm" {
            let dest_path = source_path.with_extension("asm");
            let mut emitter = Emitter::new(dest_path);

            Self::parse_file(source_path, &mut emitter);
        } else {
            panic!("Do not know how to parse \"{}\"", source_path.to_str().unwrap());
        }
    }
}
