use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

const VERSION: &str = "1.0";

fn print_help() {
    println!("Usage: catd [OPTIONS] FILE");
    println!("Display contents of a file with optional double-spacing and line numbers.");
    println!();
    println!("Options:");
    println!("  -d, --double-space    Double-space the output");
    println!("  -n, --line-numbers    Add line numbers to the output");
    println!("  -v, --version         Show version information");
    println!("  -h, --help            Show this help message");
}

fn print_version() {
    println!("catd version {}", VERSION);
}

fn read_file(filepath: &str) -> io::Result<Vec<String>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn print_lines(lines: Vec<String>, double_space: bool, line_numbers: bool) {
    for (i, line) in lines.iter().enumerate() {
        if line_numbers {
            println!("{:>4}: {}", i + 1, line);
        } else {
            println!("{}", line);
        }
        if double_space {
            println!();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        process::exit(1);
    }

    let mut double_space = false;
    let mut line_numbers = false;
    let mut file_path = String::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-v" | "--version" => {
                print_version();
                return;
            }
            "-d" | "--double-space" => {
                double_space = true;
            }
            "-n" | "--line-numbers" => {
                line_numbers = true;
            }
            _ => {
                file_path = arg.clone();
            }
        }
    }

    if file_path.is_empty() {
        print_help();
        pro

