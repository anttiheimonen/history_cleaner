use crate::read_file::read_file;
use history_cleaner::config::{self};
use history_cleaner::file_tools::read_file;
use history_cleaner::file_tools::save::save;
use history_cleaner::line_tools::line_collection;

fn main() -> std::io::Result<()> {
    let config = config::create_from_default();
    match config.config_file_error_msg {
        Some(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
        None => (),
    }

    let mut text_lines = read_file(&config.source_file);

    line_collection::remove_duplicate_lines(&mut text_lines);
    line_collection::remove_lines_starts_with(&mut text_lines, &config.remove_starts_with);

    match save(&config.target_file, text_lines) {
        Ok(_) => {
            println!("Cleaned file saved at {}", config.target_file);
        }
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    }
    Ok(())
}
