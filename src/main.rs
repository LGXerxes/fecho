use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Lines},
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

    if args.input.is_empty() {
        eprintln!("Nothing given to work with");
        return;
    }

    if args.file {
        let mut errors: Vec<(&String, io::Error)> = vec![];

        for file_path in &args.input {
            match fs::File::open(file_path) {
                Ok(_) => (),
                Err(err) => errors.push((file_path, err)),
            }
        }
        if !errors.is_empty() {
            println!("There were errors opening the following files:");
            for (file, err) in errors {
                println!("File: {:10} |💥 Error: {}", file, err);
            }
            return;
        }

        for k in 0..args.count {
            for file_path in &args.input {
                let line_buf = read_lines(&file_path).unwrap();
                for (i, line) in line_buf.enumerate() {
                    if args.top.is_some_and(|x| x <= i + 1) {
                        break;
                    }
                    println!("{}", line.unwrap());
                }
                if args.count - 1 > k {
                    if let Some(separator_option) = args.white_space.as_ref() {
                        let separator = separator_option.as_deref().unwrap_or("");
                        println!("{}", separator);
                    }
                }
            }
        }
        return;
    }

    let input_combined = args.input.join(" ");
    let i = args.count;
    for _ in 0..args.count {
        println!("{}", input_combined);

        // if args.top.is_some_and(|x| x <= i - 1) {
        //     if let Some(separator_option) = args.white_space.as_ref() {
        //         let separator = separator_option.as_deref().unwrap_or("");
        //         println!("{}", separator);
        //     }
        // }

        if let Some(separator) = args.white_space.clone() {
            if let Some(sep) = separator {
                println!("{}", sep)
            } else {
                println!("")
            }
        }
    }
}

fn print_line(args: Args, line: Lines<BufReader<File>>) {
    for (i, line) in line.enumerate() {
        if args.top.is_some_and(|x| x <= i + 1) {
            break;
        }
        println!("{}", line.unwrap());
    }
}

fn print_lines_files(args: Args) {
    for _ in 0..args.count {
        for file_path in &args.input {
            // let iterated_lines = fs::read
            let contents = fs::read_to_string(&file_path);
            match contents {
                Ok(content) => {
                    print_content(&content, &args.white_space, args.top);
                }
                Err(_) => println!("--FAILED TO OPEN {file_path} --"), // This should not really happen
            }
        }
    }
}
fn print_content(content: &str, white_space: &Option<Option<String>>, top: Option<usize>) {
    if let Some(separator_option) = white_space.as_ref() {
        let separator = separator_option.as_deref().unwrap_or("");

        println!("{}", separator);
    }
    if let Some(head_size) = top {
        let split_content: Vec<&str> = content.split("\n").collect();
        for i in 0..head_size {
            if let Some(text) = split_content.get(i) {
                println!("{}", text);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
