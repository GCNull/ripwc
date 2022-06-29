use std::fs::File;
use std::io::{BufReader, Read};

use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser)]
#[clap(author="GCNull", version=env!("CARGO_PKG_VERSION"), about="A rewrite of the GNU coreutils 'wc' tool.", long_about = None)]
struct Args {
    #[clap(value_parser, help="The file or folder to scan", required=true)]
    file: Vec<String>,

    #[clap(short='c', long, help="print the byte counts")]
    bytes: bool,

    #[clap(short='m', long, help="print the character counts")]
    chars: bool,

    #[clap(short='l', long, help="print newline counts")]
    lines: bool,

    #[clap(short='w', long, help="print word counts")]
    words: bool,

    #[clap(short='L', long, help="print maximum display width")]
    max_line_length: bool,

    #[clap(short='r', long, help="recursively search through folders and files")]
    recursive: bool,

    #[clap(short='v', help="print verbose output")]
    verbose: bool,

    #[clap(long="vv", help="print extra verbose output")]
    verbose2: bool,
}

fn main() {
    let mut app: Args = Args::parse();
    let mut result = String::new();
    let mut total_byte_count: usize = 0;
    let mut total_char_count: usize = 0;
    let mut total_word_count: usize = 0;
    let mut total_newline_count: usize = 0;
    // let mut total_max_line_counter: usize = 0;

    // Recursively go through directories. If recursion is false then stop after the first file is found
    for i in &app.file {
        for entry in WalkDir::new(&i) {
            match entry {
                Ok(path) => {
                    if app.verbose && !path.path().is_file() { println!("Searching {:?}", path.path()) }
                    if path.path().is_file() {
                        if app.verbose { println!("Found {:?}", path.path().display()) }
                        let i = path.path();

                        if app.verbose { println!("Opening {:?}", i) }
                        let mut byte_count = 0;
                        let mut char_count = 0;
                        let mut word_count = 0;
                        let mut newline_count = 0;
                        // let mut max_line_counter = 0;

                        if let Ok(open_f) = File::open(i) {
                            let data = open_f.metadata().unwrap().len();
                            let reader = BufReader::new(open_f);
                            for j in reader.bytes() {
                                // Switch flags to true if all false. Default settings on 'wc'
                                if !app.bytes && !app.lines && !app.words && !app.chars {
                                    app.bytes = true;
                                    app.lines = true;
                                    app.words = true;
                                }

                                if app.bytes && !app.lines && !app.words && !app.chars {
                                    byte_count = data as usize;
                                    break;
                                } else if app.bytes { byte_count = data as usize; }

                                // if app.verbose { println!("0x{:X}", &j.as_ref().unwrap()) }

                                if app.chars && j.as_ref().unwrap() != &0x0 {
                                    char_count += 1;
                                    // if app.verbose { println!("Found char 0x{:X}", &j.as_ref().unwrap()) }
                                }
                                if app.lines && j.as_ref().unwrap() == &0xA { // newline counter
                                    newline_count += 1;
                                    // if app.verbose { println!("Found newline 0x{:X}", &j.as_ref().unwrap()) }
                                }
                                if app.words && j.as_ref().unwrap().is_ascii_whitespace() {
                                    word_count += 1;
                                    // if app.verbose { println!("Found word 0x{:X}", &j.as_ref().unwrap()) }
                                }

                                // if app.max_line_length && !j.as_ref().unwrap().is_ascii_whitespace() {
                                //     max_line_counter += 1;
                                // }
                                // if app.max_line_length && j.as_ref().unwrap().is_ascii_whitespace() && max_line_counter > max_line_length {
                                //     max_line_length = max_line_counter;
                                //     max_line_counter = 0;
                                // }
                            }
                            total_byte_count += byte_count;
                            total_char_count += char_count;
                            total_word_count += word_count;
                            total_newline_count += newline_count;
                            // mut total_max_line_counter += max_line_counter;
                        }

                        if newline_count > 0 { result.push_str(&format!("newlines: {}", newline_count)) }
                        if word_count > 0 { result.push_str(&format!(" words: {}", word_count)) }
                        if char_count > 0 { result.push_str(&format!(" chars: {}", char_count)) }
                        if byte_count > 0 { result.push_str(&format!(" bytes: {}", byte_count)) }
                        // if max_line_length > 0 { result.push_str(&format!(" max line length: {}", max_line_length)) }
                        println!("{} {:?}", result.trim(), i);
                        result.clear();
                        if !app.recursive { break }
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        if !app.recursive && app.file.len() > 1 { continue } else if !app.recursive && app.file.len() == 1 { break }
    }
    if total_newline_count > 0 { result.push_str(&format!("newlines: {}", total_newline_count)) }
    if total_word_count > 0 { result.push_str(&format!(" words: {}", total_word_count)) }
    if total_char_count > 0 { result.push_str(&format!(" chars: {}", total_char_count)) }
    if total_byte_count > 0 { result.push_str(&format!(" bytes: {}", total_byte_count)) }
    // if max_line_length > 0 { result.push_str(&format!(" max line length: {}", max_line_length)) }
    println!("\n{} total", result.trim());
}