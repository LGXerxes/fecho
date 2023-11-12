# fecho

`echo` made in rust, which can take files as input with the `-f` flag

### Installation

```
cargo install fecho
```

## Options

```
A simple tool to echo multiple files, or text, or piped values

Usage: fecho [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...  What should be repeated

Options:
  -f, --file                     [INPUT] becomes a list of files you want to fecho
  -c, --count <COUNT>            Quantity of repetitions [default: 1]
  -s, --separator [<SEPARATOR>]  Optional separator, newline if no argument is given
  -t, --top <TOP>                Return display the first [TOP] lines of each echo
  -h, --help                     Print help
  -V, --version                  Print version
```

## Examples

Using fecho to get the first two lines of a file:

```
fecho Cargo.toml -f -c 2 -t 2

[package]
name = "fecho"
[package]
name = "fecho"
```

Or multiple files

```
fecho Cargo.toml src/main.rs -f -c 2 -t 2 -s ";🦄;"

[package]
name = "fecho"
;🦄;
use std::{
    error::Error,
;🦄;
[package]
name = "fecho"
;🦄;
use std::{
    error::Error,
```

You can also pipe it through stdin:

```
fecho Cargo.toml -f | ./fecho -c 2 -t 2

[package]
name = "fecho"
[package]
name = "fecho"
```

Or just:

```
fecho 'Hello World!' -c 3 -s 'Hello User!'

Hello World!
Hello User!
Hello World!
Hello User!
Hello World!
```
