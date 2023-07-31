use std::env;
use std::fs;
use std::process::exit;


fn md_to_html(md_file: &String) -> String {
    let md_contents: String = fs::read_to_string(md_file).expect("ERROR: could not read file");
    let md_lines: Vec<&str> = md_contents.lines().collect();

    let mut html_contents: String = String::new();

    for line in md_lines {
        let html_line = md_to_html_line(line);
        html_contents.push_str(&md_to_html_line(line));
        html_contents.push_str("\n");
    }
    html_contents
}

fn md_to_html_line(line: &str) -> String {
    let ltv = line_token_value(line);
    format!("First token: '{}'\nValue: '{}'\n", ltv.0, ltv.1)
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
    let value: &str = &line[ft_end..];
    (first_token, value)
}

// fn tokenize(line: &str) -> (&str, &str){
//     let chars = line.chars().collect::<Vec<char>>;
//     let start_token: &str;
//     let value: &str;
// 
//     let mut cursor: usize = 0;
//     let mut pivot: usize;
//     let mut curr_char: char;
// 
//     while cursor < chars.len() {
//         curr_char = chars[cursor];
//         if curr_char.is_whitespace() {
//             cursor += 1;
//         } else if curr_char == '>' {
//             pivot = cursor + 1;
//         } else if curr_char == '#' {
//             pivot= cursor +1;
//             while chars[pivot] == '#' {
//                 pivot+= 1;
//             }
//         } else if curr_char == '>' {
//             
//         }
//     }
//     start_token = chars[cursor..pivot].collect();
//     value.push_str(chars[end..].collect());
//     (start_token, value)
// }
// 
// fn tokenize(line: &str) -> Vec<&str> {
//     let line_vec: Vec<char> = line.chars().collect();
//     let mut start: usize = 0;
//     let mut curr: usize = 0;
//     let mut result: Vec<&str> = Vec::new();
// 
//     while curr < line.len() {
//         if !line_vec[curr].is_whitespace() {
//             curr += 1;
//         } else {
//             result.push(&line[start..curr]);
//             curr += 1;
//             start = curr;
//         }
//     }
//     if start != curr {
//         result.push(&line[start..]);
//     }
//     result
// }
// 
// fn tokens_to_typed_line(tokens: Vec<&str>) -> TypedLine {
//     if tokens.len() == 0 {
//         return TypedLine {
//             line_type: LineType::EmptyLine,
//             value: "".to_string(),
//         }
//     }
//     let lt: LineType;
//     let first_token: &str = tokens[0];
//     let first_char = first_token.chars.nth(0).unwrap();
// 
//     match first_token {
//         "#" => lt = LineType::H1,
//         "##" => lt = LineType::H2,
//         "###" => lt = LineType::H3,
//         "####" => lt = LineType::H4,
//         "#####" => lt = LineType::H5,
//         "######" => lt = LineType::H6,
//         "-" => lt = LineType::Ul,
//         _ => lt = LineType::Paragraph,
//     }
// 
//     let lv: String;
//     
//     if lt == LineType::Paragraph {
//         lv = tokens.join(" ");
//     } else {
//         lv = tokens[1..].join(" ");
//     }
// 
//     TypedLine{
//         line_type: lt, 
//         value: lv,
//     }
// }

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
    Blockquote,
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
            LineType::Blockquote => "Blockquote".to_string(),
        }
    }
}

struct TypedLine {
    line_type: LineType,
    value: String,
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


