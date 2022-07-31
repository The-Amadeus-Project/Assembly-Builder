use crate::return_code::{Code, Return};

const ASM_BASE_START: &str = r#"section .text
    global _start
"#;

const ASM_BASE_END: &str = r#"
    _start:
        call            main
        mov             rbx, rax

        mov             rax, 60                 ; system call for exit
        mov             rdi, rbx
        syscall
"#;

const ASM_BASE_BSS: &str = "section .bss\n";
const ASM_BASE_DATA: &str = "section .data\n";

pub struct Builder {
    text: String,
    bss: String,
    data: String,
    function_body: String,
    is_function_currently: bool,
    local_variables: Vec<u32>,
    local_variables_offset: Vec<u32>,
    local_offset: u32
}

impl Builder {
    pub fn new() -> Self {
        Self {
            text: ASM_BASE_START.to_string(),
            bss: ASM_BASE_BSS.to_string(),
            data: ASM_BASE_DATA.to_string(),
            function_body: "".to_string(),
            is_function_currently: false,
            local_variables: vec![],
            local_variables_offset: vec![],
            local_offset: 0
        }
    }
    pub fn new_program(start_function: &str) -> Self {
        Self {
            text: ASM_BASE_START.to_string().replace("_start", start_function),
            bss: ASM_BASE_BSS.to_string(),
            data: ASM_BASE_DATA.to_string(),
            function_body: "".to_string(),
            is_function_currently: false,
            local_variables: vec![],
            local_variables_offset: vec![],
            local_offset: 0
        }
    }
    fn add_line_text(&mut self, line: &str){
        self.text += &*format!("\t{}\n", line)
    }
    fn add_value_text(&mut self, line: &str){
        self.text += &*format!("\t\t{}\n", line)
    }
     fn add_line_function(&mut self, line: &str){
        self.function_body += &*format!("\t{}\n", line)
    }
    fn add_value_function(&mut self, line: &str){
        self.function_body += &*format!("\t\t{}\n", line)
    }
    fn add_line_bss(&mut self, line: &str){
        self.bss += &*format!("\t{}\n", line)
    }
    fn add_value_bss(&mut self, line: &str){
        self.bss += &*format!("\t\t{}\n", line)
    }
    fn add_line_data(&mut self, line: &str){
        self.data += &*format!("\t{}\n", line)
    }
    fn add_value_data(&mut self, line: &str){
        self.data += &*format!("\t\t{}\n", line)
    }

    pub fn add_built_in_function(&mut self, function: &str){
        self.text += function;
    }

    pub fn open_function(&mut self, function_name: &str) -> Return {
        if self.is_function_currently {
            return Return::new("Cannot Create Function Within a Function".to_string(), Code::FunctionWithinFunctionErr)
        }
        self.is_function_currently = true;
        self.function_body += &*format!("{}:\n", function_name);

        Return::new("Everything Is Fine".to_string(), Code::Good)
    }

    pub fn close_function(&mut self) -> Return{
        if !self.is_function_currently {
            return Return::new("Is not in a Function".to_string(), Code::ClosingOfNonFunctionErr)
        }
        self.local_offset = 0;
        self.local_variables = vec![];
        self.local_variables_offset = vec![];
        self.is_function_currently = false;
        self.add_value_function("ret");
        self.add_line_text(&*self.function_body.clone());
        self.function_body = "".to_string();
        Return::new("".to_string(), Code::Good)
    }

    pub fn new_local_word(&mut self, value: u16) -> Return {
        if !self.is_function_currently {
            return Return::new("cannot assign local variable not in function".to_string(), Code::LocalVariableNotInFunction)
        }
        let offset = self.local_offset + 2;
        self.local_offset += 2;
        self.local_variables.push(2);
        self.local_variables_offset.push(offset);
        self.mov(&*format!("word [rsp - {}]", offset), &*value.to_string());
        Return::new("".to_string(), Code::Good)
    }
    pub fn new_local_dword(&mut self, value: u32) -> Return {
        if !self.is_function_currently {
            return Return::new("cannot assign local variable not in function".to_string(), Code::LocalVariableNotInFunction)
        }
        let offset = self.local_offset + 4;
        self.local_offset += 4;
        self.local_variables.push(4);
        self.local_variables_offset.push(offset);
        self.mov(&*format!("dword [rsp - {}]", offset), &*value.to_string());
        Return::new("".to_string(), Code::Good)
    }
    pub fn new_local_qword(&mut self, value: u64) -> Return {
        if !self.is_function_currently {
            return Return::new("cannot assign local variable not in function".to_string(), Code::LocalVariableNotInFunction)
        }
        let offset = self.local_offset + 8;
        self.local_offset += 8;
        self.local_variables.push(8);
        self.local_variables_offset.push(offset);
        self.mov(&*format!("qword [rsp - {}]", offset), &*value.to_string());
        Return::new("".to_string(), Code::Good)
    }

