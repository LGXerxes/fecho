use std::{
    error::Error,
    fs,
    io::{self, BufRead, Error as IoError},
    path::Path,
};

use clap::Parser;

/// A simple tool to echo multiple files, text, or piped values.
#[derive(Parser, Debug)]
#[command(author = "LGXerxes", version, about)]
struct Args {
    /// What should be repeated,
    // #[arg(short, long)]
    #[arg(required_if_eq("file", "true"))]
    input: Vec<String>,

    /// [INPUT] becomes a list of files you want to fecho.
    #[arg(short, long)]
    file: bool,

    /// Quantity of repetitions.
    #[arg(short, long, default_value_t = 1)]
    number: usize,

    /// Allows for continuous returning of the input.
    /// --top decides when the separator should be printed.
    /// Only effective in stdin mode.
    #[arg(short, long)]
    continuous: bool,

    /// Optional separator, newline if no argument is given
    #[arg(short, long, allow_hyphen_values = true)]
    separator: Option<Option<String>>,

    /// Return display the first [TOP] lines of each echo.
    #[arg(short, long)]
    top: Option<usize>,
}

fn main() -> Result<(), FechoError> {
    let args = Args::parse();
    if atty::is(atty::Stream::Stdin) {
        println!("fecho: reading from stdin, press Ctrl+D to stop")
    }

    if args.file {
        process_files(&args)?;
    } else if args.input.is_empty() {
        process_stdin(&args)?;
    } else {
        process_direct_input(&args);
    }

    Ok(())
}

fn process_files(args: &Args) -> Result<(), FechoError> {
    let mut errors = vec![];

    for file_path in &args.input {
        if let Err(err) = fs::File::open(file_path) {
            errors.push((file_path, err));
        }
    }

    if !&errors.is_empty() {
        report_errors(&errors);
        return Err(FechoError::AccessingFilesError);
    }

    for i in 0..args.number {
        for (k, file_path) in args.input.iter().enumerate() {
            process_input_source(file_path, args);

            if k < args.input.len() - 1 || i < args.number - 1 {
                print_separator(args);
            }
        }
    }
    Ok(())
}

fn process_stdin(args: &Args) -> Result<(), FechoError> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // --top decides when the separator should be printed
    if args.continuous {
        let mut top = args.top;
        for line in handle.lines() {
            let line = line.map_err(|x| x)?;
            println!("{}", line);
            if let Some(top) = top.as_mut() {
                *top -= 1;
            }
            if top.is_some_and(|x| x <= 0) {
                print_separator(args);
                top = args.top;
                continue;
            }
        }
    } else {
        let lines: Result<Vec<_>, IoError> = handle.lines().collect();
        let lines = lines.map_err(|x| x)?;

        for k in 0..args.number {
            let mut i = 0;
            for line in &lines {
                if args.top.is_some_and(|x| x <= i) {
                    break;
                }
                i += 1;
                println!("{}", line);
            }
            if args.number - 1 > k {
                print_separator(args);
            }
        }
    }

    Ok(())
}

fn process_direct_input(args: &Args) {
    let input_combined = args.input.join(" ");
    for i in 0..args.number {
        println!("{}", input_combined);
        if args.number - 1 > i {
            print_separator(args);
        }
    }
}

fn process_input_source<P: AsRef<Path>>(source: P, args: &Args) {
    if let Ok(lines) = read_lines(source) {
        for (i, line) in lines.enumerate() {
            if args.top.is_some_and(|x| x <= i) {
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
    if let Some(separator_option) = args.separator.as_ref() {
        let separator = separator_option.as_deref().unwrap_or("");
        println!("{}", separator);
    }
}

fn report_errors(errors: &[(&String, io::Error)]) {
    for (file, err) in errors {
        println!("{:20} |💥 Error: {}", file, err);
    }
}

#[derive(Debug)]
enum FechoError {
    IoError(io::Error),
    AccessingFilesError,
}
impl Error for FechoError {}

impl From<io::Error> for FechoError {
    fn from(err: io::Error) -> Self {
        FechoError::IoError(err)
    }
}

impl std::fmt::Display for FechoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FechoError::IoError(ref err) => write!(f, "IO error: {}", err),
            FechoError::AccessingFilesError => write!(f, "Process aborted, see above for details"),
        }
    }
}
