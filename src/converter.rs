extern crate regex;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Line {
    index: usize,
    indent: usize,
    text: String,
}

pub fn perform(text: &str) -> String {
    let lines = parse_lines(text);
    return text.to_string();
}

impl Line {
    fn has_umpersand(&self) -> bool {
        let re = Regex::new(r"(^| |\t)\&($|[^{: +>])").unwrap();
        match re.captures(&self.text) {
            Some(_) => true,
            None => false,
        }
    }
}

fn parse_lines(text: &str) -> Vec<Line> {
    let mut lines: Vec<Line> = vec![];
    let row_lines: Vec<&str> = text.split("\n").collect();
    for (index, line) in row_lines.iter().enumerate() {
        lines.push(Line {
            index,
            indent: count_indent(line),
            text: line.to_string(),
        });
    }
    lines
}

fn count_indent(line: &str) -> usize {
    for (index, s) in line.to_string().as_str().chars().enumerate() {
        if s != ' ' && s != '\t' {
            return index;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_umpersand() {
        let mut line = Line {
            index: 0,
            indent: 0,
            text: "".to_string(),
        };
        let truthy = ["&", " &", "\t&", "&-", "&_", "&a"];
        for s in truthy.iter() {
            line.text = s.to_string();
            assert!(line.has_umpersand());
        }

        let falsy = ["a", "& ", "_&", "&:", "&+", "&{", "&>"];
        for s in falsy.iter() {
            line.text = s.to_string();
            assert!(!line.has_umpersand());
        }
    }

    #[test]
    fn test_parse_lines() {
        let lines = parse_lines(&["a", "b", " c", "d"].join("\n"));
        assert_eq!(
            lines,
            [
                Line {
                    index: 0,
                    indent: 0,
                    text: "a".to_string()
                },
                Line {
                    index: 1,
                    indent: 0,
                    text: "b".to_string()
                },
                Line {
                    index: 2,
                    indent: 1,
                    text: " c".to_string()
                },
                Line {
                    index: 3,
                    indent: 0,
                    text: "d".to_string()
                }
            ]
        );
    }

    #[test]
    fn test_count_indent() {
        assert_eq!(count_indent("a"), 0);
        assert_eq!(count_indent(" a"), 1);
        assert_eq!(count_indent("  a"), 2);
        assert_eq!(count_indent("  a "), 2);
        assert_eq!(count_indent("\ta"), 1);
        assert_eq!(count_indent("\t\ta"), 2);
        assert_eq!(count_indent("\t\ta\t"), 2);
        assert_eq!(count_indent(" \ta"), 2);
    }
}
