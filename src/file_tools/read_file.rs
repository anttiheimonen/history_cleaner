use crate::{config::Config, line_tools::line::*};
use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

pub fn read_file(config: &Config) -> Vec<Line> {
    let file_name = &config.source_file;
    let mut text_lines = vec![];

    if let Ok(lines) = read_lines_from_file(file_name) {
        let mut line_count: i32 = 0;
        for line in lines {
            if let Ok(text) = line {
                if text.len() >= config.remove_lines_shorter_than {
                    let l = build_line(text, line_count);
                    text_lines.push(l);
                    line_count += 1;
                }
            }
        }
    } else {
        println!("File is missing");
    }

    text_lines
}

fn read_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
