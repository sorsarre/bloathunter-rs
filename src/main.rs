#[macro_use] extern crate lazy_static;

use std::env;
mod stage1;
mod stage2;

fn ident(mul: usize, len: usize) -> String {
    format!("{: <1$}", "", len*mul)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough arguments, specify input file!");
        return;
    }
    let input_file_name = args[1].clone();
    let result_stage1 = stage1::parse_file(&input_file_name);
    let result_stage2 = stage2::collect_matches(&result_stage1);

    for e in result_stage2 {
        println!("{}{}", ident(2, e.level as usize), e);
    }
}
