use std::fs::File;
use std::io::{BufReader, Read};

use clap::Parser;
use rayon::prelude::*;
use walkdir::WalkDir;

#[derive(Parser)]
#[clap(author="GCNull", version=env!("CARGO_PKG_VERSION"), about="A rewrite of the GNU coreutils 'wc' tool.", long_about = None)]
struct Args {
    #[clap(help="The file or folder to scan", required=true)]
    file: String,

    #[clap(short='c', long, help="print the byte counts")]
    bytes: bool,

    #[clap(short='w', long, help="print word counts")]
    words: bool,

    #[clap(short='l', long, help="print newline counts")]
    lines: bool,

    #[clap(short='m', long, help="print the character counts")]
    chars: bool,

    #[clap(short='L', long, help="print maximum display width")]
    max_line_length: bool,

    #[clap(short='r', long, help="recursively search through folders and files")]
    recursive: bool,

    #[clap(short='v', long, help="print verbose output")]
    verbose: bool,
}

fn main() {
    let app = Args::parse();
    let mut files = Vec::new();

    // Recursively go through directories. If recursion is false then stop after the first file is found
    for entry in WalkDir::new(&app.file) {
        match entry {
            Ok(path) => {
                if path.path().is_file() {
                    files.push(path.path().display().to_string());
                    if !app.recursive { break }
                }
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
    // files.sort();

    // let mut max_line_length = 0;
    files.into_par_iter().for_each(|i| {
        let mut app = Args::parse();
        if app.verbose { println!("Opening {:?}", i) }
        let mut result = String::new();
        let mut byte_count = 0;
        let mut char_count = 0;
        let mut word_count = 0;
        let mut newline_count = 0;
        // let mut max_line_counter = 0;

        if let Ok(open_f) = File::open(&i) {
            let reader = BufReader::new(open_f);
            for j in reader.bytes() {
                if app.verbose { println!("{:?}", j) }
                // Switch flags to true if all false. Default settings on 'wc'
                if !app.bytes && !app.lines && !app.words {
                    app.bytes = true;
                    app.lines = true;
                    app.words = true;
                }

                if app.bytes { byte_count += 1; }
                if app.chars && j.as_ref().unwrap().is_ascii() {
                    char_count += 1;
                }
                if app.lines && j.as_ref().unwrap() == &0xA { // newline counter
                    newline_count += 1;
                }
                if app.words && j.as_ref().unwrap().is_ascii_whitespace() {
                    word_count += 1;
                }
                // if app.max_line_length && !j.as_ref().unwrap().is_ascii_whitespace() {
                //     max_line_counter += 1;
                // }
                // if app.max_line_length && j.as_ref().unwrap().is_ascii_whitespace() && max_line_counter > max_line_length {
                //     max_line_length = max_line_counter;
                //     max_line_counter = 0;
                // }
            }
        }
        if byte_count > 0 { result.push_str(&format!("bytes: {}", byte_count)) }
        if char_count > 0 { result.push_str(&format!(" chars: {}", char_count)) }
        if word_count > 0 { result.push_str(&format!(" words: {}", word_count)) }
        if newline_count > 0 { result.push_str(&format!(" newlines: {}", newline_count)) }
        // if max_line_length > 0 { result.push_str(&format!(" max line length: {}", max_line_length)) }
        println!("{:?} {}", i, result.trim());
    });
}