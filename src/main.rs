use std::fs::OpenOptions;
use std::io::{BufReader, Read};

use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser)]
#[clap(author="GCNull", version=env!("CARGO_PKG_VERSION"), about="A rewrite of the GNU coreutils 'wc' tool.", long_about = None)]
struct Args {
    #[clap(value_parser, help="The file or folder to read", required=true)]
    file: Vec<String>,

    #[clap(short='c', long, help="Print the byte counts")]
    bytes: bool,

    #[clap(short='m', long, help="Print the character counts")]
    chars: bool,

    #[clap(short='l', long, help="Print newline counts")]
    lines: bool,

    #[clap(short='w', long, help="Print word counts")]
    words: bool,

    #[clap(short='L', long, help="Print maximum display width")]
    max_line_length: bool,

    #[clap(short='r', long, help="Recursively search through folders and files")]
    recursive: bool,

    #[clap(short='v', help="Print verbose output")]
    verbose: bool,

    #[clap(long="vv", help="Print extra verbose output")]
    verbose2: bool,
}

fn main() {
    let mut app: Args = Args::parse();
    let mut result = String::new();
    let mut total_byte_count: usize = 0;
    let mut total_char_count: usize = 0;
    let mut total_word_count: usize = 0;
    let mut total_newline_count: usize = 0;
    let mut largest_maxline_length: usize = 0;

    // Switch flags to true if all false. Default settings on 'wc'
    if !app.bytes && !app.lines && !app.words && !app.chars && !app.max_line_length {
        app.bytes = true;
        app.lines = true;
        app.words = true;
    }
    let mut file_opts = OpenOptions::new();
    let mut written: bool = false;

    // Recursively go through directories. If recursion is false then stop after the first file is found
    for i in &app.file {
        for entry in WalkDir::new(&i).sort_by_file_name() {
            match entry {
                Ok(path) => {
                    if app.verbose {
                        println!("Found {:?}", path.path().display())
                    }
                    let i = path.path();

                    if app.verbose {
                        println!("Opening {:?}", i)
                    }
                    let mut byte_count = 0;
                    let mut char_count = 0;
                    let mut word_count = 0;
                    let mut newline_count = 0;
                    let mut maxline_length = 0;

                    if let Ok(open_f) = file_opts.read(true).open(i) {
                        let data = open_f.metadata().unwrap().len();
                        let mut reader = BufReader::with_capacity(4096, open_f);
                        let mut buffer = vec![0; data as usize];

                        if let Ok(j) = reader.read(&mut buffer) {
                            if j == 0 {
                                println!("0 bytes {:?}", i);
                            }

                            let mut temp: usize = 0;
                            for byte in &buffer {
                                if app.bytes && !app.lines && !app.words && !app.chars {
                                    byte_count = data as usize;
                                    break;
                                } else if app.bytes && !written {
                                    byte_count = data as usize; written = true;
                                }

                                if app.chars && byte != &0 {
                                    char_count += 1;
                                }

                                if app.lines && byte == &0xA { // newline counter
                                    newline_count += 1;
                                }

                                if app.words && byte.is_ascii_whitespace() {
                                    word_count += 1;
                                }

                                if app.max_line_length {
                                    if byte == &0xA && temp >= maxline_length {
                                        maxline_length = temp;

                                        if maxline_length >= largest_maxline_length {
                                            largest_maxline_length = maxline_length;
                                        }
                                    }
                                    temp += 1;
                                }
                            }

                            total_byte_count += byte_count;
                            total_char_count += char_count;
                            total_word_count += word_count;
                            total_newline_count += newline_count;
                        }
                    }

                    if newline_count > 0 { result.push_str(&format!("newlines: {}", newline_count)) }
                    if word_count > 0 { result.push_str(&format!(" words: {}", word_count)) }
                    if char_count > 0 { result.push_str(&format!(" chars: {}", char_count)) }
                    if byte_count > 0 { result.push_str(&format!(" bytes: {}", byte_count)) }
                    if maxline_length > 0 { result.push_str(&format!(" max line length: {}", maxline_length)) }

                    println!("{} {:?}", result.trim(), i);
                    result.clear();
                    if !app.recursive {
                        break
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        if !app.recursive && app.file.len() > 1 { continue }
        else if !app.recursive && app.file.len() == 1 { break }
    }
    if total_newline_count > 0 { result.push_str(&format!("newlines: {}", total_newline_count)) }
    if total_word_count > 0 { result.push_str(&format!(" words: {}", total_word_count)) }
    if total_char_count > 0 { result.push_str(&format!(" chars: {}", total_char_count)) }
    if total_byte_count > 0 { result.push_str(&format!(" bytes: {}", total_byte_count)) }
    if largest_maxline_length > 0 { result.push_str(&format!(" max line length: {}", largest_maxline_length)) }

    println!("\n{} total", result.trim());
}
