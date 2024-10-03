use std::env;

use xenon_parser::node::Program;

mod x86_64;

fn main() {
    let mut args: Vec<String> = env::args().collect();
}


fn compile(program: Program) -> String {
    x86_64::compile();
}