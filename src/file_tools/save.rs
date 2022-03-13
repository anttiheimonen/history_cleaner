use crate::line_tools::line::Line;
use std::io::Write;

pub fn save(file_name: &String, text_lines: Vec<Line>) -> Result<(), String> {
    let line_change = "\n".as_bytes();
    let file_ref = std::fs::File::create(file_name);
    match file_ref {
        Ok(_) => (),
        Err(_) => return Err(format!("Cannot save to file: {}", file_name)),
    }
    let mut file_ref = file_ref.unwrap();
    for t in text_lines {
        if t.keep {
            let line = [t.text.as_bytes(), line_change].concat();
            file_ref.write(&line).expect("write failed");
        }
    }
    Ok(())
}
