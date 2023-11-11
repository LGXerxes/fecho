use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

use clap::Parser;

/// A very cool tool
#[derive(Parser, Debug)]
#[command(author = "LGXerxes", version, about)]
struct Args {
    /// What should be repeated
    // #[arg(short, long)]
    #[arg(required_if_eq("file", "true"))]
    input: Vec<String>,

    /// Path to the file to be processed
    #[arg(short, long)]
    file: bool,

    /// Quantity of repetitions
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Optional spacing, if no argument given [\n]
    #[arg(short, long)]
    white_space: Option<Option<String>>,

    /// Return only the first {TOP} lines
    #[arg(short, long)]
    top: Option<usize>,
}

fn main() {
    let args = Args::parse();

    if args.file {
        process_files(&args);
    } else if args.input.is_empty() {
        process_stdin(&args);
    } else {
        process_direct_input(&args);
    }
}

fn process_files(args: &Args) {
    let mut errors = Vec::new();
    for file_path in &args.input {
        if let Err(err) = fs::File::open(file_path) {
            errors.push((file_path, err));
        }
    }

    if !errors.is_empty() {
        report_errors(&errors);
        return;
    }

    for i in 0..args.count {
        for file_path in &args.input {
            process_input_source(file_path, args);
            if args.count - 1 > i {
                print_separator(args);
            }
        }
    }
}

fn process_stdin(args: &Args) {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Buffer the entire stdin, as we might need to multiple times over it.
    let lines: Vec<_> = handle.lines().collect();

    for i in 0..args.count {
        let mut k = 0;
        for line in &lines {
            if args.top.is_some_and(|x| k >= x) {
                break;
            }
            k += 1;
            match line {
                Ok(line) => println!("{}", line),
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
        if args.count - 1 > i {
            print_separator(args);
        }
    }
}

fn process_direct_input(args: &Args) {
    let input_combined = args.input.join(" ");
    for i in 0..args.count {
        println!("{}", input_combined);
        if args.count - 1 > i {
            print_separator(args);
        }
    }
}

fn process_input_source<P: AsRef<Path>>(source: P, args: &Args) {
    if let Ok(lines) = read_lines(source) {
        for (i, line) in lines.enumerate() {
            if args.top.is_some_and(|x| x <= i + 1) {
                break;
            }
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_separator(args: &Args) {
    if let Some(separator_option) = args.white_space.as_ref() {
        let separator = separator_option.as_deref().unwrap_or("");
        println!("{}", separator);
    }
}

fn report_errors(errors: &[(&String, io::Error)]) {
    println!("Stopping operation, there were errors opening the following files:");
    for (file, err) in errors {
        println!("{:10} |💥 Error: {}", file, err);
    }
}