    pub fn get_local_word_size_and_offset(&self, id: usize) -> (Return, u32, u32) {
        if id >= self.local_variables.len(){
            (Return::new("Out Of Range".to_string(), Code::OutOfRange), 0, 0)
        } else {
            let word_size: u32 = self.local_variables.get(id).unwrap().clone() as u32;
            let word_offset: u32 = self.local_variables_offset.get(id).unwrap().clone()  as u32;
            (Return::new("".to_string(), Code::Good), word_size, word_offset)
        }
    }

    pub fn new_len_addr(&mut self, new_addr: &str, from_addr: &str){
        self.add_line_data(&*format!(r#"{}: equ $ - {}"#, new_addr, from_addr));
    }

    pub fn new_string_literal(&mut self, addr: &str, value: &str){
        let mut new_string = format!(r#"{}: db "#, addr);
        let mut len = 0;
        for character in value.clone().chars() {
            new_string += &*(character as u8).to_string();
            len += 1;
            if len != value.len(){
                new_string += ", ";
            } else {
                new_string += ", 0";
            }
        }
        self.add_line_data(&*new_string);
    }

    pub fn new_string_literal_with_len(&mut self, addr: &str, value: &str){
        self.new_string_literal(addr, value);
        self.add_line_data(&*format!(".len: equ $ - {}", addr))
    }

    // pub fn base(&mut self, arg1: &str, arg2: &str){}
    pub fn extern_add(&mut self, function_or_address: &str){
        self.add_line_text(&*format!("extern\t\t\t\t{}", function_or_address))
    }
    pub fn syscall(&mut self){
        self.add_raw_asm1("syscall");
    }
    pub fn pop(&mut self, register: &str){
        self.add_raw_asm2("pop", register)
    }
    pub fn push(&mut self, value_or_register: &str){
        self.add_raw_asm2("push", value_or_register)
    }
    pub fn call(&mut self, function: &str){
        self.add_raw_asm2("call", function)
    }
    pub fn mov(&mut self, register: &str, value_or_register: &str){
        self.add_raw_asm3("mov", register, value_or_register);
    }

    pub fn add_raw_asm1(&mut self, op: &str){
        self.add_value_function(op)
    }
    pub fn add_raw_asm2(&mut self, op: &str, left: &str){
        if op.len() > 3 {
            self.add_value_function(&*format!("{}\t\t\t{}", op, left))
        } else {
            self.add_value_function(&*format!("{}\t\t\t\t{}", op, left))
        }
    }
    pub fn add_raw_asm3(&mut self, op: &str, left: &str, right: &str){
        if op.len() > 3 {
            self.add_value_function(&*format!("{}\t\t\t{}, {}", op, left, right))
        } else {
            self.add_value_function(&*format!("{}\t\t\t\t{}, {}", op, left, right))
        }
    }
    pub fn syscall1(&mut self, arg1: &str){
        self.mov("rax", arg1);
        self.syscall()
    }
    pub fn syscall2(&mut self, arg1: &str, arg2: &str){
        self.mov("rax", arg1);
        self.mov("rdi", arg2);
        self.syscall()
    }
    pub fn syscall3(&mut self, arg1: &str, arg2: &str, arg3: &str){
        self.mov( "rax", arg1);
        self.mov( "rdi", arg2);
        self.mov( "rsi", arg3);
        self.syscall()
    }
    pub fn syscall4(&mut self, arg1: &str, arg2: &str, arg3: &str, arg4: &str){
        self.mov("rax", arg1);
        self.mov("rdi", arg2);
        self.mov("rsi", arg3);
        self.mov("rdx", arg4);
        self.syscall()
    }
    pub fn syscall5(&mut self, arg1: &str, arg2: &str, arg3: &str, arg4: &str, arg5: &str){
        self.mov("rax", arg1);
        self.mov("rdi", arg2);
        self.mov("rsi", arg3);
        self.mov("rdx", arg4);
        self.mov("r10", arg5);
        self.syscall()
    }
    pub fn syscall6(&mut self, arg1: &str, arg2: &str, arg3: &str, arg4: &str, arg5: &str, arg6: &str){
        self.mov("rax", arg1);
        self.mov("rdi", arg2);
        self.mov("rsi", arg3);
        self.mov("rdx", arg4);
        self.mov("r10", arg5);
        self.mov("r8", arg6);
        self.syscall()
    }
    pub fn syscall7(&mut self, arg1: &str, arg2: &str, arg3: &str, arg4: &str, arg5: &str, arg6: &str, arg7: &str){
        self.mov("rax", arg1);
        self.mov( "rdi", arg2);
        self.mov( "rsi", arg3);
        self.mov( "rdx", arg4);
        self.mov( "r10", arg5);
        self.mov( "r8", arg6);
        self.mov( "r9", arg7);
        self.syscall()
    }

    pub fn build(&mut self) -> String {
        self.text += ASM_BASE_END;
        self.data.to_owned() + &*self.bss + &*self.text
    }
    pub fn build_no_start(&mut self) -> String {
        self.data.to_owned() + &*self.bss + &*self.text
    }
}