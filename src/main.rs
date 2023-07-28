use std::env;
use std::fs;


fn usage() -> String{
    let usage: String = "cargo run [file_path]".to_string();
    usage
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len: usize = args.len();

    // TODO: assert correct filetype
    // TODO: flag to indicate direction of translation

    if args_len != 2 {
        eprintln!("ERROR: expected 1 argument, but got {}", args_len-1);
        panic!();
        // println!("{}", usage());
        // TODO: handle this error properly  
    }

    let file_path: &String = &args[1];

    let contents: String = fs::read_to_string(file_path)
        .expect("ERROR: could not read file");

    print!("{contents}");
}

