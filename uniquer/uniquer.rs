use std::env;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("No argument provided");
    }
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);

    let mut line_vec = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        line_vec.push(line);
    }
    let mut unique_line_vec: Vec<String> = Vec::new();
    for line in line_vec.iter() {
        if ! val_in_vec(&line, &mut unique_line_vec) {
            unique_line_vec.push(line.to_string());
        }
    }
    let mut file = File::create(&args[1]).unwrap();

    for line in unique_line_vec.iter() {
        writeln!(&mut file, "{}", line).unwrap();
    }
}
fn val_in_vec(val: &str, vec: &mut Vec<String>) -> bool {
    for unique_line in vec.iter() {
        if unique_line == val {
            return true
        }
    }
    false
}
