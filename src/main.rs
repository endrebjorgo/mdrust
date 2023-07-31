use std::env;
use std::fs;
use std::process::exit;
use regex::Regex;


fn md_to_html(md_file: &String) -> String {
    let md_contents: String = fs::read_to_string(md_file).expect("ERROR: could not read file");
    let md_lines: Vec<&str> = md_contents.lines().collect();

    let mut html_contents: String = String::new();

    for line in md_lines {
        html_contents.push_str(&md_to_html_line(line));
        html_contents.push_str("\n");
    }
    html_contents
}

fn md_to_html_line(line: &str) -> String {
    let ltv = line_token_value(line);
    let html = ltv_to_html(ltv);
    post_parser(&html)
}

fn post_parser(line: &str) -> String {
    let chars = line.chars().collect::<Vec<char>>();
    let mut new_chars =  String::new();
    let mut cursor: usize = 0;
    let mut curr_char: char;
    let mut opened_block: Vec<char> = Vec::new();

    while cursor < chars.len() {
        curr_char = chars[cursor];

        if curr_char == '*' {
            if chars[cursor+1] != '*'{
                if opened_block.contains(&'e') {
                    new_chars.push_str("</em>");
                } else {
                    new_chars.push_str("<em>");
                    opened_block.push('e');
                }
                cursor += 1;
            } else {
                if opened_block.contains(&'*') {
                    new_chars.push_str("</b>");
                } else {
                    new_chars.push_str("<b>");
                    opened_block.push('*');
                }
                cursor += 2;
            }
        } else if curr_char == '`' {
            new_chars.push(curr_char);
            cursor += 1;
        } else if curr_char == '~' {
            new_chars.push(curr_char);
            cursor += 1;
        } else {
            new_chars.push(curr_char);
            cursor += 1;
        }
    }
    new_chars
}

fn line_token_value(line: &str) -> (&str, &str) {
    let chars = line.chars().collect::<Vec<char>>();
    let mut ft_start: usize = 0;
    let mut ft_end: usize; 

    while ft_start < chars.len() && chars[ft_start].is_whitespace() {
        ft_start += 1;
    } 
    if ft_start == chars.len() {
        // The whole line was a whitespace
        return ("", "");
    }
    ft_end = ft_start + 1; 

    while ft_end < chars.len() && !chars[ft_end].is_whitespace() {
        ft_end += 1;
    }
    let first_token: &str = &line[ft_start..ft_end];
    let value: &str = strip_left(&line[ft_end..]);
    (first_token, value)
}

fn strip_left(value: &str) -> &str {
    let mut cursor: usize = 0;
    let chars = value.chars().collect::<Vec<char>>();
    
    while cursor < chars.len() && chars[cursor].is_whitespace() {
        cursor += 1;
    }
    &value[cursor..]
}

fn ltv_to_html(ltv: (&str, &str)) -> String {
    let mut first_token = ltv.0;
    let value = ltv.1;
    let tag_name: &str;

    if first_token == "" {
        return "<br>".to_string();
    }

    if Regex::new(r"^\d+.{1}$").unwrap().is_match(first_token){
        first_token = "ol";
    }

    match first_token {
        "#" => tag_name = "h1",
        "##" => tag_name = "h2",
        "###" => tag_name = "h3",
        "####" => tag_name = "h4",
        "#####" => tag_name = "h5",
        "######" => tag_name = "h6",
        "-" => tag_name = "ul",
        "ol" => tag_name = "ol",
        ">" => tag_name = "blockquote",
        &_ => tag_name = "p",
    }
    if tag_name == "p" {
        format!("<{tag_name}>{first_token} {value}</{tag_name}>")
    } else {
        format!("<{tag_name}>{value}</{tag_name}>")
    }
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

    let html = md_to_html(&file_path);
    println!("{html}");
}

