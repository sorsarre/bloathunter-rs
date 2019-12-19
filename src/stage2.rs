use std::fmt;
use std::fs;
use super::stage1;

#[derive(Debug)]
pub struct Entry {
    pub file_name: String,
    pub level: u32,
    pub lines_proper: u32
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Stage2 file_name={}, level={}, lines_proper={}]",
            self.file_name, self.level, self.lines_proper)
    }
}

fn get_absolute_path(p: &String) -> String {
    let path_result = fs::canonicalize(p.clone());
    if path_result.is_err() {
        return p.clone();
    }

    let path = path_result.unwrap();
    return String::from(path.to_str().unwrap());
}

pub fn collect_matches(matches: &Vec<stage1::Stage1Entry>) -> Vec<Entry> {
    let mut counter: u32 = 0;
    let mut level: u32 = 0;
    let mut file_name: String = String::from("<root>");
    let mut result: Vec<Entry> = Vec::new();

    for m in matches {
        match m {
            stage1::Stage1Entry::BlankLine => {
                // do nothing here!
            },

            stage1::Stage1Entry::CodeLine => {
                counter += 1;
            },

            stage1::Stage1Entry::IncludeMatch(im) => {
                let entry = Entry{
                    file_name: get_absolute_path(&file_name),
                    level: level,
                    lines_proper: counter
                };
                result.push(entry);

                counter = 0;
                file_name = im.file_name.clone();
                if im.is_new || result.len() == 1 {
                    level += 1;
                } else if im.is_return {
                    level -= 1;
                }
            }
        }
    }

    let entry = Entry{
        file_name: file_name.clone(),
        level: level,
        lines_proper: counter
    };
    result.push(entry);

    return result;
}