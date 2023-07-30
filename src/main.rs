use std::env;
use std::fs;
use std::process::exit;


fn tokenize(line: &str) -> Vec<&str> {
    let line_vec: Vec<char> = line.chars().collect();
    let mut start: usize = 0;
    let mut curr: usize = 0;
    let mut result: Vec<&str> = Vec::new();

    while curr < line.len() {
        if !line_vec[curr].is_whitespace() {
            curr += 1;
        } else {
            result.push(&line[start..curr]);
            curr += 1;
            start = curr;
        }
    }
    if start != curr {
        result.push(&line[start..]);
    }
    result
}

#[derive(PartialEq)]
enum LineType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Ul,
    Paragraph,
    EmptyLine,
}

impl ToString for LineType {
    fn to_string(&self) -> String {
        match self {
            LineType::H1 => "H1".to_string(),
            LineType::H2 => "H2".to_string(),
            LineType::H3 => "H3".to_string(),
            LineType::H4 => "H4".to_string(),
            LineType::H5 => "H5".to_string(),
            LineType::H6 => "H6".to_string(),
            LineType::Ul => "Ul".to_string(),
            LineType::Paragraph => "Paragraph".to_string(),
            LineType::EmptyLine => "EmptyLine".to_string(),
        }
    }
}

struct TypedLine {
    line_type: LineType,
    value: String,
}

fn tokens_to_typed_line(tokens: Vec<&str>) -> TypedLine {
    if tokens.len() == 0 {
        return TypedLine {
            line_type: LineType::EmptyLine,
            value: "".to_string(),
        }
    }

    let first_token: &str = tokens[0];
    let mut lt: LineType;

    match first_token {
        "#" => lt = LineType::H1,
        "##" => lt = LineType::H2,
        "###" => lt = LineType::H3,
        "####" => lt = LineType::H4,
        "#####" => lt = LineType::H5,
        "######" => lt = LineType::H6,
        "-" => lt = LineType::Ul,
        _ => lt = LineType::Paragraph,
    }
    
    let lv: String;
    
    if lt == LineType::Paragraph {
        lv = tokens.join(" ");
    } else {
        lv = tokens[1..].join(" ");
    }

    TypedLine{
        line_type: lt, 
        value: lv,
    }
}

fn md_to_html_line(line: &str) -> String {
    let tokens: Vec<&str> = tokenize(line);    
    let tl = tokens_to_typed_line(tokens);
    format!("{} {}", tl.line_type.to_string(), tl.value)
}

fn markdown_to_html(file_path: &String) -> String {
    let contents: String = fs::read_to_string(file_path).expect("ERROR: could not read file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut html_contents: String = String::new();

    for line in lines {
        html_contents.push_str(&md_to_html_line(line));
        html_contents.push_str("\n");
    }
    html_contents
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len() - 1;

    // TODO: assert correct filetype
    // TODO: flag to indicate direction of translation

    if argc != 1 {
        eprintln!("ERROR: expected 1 argument, but got {argc}");
        exit(1);
        // TODO: handle this error properly  
    }

    let file_path: &String = &argv[1];

    let html = markdown_to_html(&file_path);
    println!("{html}");
}

