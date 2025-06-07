use std::str::FromStr;
use crate::assembler::Parser;

pub mod assembler;

fn main() {
    let mut prog = assembler::Program::new();
    prog.parse(String::from_str(".main:\n\tmov r1, r2\n\tmov r4, r3").ok().unwrap());
    println!("{}", prog);
}
