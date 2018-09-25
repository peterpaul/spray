extern crate regex;

use regex::Regex;

use std::io::{self, BufRead, BufWriter, Write};
use std::env;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};

fn main() {
    let args: Vec<String> = env::args().collect();
    let stdin = io::stdin();
    let reader = stdin.lock();
    let pattern = Regex::new(args[1].as_str()).unwrap();
    let replacement = args[2].as_str();
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let filename = pattern.replace_all(line.as_str(), replacement).into_owned();
                let handle = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(false)
                    .append(true)
                    .open(filename)
                    .expect("Failed to open file");
                let mut writer = BufWriter::new(handle);
                writeln!(writer, "{}", line).unwrap();
            },
            Err(e) => {
                println!("error reading line: {:?}", e);
            },
        }
    }
}
