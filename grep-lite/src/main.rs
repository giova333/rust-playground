use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use clap::{App, Arg, ArgMatches};
use regex::Regex;

fn main() {
    let args = read_arguments();

    let pattern_argument = args.value_of("pattern").unwrap();
    let regex = Regex::new(pattern_argument).unwrap();
    let input_type = args
        .value_of("file")
        .map(|name| Input::File(name.to_string()))
        .unwrap_or(Input::Stdin);

    match input_type {
        Input::File(val) => {
            let file = File::open(val).unwrap();
            let reader = BufReader::new(file);
            process_lines(reader, regex);
        }
        Input::Stdin => {
            let stdin = io::stdin();
            let reader = stdin.lock();
            process_lines(reader, regex);
        }
    };
}

enum Input {
    File(String),
    Stdin,
}

fn read_arguments() -> ArgMatches<'static> {
    App::new("grep-lite")
        .version("1.0")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("the pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("file")
            .help("the file to search in")
            .takes_value(true)
            .required(false))
        .get_matches()
}

fn process_lines<T: BufRead + Sized>(buf_reader: T, regex: Regex) {
    for line_result in buf_reader.lines() {
        let line = line_result.unwrap();
        if let Some(_) = regex.find(&line) { println!("{}", line); }
    }
}
