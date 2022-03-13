pub struct Config {
    pub source_file: String,
    pub target_file: String,
}

impl Config {
    pub fn new(source_file: String) -> Self {
        let target = format!("{}_cleaned", source_file);
        Config {
            source_file,
            target_file: target,
        }
    }
}
