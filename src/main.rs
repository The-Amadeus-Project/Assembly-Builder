extern crate core;

mod builder;
mod return_code;
mod b_lib;

fn main() {
    let mut asm_builder = builder::Builder::new_program("main");
    asm_builder.new_string_literal("name", "eric\n");
    asm_builder.extern_add( "printf");

    asm_builder.open_function("main");
    asm_builder.mov("rdi", "name");
    asm_builder.mov("al", "0");
    asm_builder.call("printf");

    asm_builder.new_local_dword(100);


    asm_builder.mov("rax","0");

    asm_builder.close_function();
    println!("{}", asm_builder.build_no_start());
}
