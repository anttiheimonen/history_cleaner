use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io::Read;
use std::{env, fs::File};

const CONFIG_PATH: &'static str = "/.config/historycleaner/config.json";

pub fn user_home_path() -> Option<String> {
    let evars = env::vars();
    for var in evars {
        if var.0.eq("HOME") {
            return Some(var.1);
        }
    }
    None
}

pub struct Config {
    pub config_file: Option<String>,
    pub source_file: String,
    pub target_file: String,
    pub auto_replace_old: bool,
    pub config_file_error_msg: Option<String>,
    pub remove_lines_shorter_than: usize,
    pub remove_starts_with: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunValues {
    auto_replace_old: bool,
    remove_lines_shorter_than: usize,
    remove_starts_with: Vec<String>,
    source_file: String,
    target_file: String,
}

impl Config {
    pub fn new_with_error(error: String) -> Self {
        Config {
            config_file_error_msg: Some(error),
            ..Default::default()
        }
    }

    pub fn new_from_run_values(path: String, run_values: RunValues) -> Self {
        Config {
            config_file: Some(path),
            source_file: run_values.source_file,
            target_file: run_values.target_file,
            auto_replace_old: run_values.auto_replace_old,
            config_file_error_msg: None,
            remove_lines_shorter_than: run_values.remove_lines_shorter_than,
            remove_starts_with: run_values.remove_starts_with,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            config_file: None,
            source_file: String::from(""),
            target_file: String::from(""),
            auto_replace_old: false,
            config_file_error_msg: None,
            remove_lines_shorter_than: 0,
            remove_starts_with: vec![],
        }
    }
}

pub fn create_from_file() -> Config {
    let home_path: String;
    match user_home_path() {
        Some(value) => home_path = value,
        None => return Config::new_with_error(String::from("User home path cannot be resolved")),
    }
    let config_path = format!("{}{}", home_path, CONFIG_PATH);
    process_file(&config_path)
}

fn process_file(config_path: &str) -> Config {
    let config_file: File;
    match File::open(config_path) {
        Ok(f) => config_file = f,
        Err(e) => return Config::new_with_error(e.to_string()),
    }

    let file_content: String;
    match read_file(config_file) {
        Ok(content) => file_content = content,
        Err(e) => return Config::new_with_error(e.to_string()),
    }

    let rv: RunValues;
    match parse_file(file_content) {
        Ok(value) => rv = value,
        Err(e) => return Config::new_with_error(e.to_string()),
    }

    Config::new_from_run_values(String::from(config_path), rv)
}

fn read_file(mut file: File) -> std::io::Result<String> {
    let mut contents = String::new();
    let result = file.read_to_string(&mut contents);
    match result {
        Ok(_) => return Ok(contents),
        Err(e) => return Err(e),
    }
}

fn parse_file(content: String) -> Result<RunValues> {
    let rv: RunValues = serde_json::from_str(&content)?;
    Ok(rv)
}
