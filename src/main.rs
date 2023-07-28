use std::env;
use std::fs;
use std::str::Chars;


fn lex_md(md_file: String) {
    for line in md_file.lines() {
        lex_md_line(line);
    }
}

fn lex_md_line(md_line: &str) {
    // println!("{}", md_line.to_string());
    let mut md_line_chars: Chars = md_line.chars();
    println!("{}", md_line_chars.nth(0).unwrap_or(' '));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len: usize = args.len();

    // TODO: assert correct filetype
    // TODO: flag to indicate direction of translation

    if args_len != 2 {
        eprintln!("ERROR: expected 1 argument, but got {}", args_len-1);
        panic!();
        // TODO: handle this error properly  
    }

    let file_path: &String = &args[1];

    let contents: String = fs::read_to_string(file_path)
        .expect("ERROR: could not read file");

    lex_md(contents);
}

