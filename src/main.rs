#[macro_use] extern crate runtime_fmt;
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
                match pattern.captures(line.as_str()) {
                    Some(captures) => {
                        let filename = rt_format!(replacement,
                                                  captures.get(0).map_or("", |m| m.as_str()),
                                                  captures.get(1).map_or("", |m| m.as_str()),
                                                  captures.get(2).map_or("", |m| m.as_str()),
                                                  captures.get(3).map_or("", |m| m.as_str()),
                                                  captures.get(4).map_or("", |m| m.as_str()),
                                           captures.get(5).map_or("", |m| m.as_str())).unwrap();
                        let writer = files.entry(filename.clone())
                            .or_insert_with(|| {
                                BufWriter::new(OpenOptions::new()
                                               .create(true)
                                               .write(true)
                                               .truncate(false)
                                               .append(true)
                                               .open(filename.clone())
                                               .expect(&format!("Failed to open file '{}'", &filename)))
                            });
                        writer.write_fmt(format_args!("{}\n", line)).unwrap();
                    },
                    None => println!("{}", line),
                }
            },
            Err(e) => {
                println!("error reading line: {:?}", e);
            },
        }
    }
}
