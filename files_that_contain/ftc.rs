use std::fs::{metadata, File};
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let stdin = io::stdin();
    // iterate lines with buffer (one line at the time)
    for line in stdin.lock().lines() {
        let file_path = line.unwrap();

        // if directory, continue
        if metadata(file_path.clone()).unwrap().is_dir() {
            continue;
        }
        // iterate file content
        let file = File::open(file_path.clone()).unwrap();
        let reader = BufReader::new(file);
        for (_, line) in reader.lines().enumerate() {
            match line {
                Ok(line_text) => {
                    println!("file: {} => yay first line!!! {}", file_path, line_text);
                    break;
                }
                Err(_) => {
                    println!("Can't read {}", file_path);
                    break;
                }
            }
        }
    }
    println!("END");
}
