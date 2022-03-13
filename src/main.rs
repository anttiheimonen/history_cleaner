use crate::read_file::read_file;
use history_cleaner::config::Config;
use history_cleaner::file_tools::read_file;
use history_cleaner::file_tools::save::save;
use history_cleaner::line_tools::line_collection;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = config(&args);
    match &config {
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
        Ok(_) => (),
    }
    let config = config.unwrap();
    let remove_these: Vec<&str> = vec!["cd", "code"];
    let mut text_lines = read_file(&config.source_file);

    line_collection::remove_duplicate_lines(&mut text_lines);
    line_collection::remove_lines_starts_with(&mut text_lines, &remove_these);

    match save(&config.target_file, text_lines) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}

fn config(args: &Vec<String>) -> Result<Config, String> {
    if args.len() < 2 {
        return Err(format!("Usage: {} [source file]", &args[0]));
    }
    let config = Config::new(String::from(&args[1]));
    return Ok(config);
}
