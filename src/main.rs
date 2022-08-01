extern crate core;

mod builder;
mod return_code;
mod b_lib;

fn main() {
    let mut asm_builder = builder::Builder::new_program("main");
    asm_builder.new_string_literal("put_i_fmt_str", "%d\n");
    asm_builder.new_string_literal("put_s_fmt_str", "%s\n");
    asm_builder.new_string_literal("string_1", "mongrel\n");
    asm_builder.extern_add( "printf");

    asm_builder.open_function("put_i");
    asm_builder.mov("esi", "edi");
    asm_builder.mov("eax", "0");
    asm_builder.call_function("printf", vec!["put_i_fmt_str"]);
    asm_builder.close_function();

    asm_builder.open_function("put_s");
    asm_builder.mov("esi", "edi");
    asm_builder.mov("eax", "0");
    asm_builder.call_function("printf", vec!["put_s_fmt_str"]);
    asm_builder.close_function();

    asm_builder.open_function("main");
    asm_builder.new_local_dword(32);
    asm_builder.add("dword [rsp - 4]", "10");
    asm_builder.call_function("put_i", vec!["dword [rsp - 4]"]);

    asm_builder.call_function("put_s", vec!["string_1"]);

    asm_builder.mov("rax","0");

    asm_builder.close_function();
    println!("{}", asm_builder.build_no_start());
}
