use std::fmt;

#[derive(Debug)]
pub struct Line {
    pub text: String,
    pub line_number: i32,
    pub keep: bool,
}

impl Line {
    fn _fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }

    pub fn drop(&mut self) {
        self.keep = false;
    }

    pub fn get_text(&self) -> &str {
        self.text.as_str()
    }
}

pub fn build_line(text: String, line_number: i32) -> Line {
    Line {
        text,
        line_number,
        keep: true,
    }
}
