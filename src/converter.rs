extern crate regex;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Line {
    index: usize,
    indent: usize,
    text: String,
}

pub fn perform(text: &str) -> String {
    let mut lines = parse_lines(text);
    if lines.len() == 0 {
        return text.to_string();
    }

    let mut parent = lines.remove(0);
    let mut past = parent.clone();
    let mut converted_lines = vec![parent.text.clone()];
    for mut line in lines {
        if parent.indent < line.indent {
            if parent.indent < past.indent && past.indent < line.indent {
                parent = past;
            }
            line.resolve(&parent);
        } else if parent.indent == line.indent {
            parent = line.clone();
        }
        converted_lines.push(line.text.clone());
        past = line.clone();
    }
    converted_lines.join("\n")
}

impl Line {
    fn has_umpersand(&self) -> bool {
        let re = Regex::new(r"(^| |\t)\&($|[^{: +>])").unwrap();
        match re.captures(&self.text) {
            Some(_) => true,
            None => false,
        }
    }

    fn resolve(&mut self, parent: &Line) {
        if !self.has_umpersand() {
            return;
        }
        let parent_selectors = parse_selectors(&parent.text);
        self.text = resolve_umpersand(&self.text, parent_selectors);
    }
}

fn parse_lines(text: &str) -> Vec<Line> {
    let mut lines: Vec<Line> = vec![];
    let row_lines = text.split("\n");
    for (index, line) in row_lines.enumerate() {
        lines.push(Line {
            index,
            indent: count_indent(line),
            text: line.to_string(),
        });
    }
    lines
}

fn count_indent(line: &str) -> usize {
    let pre_re = Regex::new(r"( |\t)*").unwrap();
    if let Some(cap) = pre_re.captures(line) {
        cap[0].len()
    } else {
        0
    }
}

fn parse_selectors(line: &str) -> Vec<String> {
    let mut selectors = vec![];
    let re = Regex::new(r"[^{]*").unwrap();
    if let Some(cap) = re.captures(line) {
        let dropped = cap[0].to_string();
        let splited = dropped.split(',');
        for s in splited {
            let trimmed = s.trim();
            if trimmed != "" {
                selectors.push(trimmed.to_string());
            }
        }
    }
    selectors
}

fn resolve_umpersand(line: &str, parent_selectors: Vec<String>) -> String {
    let mut selectors: Vec<String> = vec![];
    let src_selectors = parse_selectors(line);
    for s in src_selectors {
        for p in &parent_selectors {
            selectors.push(s.replace("&", &p));
        }
    }

    let mut resolved = "".to_string();

    let pre_re = Regex::new(r"( |\t)*").unwrap();
    if let Some(cap) = pre_re.captures(line) {
        resolved = cap[0].to_string();
    }

    resolved = resolved + &selectors.join(", ").to_string();

    let re = Regex::new(r"\{+.*").unwrap();
    if let Some(cap) = re.captures(line) {
        resolved = resolved + " " + &cap[0];
    }

    resolved
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perform() {
        let data = [
            [
                ["a {", " &_b {", " }", "}"].join("\n"),
                ["a {", " a_b {", " }", "}"].join("\n"),
            ],
            [
                ["a {", " &_b {", "  &-c {}", " }", "}"].join("\n"),
                ["a {", " a_b {", "  a_b-c {}", " }", "}"].join("\n"),
            ],
        ];
        for d in data.iter() {
            assert_eq!(perform(&d[0]), d[1]);
        }
    }

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
    fn test_resolve() {
        let mut parent = Line {
            index: 0,
            indent: 0,
            text: "p".to_string(),
        };
        let mut line = Line {
            index: 1,
            indent: 1,
            text: "&_a".to_string(),
        };
        let data = [
            ["p {", "& {", "& {"],
            ["p {", "& > a {", "& > a {"],
            ["p {", "&_a {", "p_a {"],
            ["p q {", "&_a {", "p q_a {"],
            ["p, q {", "&_a {", "p_a, q_a {"],
        ];
        for d in data.iter() {
            parent.text = d[0].to_string();
            line.text = d[1].to_string();
            line.resolve(&parent);
            assert_eq!(line.text, d[2]);
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

    #[test]
    fn test_parse_selectors() {
        assert_eq!(parse_selectors(" a "), ["a"]);
        assert_eq!(parse_selectors("a b"), ["a b"]);
        assert_eq!(parse_selectors("a, b"), ["a", "b"]);
        assert_eq!(parse_selectors("a {"), ["a"]);
        assert_eq!(parse_selectors("a { b {} }"), ["a"]);
    }

    #[test]
    fn test_resolve_umpersand() {
        assert_eq!(resolve_umpersand(" &_a", vec!("p".to_string())), " p_a");
        assert_eq!(
            resolve_umpersand("&_a", vec!("p".to_string(), "q".to_string())),
            "p_a, q_a"
        );
        assert_eq!(
            resolve_umpersand("&_a {", vec!("p".to_string(), "q".to_string())),
            "p_a, q_a {"
        );
        assert_eq!(
            resolve_umpersand("&_a, &_b {}", vec!("p".to_string(), "q".to_string())),
            "p_a, q_a, p_b, q_b {}"
        );
    }
}
