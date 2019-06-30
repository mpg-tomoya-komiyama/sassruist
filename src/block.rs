extern crate regex;
use regex::Regex;
use std::cell::RefCell;

#[derive(Debug, PartialEq)]
pub struct Block {
    lines: RefCell<Vec<String>>,
    // head: String,
    // body: String,
    children: Vec<Block>,
    oneliner: bool,
}

impl Block {
    fn new(lines: Vec<String>) -> Block {
        let mut children: Vec<Block> = vec!();
        let head_end = get_head_end(&lines);
        let block_end = get_block_end(&lines);
        let oneliner = head_end == block_end;

        if !oneliner && block_end < lines.len() {
            children = parse_blocks(&lines[head_end + 1..block_end].to_vec());
        }
        Block {
            lines: RefCell::new(lines),
            children,
            oneliner,
        }
    }

    fn selectors(&self) -> Vec<String> {
        let mut selectors: Vec<String> = vec![];
        let re = Regex::new(r"[^{]*").unwrap();
        let head_end = get_head_end(&self.lines.borrow());

        for line in self.lines.borrow()[..head_end + 1].to_vec() {
            if let Some(cap) = re.captures(&line) {
                let dropped = cap[0].to_string();
                for s in dropped.split(',') {
                    let trimmed = s.trim();
                    if trimmed != "" {
                        selectors.push(trimmed.to_string());
                    }
                }
            }
        }
        selectors
    }

    fn has_ampersand(&self) -> bool {
        let re = Regex::new(r"(^| |\t)\&($|[^{: +>.#])").unwrap();
        for selector in self.selectors() {
            if has_ampersand(&selector) {
                return true;
            }
        }
        false
    }

    fn resolve_anpersand(&self, parent_selectors: &Vec<String>) {
        let self_selectors = self.selectors();

        let mut resolved_selectors: Vec<String> = vec![];
        for selector in self_selectors {
            let mut s = selector;
            if !has_ampersand(&s) {
                s = "& ".to_string() + &s;
            }
            for p in parent_selectors {
                resolved_selectors.push(s.replace("&", &p));
            }
        }

        let mut resolved = resolved_selectors.join(", ").to_string();

        let head_end = get_head_end(&self.lines.borrow());
        let re = Regex::new(r"\{+.*").unwrap();

        resolved = get_indent_text(&self.lines.borrow()[head_end]) + &resolved;
        if let Some(cap) = re.captures(&self.lines.borrow()[head_end]) {
            resolved = resolved + " " + &cap[0];
        }

        self.lines.borrow_mut().drain(..head_end);
        self.lines.borrow_mut()[0] = resolved.clone();
    }

}

pub fn parse_blocks(lines: &Vec<String>) -> Vec<Block> {
    let mut blocks = vec!();
    let mut index = 0;

    while index < lines.len() {
        let (start_index, end_index) = get_block_range(&lines[index..].to_vec());
        let block = Block::new(
            lines[index + start_index..index + end_index + 1]
            .iter().map(|s| s.to_string()).collect());
        blocks.push(block);
        index = index + end_index + 1;
    }
    blocks
}

fn get_block_range(lines: &Vec<String>) -> (usize, usize) {
    let head_end = get_head_end(&lines);
    let body_end = get_block_end(&lines[head_end..].to_vec());
    let head_start = get_head_start(&lines, head_end);
    (head_start, body_end)
}

fn get_head_start(lines: &Vec<String>, head_end: usize) -> usize {
    let in_head_re = Regex::new(r"(.*,$)|(^( |\t)*$)").unwrap();
    for (index, line) in lines[..head_end].iter().rev().enumerate() {
        if let None = in_head_re.captures(line) {
            return head_end - index;
        }
    }
    0
}

fn get_head_end(lines: &Vec<String>) -> usize {
    let head_end_re = Regex::new(r"(.*\{.*\}?$)").unwrap();
    for (index, line) in lines.iter().enumerate() {
        if let Some(_) = head_end_re.captures(line.trim()) {
            return index;
        }
    }
    0
}

fn get_block_end(lines: &Vec<String>) -> usize {
    if lines.len() == 0 { return 0; }

    let indent_len = get_indent_text(&lines[0]).len();
    let body_end_re = Regex::new(r"}$").unwrap();
    for (index, line) in lines.iter().enumerate() {
        if indent_len != get_indent_text(&line).len() { continue; }
        if let Some(_) = body_end_re.captures(line.trim()) { return index; }
    }
    lines.len() - 1
}

fn get_indent_text(line: &String) -> String {
    let pre_re = Regex::new(r"( |\t)*").unwrap();
    if let Some(cap) = pre_re.captures(line) {
        cap[0].to_string()
    } else {
        "".to_string()
    }
}

