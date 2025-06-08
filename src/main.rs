use std::{fs::File, io::Read, str::FromStr, env};
use crate::{assembler::Parser, lexer::Lexer};

pub mod assembler;
pub mod lexer;

fn main() {
    // test_assembly()

    let mut args = std::env::args();
    let path_option = args.nth(1);
    if let None = path_option { panic!("Usage: ./compiler <input file>") }

    let path = path_option.unwrap();
    let mut buf = String::new();
    let file = File::open(&path);
    if let Err(_) = file { panic!("Failed to open file {}", &path) }
    let _ = File::read_to_string(&mut file.unwrap(), &mut buf);

    let mut lexer = Lexer::new();
    lexer.set_program(buf);
    lexer.lex();
    println!("{:?}", lexer.tokens);
}

fn test_assembly() {
    let mut args = std::env::args();
    let path_option = args.nth(1);
    if let None = path_option { panic!("Usage: ./assembler <input file>") }

    let path = path_option.unwrap();
    let mut buf = String::new();
    let file = File::open(&path);
    if let Err(_) = file { panic!("Failed to open file {}", &path) }

    let mut prog = assembler::Program::new();
    let _ = File::read_to_string(&mut file.unwrap(), &mut buf);

    prog.parse(buf);
    println!("{}", prog);
    let bytecode = prog.assemble();
    println!("{}", bytecode);
}
