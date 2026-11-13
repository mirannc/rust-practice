use clap::(
use clap::Arg;App, Arg);
use std::error::Error;

type MyReselt<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Read head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
            .short("n")
            .long("number")
            .help("Number lines")
            .takes_value(false)
            .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false)
        )
        .get_matches();
}