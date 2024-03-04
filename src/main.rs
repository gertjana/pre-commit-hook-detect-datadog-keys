use std::{
    env::args, error::Error, fs::File, io::Read, process::exit
};

use regex::RegexBuilder;

fn main()  -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();

    let staged_files = args[1..].to_vec();

    let pattern = "^.*ap[ip]_key[\\ ]*=[\\ ]*\\\"[a-f0-9]+\\\".*$";

    let mut result = false;

    for file in staged_files {
        match search_file_for_pattern(&file, pattern) {
            Ok(true) => result = true,
            Ok(false) => (),
            Err(e) => {
                // swallow but print the error 
                eprintln!("Error searching file: {}", e);
            }
        }
    }

    match result {
       false => exit(0),
       _ => exit(1)
    }
}

fn search_file_for_pattern(in_file_str:&str, re_str: &str) -> Result<bool, Box<dyn Error>> {
    let re = RegexBuilder::new(re_str)
        .case_insensitive(true)
        .build()?;

    let mut secrets_found = false;
    let mut in_file = File::open(in_file_str)?;
    let mut buffer = Vec::new();
    in_file.read_to_end(&mut buffer)?;

    let mut start = 0;
    let mut line_number = 0;
    for (index, &byte) in buffer.iter().enumerate() {
        if byte == b'\n' {
            line_number += 1;
            if let Ok(line) = std::str::from_utf8(&buffer[start..index]) {
                for capture in re.captures_iter(line) {
                    if let Some(_) = capture.get(0) {
                        println!("Datadog API/APP key found in: {} , line {}", in_file_str, line_number);
                        secrets_found = true;
                    }
                }
            }
            start = index + 1;
        }
    }
    Ok(secrets_found)

}

#[cfg(test)]
mod tests {
    use super::*;

    const PATTERN: &str = "^.*ap[ip]_key[\\ ]*=[\\ ]*\\\"[a-f0-9]+\\\".*$";

    #[test]
    fn test_search_file_for_pattern() {
        let file = "test_files/test_secrets.txt";
        let secrets_found = search_file_for_pattern(file, PATTERN).unwrap();
        assert_eq!(secrets_found, true);
    }
    #[test]

    fn test_search_file_for_pattern_no_secrets() {
        let file = "test_files/test_no_secrets.txt";
        let secrets_found = search_file_for_pattern(file, PATTERN).unwrap();
        assert_eq!(secrets_found, false);
    }
    #[test]
    fn test_search_file_for_pattern_invalid_file() {
        let file = "test_files/invalid_file.txt";
        let secrets_found = search_file_for_pattern(file, PATTERN);
        assert!(secrets_found.is_err());
    }
}

