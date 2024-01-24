# tmrs

CLI utility to measure time performance of commands with things like averages and standard deviation built in.

Name inspired by `time`, with the obvious `rs` suffix. Pronounced "timers" (I guess?).

### Usage

Just follow `tmrs` with the command you want to time

```shell
$ tmrs sleep 1

# output:
# avg: 1.014sec
# std dev: 0.006sec
```

### Installation

Build it yourself:

```shell
$ git clone git@github.com:alxgmpr/tmrs.git
$ cd tmrs
$ cargo build --release
$ cp target/release/tmrs /usr/local/bin # or wherever you want
```

### License

MIT

