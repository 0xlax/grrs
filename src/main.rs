#![allow(unused)]
use clap::Parser;
use std::env::args;
use std::io::{ BufReader,Read };
use std::path::PathBuf;
use std::fs::File;




#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: PathBuf,
}


fn main() {
    let args = Cli::parse();

    let f = File::open(&args.path).expect("Could not read file");

    let mut buf_reader = BufReader::new(f);

    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).expect("_BUFFER_ERROR_");



    fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
        for line in contents.lines() {
            if line.contains(&args.pattern) {
                println!("{}", line);
            }
        }
    }



}




