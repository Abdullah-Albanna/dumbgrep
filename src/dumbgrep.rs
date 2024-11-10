use std::{
    collections::BTreeMap,
    env::args,
    fs,
    io::{self, ErrorKind},
};

mod errors;

use errors::MiniGrepError;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new() -> Result<Self, MiniGrepError> {
        let args: Vec<String> = args().collect();

        let text_to_search = args.get(1).cloned().ok_or(MiniGrepError::QueryParseError)?;

        let file_path = args.get(2).cloned().ok_or(MiniGrepError::FileParseError)?;

        // let file_path = args.get(2)
        //                         .map(|text| text.clone())
        //                         .expect("Failed to get the file path");

        Ok(Self {
            query: text_to_search,
            file_path: file_path,
        })
    }
}

pub fn run(config: &Config) -> Result<(), String> {
    let file_content = get_file_content(&config.file_path).unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            format!("{} does not exists", &config.file_path)
        } else {
            format!("couldn't read the content of the file, error {}", err)
        }
    });

    // println!(
    //     "\nSearching for '{}' in '{}'\n",
    //     config.query, config.file_path
    // );
    // println!("Contents: \n'{}'\n", file_content.trim());

    let matches = get_match(&config.query, &file_content);

    for (line, text) in &matches {
        println!("\n{}: {}\n\n", line, text);
    }

    Ok(())
}

fn get_file_content(file_path: &str) -> Result<String, io::Error> {
    let file_contents = fs::read_to_string(file_path)?;

    Ok(file_contents)
}

fn get_match(query: &str, content: &str) -> BTreeMap<usize, String> {
    let mut found_lines: BTreeMap<usize, String> = BTreeMap::new();

    for (i, item) in content.lines().enumerate() {
        if item.contains(query) {
            // replaces the query text to be colored
            let colored_item = item.replace(query, &format!("\x1b[1;31m{}\x1b[0m", query).as_str());
            found_lines.insert(i, colored_item);
        }
    }

    return found_lines;
}
