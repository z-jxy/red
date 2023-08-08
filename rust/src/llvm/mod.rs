use std::process::Command;

use inkwell::context::Context;

use self::compiler::Compiler;

pub mod codegen;
mod compiler;
pub mod ast;
/*
pub fn llvm_codegen() {
	codegen::codegen();
}
 */

//pub fn llvm_debug() {
//	codegen::debug();
//}

pub fn emit(stmts: Vec<ast::Stmt>) {
	codegen::emit_from_statements(stmts);
}

pub fn compile_ast(ast: Vec<ast::Stmt>) {
	let context = Context::create();
    let mut compiler = Compiler::new(&context);
	compiler.add_stdlib();

    compiler.compile(&ast);
	println!("[*] Emitted to LLVM IR 1/2");

	let build_out = "akame-build";
	let ir_file_name = "main.ll";

	let ir_out: String = format!("{}/{}", build_out, ir_file_name);

	// create build directory if it doesn't exist
	match std::path::Path::new(build_out).try_exists() {
		Ok(bool) => {
			if bool == false {
				std::fs::create_dir(build_out).expect("failed to create build directory");
			}
		},
		Err(e) => {
			println!("Error: {}", e);
			std::fs::create_dir(build_out).expect("failed to check if directory exists");
		}
	}
    
	compiler.module.print_to_file(std::path::Path::new(&ir_out)).unwrap();


	Command::new("clang")
		// set the entry point to our _entry function which wraps the users main function
		.arg("-Wl,-e,__entry")
		// set the output file to build/main 
		.arg("-o")
		.arg(&format!("{}/main", build_out))
		// set the input file to build/main.ll
		.arg(ir_out)
		.output()
		.expect("failed to execute process");
	

	println!("[+] Successfully compiled to build/main 2/2")

}