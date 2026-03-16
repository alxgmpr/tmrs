# tmrs

CLI utility to measure time performance of commands with averages and standard deviation.

Name inspired by `time`, with the obvious `rs` suffix. Pronounced "timers" (I guess?).

Requires Rust 1.85+ (edition 2024).

## Usage

Follow `tmrs` with the command you want to time:

```shell
$ tmrs ls
avg: 0.003sec
std dev: 0.000sec
```

Use `--` to separate tmrs options from the command's arguments:

```shell
$ tmrs -n 10 -- ls -R
avg: 0.013sec
std dev: 0.006sec
```

With a single run, standard deviation is not available:

```shell
$ tmrs -n 1 echo hello
avg: 0.002sec
std dev: N/A
```

## Options

```
Usage: tmrs [OPTIONS] <COMMAND>...

Arguments:
  <COMMAND>...  The command to time against

Options:
  -n, --number <NUMBER>  Number of runs to average [default: 5]
  -d, --debug            Enable debug logging, log timing for each run
  -v, --verbose          Log the output of the ran commands to stdout
  -h, --help             Print help
  -V, --version          Print version
```

## Installation

From crates.io:

```shell
$ cargo install tmrs
```

From source:

```shell
$ git clone git@github.com:alxgmpr/tmrs.git
$ cd tmrs
$ cargo build --release
$ cp target/release/tmrs /usr/local/bin
```

## License

MIT
