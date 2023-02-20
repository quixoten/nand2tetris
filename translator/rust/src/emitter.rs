use core::cell::{Cell,RefCell};
use indoc::formatdoc;
use std::io::Write;
use std::path::PathBuf;

use crate::parser::{SP, LCL, ARG, THIS, THAT};

pub struct Emitter {
    dest: RefCell<std::fs::File>,
    file_name: RefCell<String>,
    func_name: RefCell<Option<String>>,
    ret_count: Cell<usize>,
    bool_count: Cell<usize>,
}

impl Emitter {
    pub fn new(dest_path: PathBuf) -> Emitter {
        let dest = RefCell::new(std::fs::File::create(dest_path).unwrap());
        let file_name = RefCell::new(String::from("Bootstrap"));
        let func_name = RefCell::new(None);
        let ret_count = Cell::new(0);
        let bool_count = Cell::new(0);

        Emitter {
            dest,
            file_name,
            func_name,
            ret_count,
            bool_count,
        }
    }

    pub fn set_file_name(&self, file_name: String) {
        self.file_name.replace(file_name);
        self.func_name.replace(None);
    }

    pub fn set_func_name(&self, func_name: String) {
        self.func_name.replace(Some(func_name));
    }

    fn label_prefix(&self) -> String {
        if let Some(func_name) = &*self.func_name.borrow() {
            func_name.to_string()
        } else {
            self.file_name.borrow().to_string()
        }
    }

    fn segment_addr_to_name(segment_addr: usize) -> String {
        match segment_addr {
            ARG => "argument".to_string(),
            LCL => "local".to_string(),
            THAT => "that".to_string(),
            THIS => "this".to_string(),
            _ => "unknown".to_string(),
        }
    }

    pub fn write(&self, content: String) {
        let mut dest = self.dest.borrow_mut();
        dest.write_all(content.as_bytes()).unwrap();
    }

    pub fn writeln(&self, content: &str) {
        let mut dest = self.dest.borrow_mut();
        dest.write_all(content.as_bytes()).unwrap();
        dest.write_all(&[10u8]).unwrap();
    }

