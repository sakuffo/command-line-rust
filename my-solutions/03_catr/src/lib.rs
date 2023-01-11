use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)] // This is so we can print out the struct
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.1")
        .author("Saku + Keny")
        .about("Rust Cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number-lines")
                .help("Number lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank-lines")
                .help("Number nonblank lines")
                .takes_value(false)
                .conflicts_with("number"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        // open the file at path filename with the open function safe code only
        let mut reader = open(&filename)?;
        let mut line_number = 1;
        // read the file line by line
        for line in reader.lines() {
            let line = line?;
            if config.number_lines {
                println!("{} {}", line_number, line);
            } else if config.number_nonblank_lines {
                if line.len() > 0 {
                    println!("{} {}", line_number, line);
                } else {
                    println!("{}", line);
                }
            } else {
                println!("{}", line);
            }
            line_number += 1;
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
