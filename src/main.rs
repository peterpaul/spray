extern crate regex;

use regex::Regex;

use std::io::{self, BufRead, BufWriter, Write};
use std::env;
use std::collections::HashMap;
use std::fs::OpenOptions;

fn main() {
    let args: Vec<String> = env::args().collect();
    let stdin = io::stdin();
    let reader = stdin.lock();
    let pattern = Regex::new(args[1].as_str()).unwrap();
    let replacement = args[2].as_str();
    let mut files = HashMap::new();
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                if pattern.is_match(line.as_str()) {
                    let filename = pattern.replace_all(line.as_str(), replacement).into_owned();
                    let writer = files.entry(filename.clone())
                        .or_insert_with(|| {
                            BufWriter::new(OpenOptions::new()
                                           .create(true)
                                           .write(true)
                                           .truncate(false)
                                           .append(true)
                                           .open(filename.clone())
                                           .expect("Failed to open file"))
                        });
                    writer.write_fmt(format_args!("{}\n", line)).unwrap();
                } else {
                    println!("{}", line);
                }
            },
            Err(e) => {
                println!("error reading line: {:?}", e);
            },
        }
    }
}
