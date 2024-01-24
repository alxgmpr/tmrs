# tmrs

CLI utility to measure time performance of commands with things like averages and standard deviation built in.

Name inspired by `time`, with the obvious `rs` suffix. Pronounced "timers" (I guess?).

### Usage

Just follow `tmrs` with the command you want to time:

```shell
$ tmrs ls
avg: 0.003sec
std dev: 0.000sec
```

To pass arguments, separate the command from the arguments with `--`:

```shell
$ tmrs -- ls -R
avg: 0.013sec
std dev: 0.006sec
```

### Options

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

### Installation

Install directly from crates.io and have package built on your machine (takes just a sec!):

```shell
$ cargo install tmrs
```

Clone and build from source:

```shell
$ git clone git@github.com:alxgmpr/tmrs.git
$ cd tmrs
$ cargo build --release
$ cp target/release/tmrs /usr/local/bin # or wherever you want
```

### License

MIT