fn has_ampersand(selector: &str) -> bool {
    let re = Regex::new(r"(^| |\t)\&($|[^{: +>.#])").unwrap();
    match re.captures(selector) {
        Some(_) => true,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_new_1() {
        let block = Block::new(vec![
                   "a {",
                   " b {",
                   "  c {",
                   "  }",
                   " }",
                   " b {",
                   " }",
                   "}",
        ].iter().map(|s| s.to_string()).collect());
        assert_eq!(block.children.len(), 2);
        assert_eq!(block.children[0].lines.borrow(), vec![" b {", "  c {", "  }", " }"]);
        assert_eq!(block.children[0].children.len(), 1);
        assert_eq!(block.children[0].children[0].lines.borrow(), vec!["  c {", "  }"]);
        assert_eq!(block.children[1].lines.borrow(), vec![" b {", " }"]);
        assert_eq!(block.children[1].children.len(), 0);
    }

    #[test]
    fn test_block_new_2() {
        let block = Block::new(vec![
                   "a {",
                   " b {}",
                   " c {",
                   " }",
                   " b {}",
                   "}",
        ].iter().map(|s| s.to_string()).collect());
        assert_eq!(block.children.len(), 3);
        assert_eq!(block.children[0].lines.borrow(), vec![" b {}"]);
        assert_eq!(block.children[0].children.len(), 0);
        assert_eq!(block.children[1].lines.borrow(), vec![" c {", " }"]);
        assert_eq!(block.children[1].children.len(), 0);
        assert_eq!(block.children[2].lines.borrow(), vec![" b {}"]);
        assert_eq!(block.children[2].children.len(), 0);
    }

    #[test]
    fn test_block_selectors() {
        let block = Block::new(vec![
                   "a {",
                   "}",
        ].iter().map(|s| s.to_string()).collect());
        assert_eq!(block.selectors(), vec!["a"]);

        let block = Block::new(vec![
                   "a, b,",
                   " ",
                   "c, d f {",
                   "}",
        ].iter().map(|s| s.to_string()).collect());
        assert_eq!(block.selectors(), vec!["a", "b", "c", "d f"]);
    }

    #[test]
    fn test_block_has_ampersand() {
        let block = Block::new(vec![
                   "a {",
                   "}",
        ].iter().map(|s| s.to_string()).collect());
        assert_eq!(block.has_ampersand(), false);

        let block = Block::new(vec![
                   "a, &b,",
                   "c, d f {",
                   "}",
        ].iter().map(|s| s.to_string()).collect());
        assert_eq!(block.has_ampersand(), true);
    }

    #[test]
    fn test_parse_blocks_1() {
        let blocks = parse_blocks(&vec![
                   "a {",
                   "}",
                   "a {",
                   "}",
        ].iter().map(|s| s.to_string()).collect());
        assert_eq!(blocks, vec!(
            Block::new(vec!("a {".to_string(), "}".to_string())),
            Block::new(vec!("a {".to_string(), "}".to_string())),
        ));
    }

    #[test]
    fn test_parse_blocks_2() {
        let blocks = parse_blocks(&vec![
                   "a {}",
                   "b {",
                   "}",
        ].iter().map(|s| s.to_string()).collect());
        assert_eq!(blocks, vec!(
            Block::new(vec!("a {}".to_string())),
            Block::new(vec!("b {".to_string(), "}".to_string())),
        ));
    }

    #[test]
    fn test_get_head_start() {
        assert_eq!(get_head_start(&vec!(), 0), 0);
        assert_eq!(get_head_start(&vec!("a {}").iter().map(|s| s.to_string()).collect(), 0), 0);
        assert_eq!(get_head_start(&vec!(
                    "}",
                    "",
                    " ",
                    "a {",
                    "}").iter().map(|s| s.to_string()).collect(), 2), 1);
    }

    #[test]
    fn test_get_head_end() {
        assert_eq!(get_head_end(&vec!()), 0);
        assert_eq!(get_head_end(&vec!("a {}").iter().map(|s| s.to_string()).collect()), 0);
        assert_eq!(get_head_end(&vec!("a {}", "b {", "}").iter().map(|s| s.to_string()).collect()), 0);
        assert_eq!(get_head_end(&vec!(
                    "",
                    " ",
                    "a {",
                    "}").iter().map(|s| s.to_string()).collect()), 2);
        assert_eq!(get_head_end(&vec!(
                    "a,",
                    "a, ",
                    "a {").iter().map(|s| s.to_string()).collect()), 2);
        assert_eq!(get_head_end(&vec!(
                    "a,",
                    "",
                    "a,",
                    "a {").iter().map(|s| s.to_string()).collect()), 3);
    }

    #[test]
    fn test_get_block_end() {
        assert_eq!(get_block_end(&vec!()), 0);
        assert_eq!(get_block_end(&vec!("a {}").iter().map(|s| s.to_string()).collect()), 0);
        assert_eq!(get_block_end(&vec!("a {}", "b {", "}").iter().map(|s| s.to_string()).collect()), 0);
        assert_eq!(get_block_end(&vec!(
                    "a {",
                    "}").iter().map(|s| s.to_string()).collect()), 1);
        assert_eq!(get_block_end(&vec!(
                    "a {",
                    "}",
                    "a {",
                    "}").iter().map(|s| s.to_string()).collect()), 1);
        assert_eq!(get_block_end(&vec!(
                    "a {",
                    " b {",
                    " }",
                    "}").iter().map(|s| s.to_string()).collect()), 3);
        assert_eq!(get_block_end(&vec!(
                    "a {",
                    " b {",
                    " }",
                    " b {",
                    " }",
                    "}").iter().map(|s| s.to_string()).collect()), 5);
    }
}
