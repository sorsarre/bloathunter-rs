use std::fs;
use std::io;
use std::io::BufRead;
use std::fmt;
use std::convert::From;
use regex::Regex;
use regex::Captures;

#[derive(Debug)]
pub struct IncludeMatch {
    pub line: u32,
    pub file_name: String,
    pub is_new: bool,
    pub is_return: bool
}

#[derive(Debug)]
pub enum Stage1Entry {
    CodeLine,
    BlankLine,
    IncludeMatch(IncludeMatch),
}

impl fmt::Display for Stage1Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stage1Entry::CodeLine => {
                write!(f, "[Stage1 code line]")
            },

            Stage1Entry::BlankLine => {
                write!(f, "[Stage1 blank line]")
            },

            Stage1Entry::IncludeMatch(im) => {
                write!(f, "[Stage1 line={}, file={}, is_new={}, is_return={}",
                    im.line, im.file_name, im.is_new, im.is_return)
            },
        }
    }
}

fn parse_raw_include_line(line: &String) -> Stage1Entry {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"#\s+(\d+)\s+"(.*)"(?:\s+(\d+))?(?:\s+(\d+))?(?:\s+(\d+))?(?:\s+(\d+))?\s*\n?"#).unwrap();
        static ref BLANK_RE: Regex = Regex::new(r#"^\s*\n$"#).unwrap();
    }

    if BLANK_RE.is_match(line) {
        return Stage1Entry::BlankLine;
    }

    let captures: Option<Captures> = RE.captures(line);
    match captures {
        Some(caps) => {
            let lino = caps[1].parse::<u32>().unwrap();
            let file_name = String::from(&caps[2]);
            let mut flag_new = false;
            let mut flag_ret = false;
            for iter in 3..7 {
                caps.get(iter).map(|m| {
                    let s = m.as_str();
                    match s {
                        "1" => { flag_new = true; },
                        "2" => { flag_ret = true; },
                        _ => {},
                    }
                });
            }
            let im = IncludeMatch{line: lino, file_name: file_name, is_new: flag_new, is_return: flag_ret };
            Stage1Entry::IncludeMatch(im)
        },

        None => {
            Stage1Entry::CodeLine
        },
    }
}

pub fn parse_file(input_file_name: &String) -> Vec<Stage1Entry> {
    let input_file = fs::File::open(&input_file_name).expect("Could not open file");
    let reader = io::BufReader::new(&input_file);

    let mut result: Vec<Stage1Entry> = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let entry = parse_raw_include_line(&l);
        result.push(entry);
    }

    return result;
}