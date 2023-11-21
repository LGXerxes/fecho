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
fecho Cargo.toml -f -n 2 -t 2

[package]
name = "fecho"
[package]
name = "fecho"
```

Or multiple files

```
fecho Cargo.toml src/main.rs -f -n 2 -t 2 -s ";🦄;"

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
fecho Cargo.toml -f | fecho -n 2 -t 2

[package]
name = "fecho"
[package]
name = "fecho"
```

Or just:

```
fecho 'Hello World!' -n 3 -s 'Hello User!'

Hello World!
Hello User!
Hello World!
Hello User!
Hello World!
```
When fecho is used to read stdin you can use the `--continuos` flag to make it read stdin continuously:\
Which doesn't support the `-n` flag.\
And makes the -t flag dictate the quantity of lines between each separator.

```
ping archlinux.org -c 6 | fecho -c -t 2 -s "=============="

PING archlinux.org (xx.xxx.xxx.xxx) 56(84) bytes of data.
64 bytes from archlinux.org (xx.xxx.xxx.xxx): icmp_seq=1 ttl=53 time=30.6 ms
==============
64 bytes from archlinux.org (xx.xxx.xxx.xxx): icmp_seq=2 ttl=53 time=33.5 ms
64 bytes from archlinux.org (xx.xxx.xxx.xxx): icmp_seq=3 ttl=53 time=33.2 ms
==============
64 bytes from archlinux.org (xx.xxx.xxx.xxx): icmp_seq=4 ttl=53 time=32.7 ms
64 bytes from archlinux.org (xx.xxx.xxx.xxx): icmp_seq=5 ttl=53 time=30.6 ms
==============
64 bytes from archlinux.org (xx.xxx.xxx.xxx): icmp_seq=6 ttl=53 time=33.0 ms
```
Comparing it to the normal output:\
Where it will take the entire output and only then print it.
```
ping archlinux.org -c 6 | fecho -t 2 -n 3 -s "=============="

PING archlinux.org (xx.xxx.xxx.xxx) 56(84) bytes of data.
64 bytes from archlinux.org (xx.xxx.xxx.xxx): icmp_seq=1 ttl=53 time=44.3 ms
==============
PING archlinux.org (xx.xxx.xxx.xxx) 56(84) bytes of data.
64 bytes from archlinux.org (xx.xxx.xxx.xxx): icmp_seq=1 ttl=53 time=44.3 ms
==============
PING archlinux.org (xx.xxx.xxx.xxx) 56(84) bytes of data.
64 bytes from archlinux.org (xx.xxx.xxx.xxx): icmp_seq=1 ttl=53 time=44.3 ms
``````

## License
MIT