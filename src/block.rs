extern crate regex;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Block {
    lines: Vec<String>,
    // head: String,
    // body: String,
    children: Vec<Block>,
}

impl Block {
    fn new(lines: Vec<String>) -> Block {
        Block {
            lines,
            children: vec!(),
        }
    }

    fn parse_children(&self) {
    }
}

fn parse_blocks(text: &str) -> Vec<Block> {
    let mut blocks = vec!();
    let lines: Vec<&str> = text.split("\n").collect();
    let mut index = 0;

    while index < lines.len() {
        let end_index = get_block_end(&lines[index..].to_vec());
        let block = Block::new(
            lines[index..index + end_index + 1]
            .iter().map(|s| s.to_string()).collect());
        blocks.push(block);
        index = index + end_index + 1;
    }
    blocks
}

pub fn get_block_range(lines: Vec<&str>) -> (usize, usize) {
    let head_end = get_head_end(&lines);
    let body_end = get_block_end(&lines[head_end + 1..].to_vec());
    let head_start = get_head_start(&lines, head_end);
    (head_start, body_end)
}

fn get_head_start(lines: &Vec<&str>, head_end: usize) -> usize {
    let in_head_re = Regex::new(r"(.*,$)|(^( |\t)*$)").unwrap();
    for (index, line) in lines[..head_end].iter().rev().enumerate() {
        if let None = in_head_re.captures(line) {
            return head_end - index;
        }
    }
    0
}

fn get_head_end(lines: &Vec<&str>) -> usize {
    let head_end_re = Regex::new(r"(.*\{$)").unwrap();
    for (index, line) in lines.iter().enumerate() {
        if let Some(_) = head_end_re.captures(line.trim()) {
            return index;
        }
    }
    0
}

fn get_block_end(lines: &Vec<&str>) -> usize {
    if lines.len() == 0 { return 0; }

    let indent_len = get_indent_text(&lines[0]).len();
    let body_end_re = Regex::new(r"}$").unwrap();
    for (index, line) in lines.iter().enumerate() {
        if indent_len != get_indent_text(&line).len() { continue; }
        if let Some(_) = body_end_re.captures(line.trim()) { return index; }
    }
    lines.len() - 1
}

fn get_indent_text(line: &str) -> String {
    let pre_re = Regex::new(r"( |\t)*").unwrap();
    if let Some(cap) = pre_re.captures(line) {
        cap[0].to_string()
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_blocks() {
        assert_eq!(parse_blocks(&[
                   "a {",
                   "}",
                   "a {",
                   "}",
        ].join("\n")), vec!(
            Block::new(vec!("a {".to_string(), "}".to_string())),
            Block::new(vec!("a {".to_string(), "}".to_string())),
        ));
    }

    #[test]
    fn test_get_head_start() {
        assert_eq!(get_head_start(&vec!(), 0), 0);
        assert_eq!(get_head_start(&vec!("a {}"), 0), 0);
        assert_eq!(get_head_start(&vec!(
                    "}",
                    "",
                    " ",
                    "a {",
                    "}"), 2), 1);
    }

    #[test]
    fn test_get_head_end() {
        assert_eq!(get_head_end(&vec!()), 0);
        assert_eq!(get_head_end(&vec!("a {}")), 0);
        assert_eq!(get_head_end(&vec!(
                    "",
                    " ",
                    "a {",
                    "}")), 2);
        assert_eq!(get_head_end(&vec!(
                    "a,",
                    "a, ",
                    "a {")), 2);
        assert_eq!(get_head_end(&vec!(
                    "a,",
                    "",
                    "a,",
                    "a {")), 3);
    }

    #[test]
    fn test_get_block_end() {
        assert_eq!(get_block_end(&vec!()), 0);
        assert_eq!(get_block_end(&vec!("a {}")), 0);
        assert_eq!(get_block_end(&vec!(
                    "a {",
                    "}")), 1);
        assert_eq!(get_block_end(&vec!(
                    "a {",
                    "}",
                    "a {",
                    "}")), 1);
        assert_eq!(get_block_end(&vec!(
                    "a {",
                    " b {",
                    " }",
                    "}")), 3);
        assert_eq!(get_block_end(&vec!(
                    "a {",
                    " b {",
                    " }",
                    " b {",
                    " }",
                    "}")), 5);
    }
}
