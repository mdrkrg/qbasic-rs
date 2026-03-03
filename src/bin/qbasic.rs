use std::{env::args, fs, process::exit};

use qbasic_rs::*;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() != 2 {
        let prog = &args[0];
        println!("Usage: {prog} /path/to/script.bas");
        exit(1);
    }

    let file = &args[1];
    if let Err(err) = fs::exists(file) {
        eprintln!("{err}");
        exit(1);
    }

    let read_result = fs::read_to_string(file);
    if let Err(err) = &read_result {
        eprintln!("{err}");
        exit(1);
    }

    let read_result = read_result.unwrap();

    let tokens = lexer::tokenize(&read_result);

    if let Err(err) = &tokens {
        for err in err {
            eprintln!("{err:#?}");
        }
        exit(1);
    }

    let mut parser = parser::Parser::new(tokens.unwrap());
    let lines = parser.parse();

    if let Err(err) = &lines {
        eprintln!("{err}");
        exit(1);
    }

    let mut interpreter = eval::Interpreter::new(lines.unwrap());
    interpreter.run_bin();
}