    pub fn emit_bootstrap(&self) {
        self.write(formatdoc!("

            // Bootstrap
            @256
            D=A
            @SP
            M=D
        "));

        self.emit_call(String::from("Sys.init"), 0);
    }

    pub fn emit_add(&self) {
        self.write(formatdoc!("

            // add
            @{SP}
            AM=M-1
            D=M
            @{SP}
            A=M-1
            M=D+M
        "));
    }

    pub fn emit_and(&self) {
        self.write(formatdoc!("

            // and
            @{SP}
            AM=M-1
            D=M
            @{SP}
            A=M-1
            M=D&M
        "));
    }

    pub fn emit_eq(&self) {
        let bool_id = self.bool_count.replace(self.bool_count.get() + 1);
        let label_prefix = self.label_prefix();

        self.write(formatdoc!("

            // eq
            @{SP}
            AM=M-1
            D=M
            @{SP}
            A=M-1
            D=M-D
            @{label_prefix}-eq-true.{bool_id}
            D;JEQ
            @{label_prefix}-eq-done.{bool_id}
            D=0;JMP // false
            ({label_prefix}-eq-true.{bool_id})
            D=-1    // true
            ({label_prefix}-eq-done.{bool_id})
            @{SP}
            A=M-1
            M=D
        "));
    }

    pub fn emit_gt(&self) {
        let bool_id = self.bool_count.replace(self.bool_count.get() + 1);
        let label_prefix = self.label_prefix();

        self.write(formatdoc!("

            // gt
            @{SP}
            AM=M-1
            D=M
            @{SP}
            A=M-1
            D=M-D
            @{label_prefix}-gt-true.{bool_id}
            D;JGT
            @{label_prefix}-gt-done.{bool_id}
            D=0;JMP // false
            ({label_prefix}-gt-true.{bool_id})
            D=-1    // true
            ({label_prefix}-gt-done.{bool_id})
            @{SP}
            A=M-1
            M=D
        "));
    }

    pub fn emit_lt(&self) {
        let bool_id = self.bool_count.replace(self.bool_count.get() + 1);
        let label_prefix = self.label_prefix();

        self.write(formatdoc!("

            // lt
            @{SP}
            AM=M-1
            D=M
            @{SP}
            A=M-1
            D=M-D
            @{label_prefix}-lt-true.{bool_id}
            D;JLT
            @{label_prefix}-lt-done.{bool_id}
            D=0;JMP // false
            ({label_prefix}-lt-true.{bool_id})
            D=-1    // true
            ({label_prefix}-lt-done.{bool_id})
            @{SP}
            A=M-1
            M=D
        "));
    }

    pub fn emit_neg(&self) {
        self.write(formatdoc!("

            // neg
            @{SP}
            A=M-1
            M=-M
        "));
    }

    pub fn emit_not(&self) {
        self.write(formatdoc!("

            // not
            @{SP}
            A=M-1
            M=!M
        "));
    }

    pub fn emit_or(&self) {
        self.write(formatdoc!("

            // or
            @{SP}
            AM=M-1
            D=M
            @{SP}
            A=M-1
            M=D|M
        "));
    }

    pub fn emit_sub(&self) {
        self.write(formatdoc!("

            // sub
            @{SP}
            AM=M-1
            D=M
            @{SP}
            A=M-1
            M=M-D
        "));
    }


    pub fn emit_call(&self, function_name: String, n_args: usize) {
        let args_offset = n_args + 5;
        let ret_id = self.ret_count.replace(self.ret_count.get() + 1);

        self.write(formatdoc!("

            // call {function_name} {n_args}
            @{function_name}$ret.{ret_id}    // push the return addr onto the stack
            D=A
            @{SP}
            A=M
            M=D
            @{SP}
            M=M+1
            @{LCL}   // push the LCL value onto the stack
            D=M
            @{SP}
            A=M
            M=D
            @{SP}
            M=M+1
            @{ARG}   // push the ARG value onto the stack
            D=M
            @{SP}
            A=M
            M=D
            @{SP}
            M=M+1
            @{THIS}  // push the THIS value onto the stack
            D=M
            @{SP}
            A=M
            M=D
            @{SP}
            M=M+1
            @{THAT}  // push the THAT value onto the stack
            D=M
            @{SP}
            A=M
            M=D
            @{SP}
            MD=M+1
            @{args_offset} // Re-position ARG
            D=D-A
            @{ARG}
            M=D
            @{SP}    // set LCL=SP
            D=M 
            @{LCL}
            M=D
            @{function_name}
            0;JMP
            ({function_name}$ret.{ret_id})
        "));
    }

    pub fn emit_return(&self) {
        self.write(formatdoc!("

            // return
            @{LCL}    // copy the return address to R13: *(RAM[LCL] - 5)
            D=M
            @5
            A=D-A
            D=M
            @R13
            M=D
            @{SP}     // pop the return value into arg 0
            AM=M-1
            D=M
            @{ARG}
            A=M
            M=D
            D=A+1     // move the stack pointer to arg + 1
            @{SP}
            M=D
            @{LCL}    // restore the caller's THAT address: *(RAM[LCL] - 1)
            AM=M-1
            D=M
            @{THAT}
            M=D
            @{LCL}    // restore the callers' THIS address: *(RAM[LCL] - 2)
            AM=M-1
            D=M
            @{THIS}
            M=D
            @{LCL}    // restore the callers'  ARG address: *(RAM[LCL] - 3)
            AM=M-1
            D=M
            @{ARG}
            M=D
            @{LCL}    // restore the callers'  LCL address: *(RAM[LCL] - 4)
            AM=M-1
            D=M
            @{LCL}
            M=D
            @R13      // jump to the return address
            A=M;JMP
        "));
    }

    pub fn emit_function(&self, name: String, n_vars: usize) {
        self.writeln(&format!("\n// function {name} {n_vars}"));
        self.writeln(&format!("({name})"));

        // initialize the local variables to 0
        if n_vars > 0 {
            self.writeln(&format!("@{SP}"));
            self.writeln("A=M");
            for _ in 0..n_vars {
                self.writeln("M=0");
                self.writeln("A=A+1");
            }
            self.writeln("D=A");
            self.writeln(&format!("@{SP}"));
            self.writeln("M=D");
        }
    }

    pub fn emit_goto(&self, label_name: String) {
        let label_prefix = self.label_prefix();

        self.write(formatdoc!("

            // goto {label_name}
            @{label_prefix}${label_name} 
            0;JMP
        "));
    }

    pub fn emit_ifgoto(&self, label_name: String) {
        let label_prefix = self.label_prefix();

        self.write(formatdoc!("

            // if-goto {label_name}
            @{SP}
            AM=M-1
            D=M
            @{label_prefix}${label_name} 
            D;JNE
        "));
    }

    pub fn emit_label(&self, label_name: String) {
        let label_prefix = self.label_prefix();

        self.write(formatdoc!("

            // label {label_name}
            ({label_prefix}${label_name}) 
        "));
    }

    pub fn emit_constant_push(&self, number: usize) {
        self.write(formatdoc!("

            // push constant {number}
            @{number}
            D=A
            @{SP}
            A=M
            M=D
            @{SP}
            M=M+1
        "));
    }

    pub fn emit_pointer_push(&self, number: usize, segment_addr: usize) {
        self.write(formatdoc!("

            // push pointer {number}
            @{segment_addr}
            D=M
            @{SP}
            A=M
            M=D
            @{SP}
            M=M+1
        "));
    }

    pub fn emit_pointer_pop(&self, number: usize, segment_addr: usize) {
        self.write(formatdoc!("

            // pop pointer {number}
            @{SP}
            AM=M-1
            D=M
            @{segment_addr}
            M=D
        "));
    }

    pub fn emit_segment_push(&self, number: usize, segment_addr: usize) {
        let segment_name = Self::segment_addr_to_name(segment_addr);
        self.write(formatdoc!("

            // push {segment_name} {number}
            @{segment_addr}
            D=M
            @{number}
            A=D+A
            D=M
            @{SP}
            A=M
            M=D
            @{SP}
            M=M+1
        "));
    }

    pub fn emit_segment_pop(&self, number: usize, segment_addr: usize) {
        // could be simplified for number = 0 and number = 1
        let segment_name = Self::segment_addr_to_name(segment_addr);

        self.write(formatdoc!("

            // pop {segment_name} {number}
            @{number}
            D=A
            @{segment_addr}
            D=D+M
            @R13
            M=D
            @{SP}
            AM=M-1
            D=M
            @R13
            A=M
            M=D
        "));
    }

    pub fn emit_static_push(&self, number: usize) {
        let file_name = &self.file_name.borrow();

        self.write(formatdoc!("
            // push static {number}
            @{file_name}.{number}
            D=M
            @{SP}
            A=M
            M=D
            @{SP}
            M=M+1
        "));
    }

    pub fn emit_static_pop(&self, number: usize) {
        let file_name = &self.file_name.borrow();

        self.write(formatdoc!("
            // pop static {number}
            @{SP}
            AM=M-1
            D=M
            @{file_name}.{number}
            M=D
        "));
    }

    pub fn emit_temp_push(&self, number: usize) {
        let temp_addr = 5 + number;

        self.write(formatdoc!("

            // push temp {number}
            @{temp_addr}
            D=M
            @{SP}
            A=M
            M=D
            @{SP}
            M=M+1
        "));
    }

    pub fn emit_temp_pop(&self, number: usize) {
        let temp_addr = 5 + number;

        self.write(formatdoc!("

            // pop temp {number}
            @{SP}
            AM=M-1
            D=M
            @{temp_addr}
            M=D
        "));
    }
}
