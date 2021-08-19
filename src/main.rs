use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};


fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}: - {}", index + 1, line.trim()),
            None => ()
        }
    }
}
fn main() {
    let matches = App::new("Grep-lite")
        .version("1.0")
        .author("M. <maurimura@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("pattern")
            .short("p")
            .long("pattern")
            .help("Pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .help("File to search")
            .takes_value(true))
        .get_matches();
        
    let pattern = matches.value_of("pattern").unwrap();
    let input = matches.value_of("input").unwrap_or("-");
    let re = Regex::new(pattern).unwrap();

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re)
    };
}

