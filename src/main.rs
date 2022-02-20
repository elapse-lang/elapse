use elapse_parser::ast::parse_elapse;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    if argc < 2 {
        println!("Please specify source file path.");
        return;
    }

    let path = &args[1];
    if !path.ends_with(".syt") {
        println!("File specified was not a .syt source file.");
        return;
    }

    let source = fs::read_to_string(path)
        .expect("Unable to read source file.");
    let ast = parse_elapse(&source);
    match ast {
        Ok(v) => println!("working"),
        Err(e) => println!("{:?}", e)
    }
}