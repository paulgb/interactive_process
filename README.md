# `interactive_process`

[![crates.io](https://img.shields.io/crates/v/interactive_process.svg)](https://crates.io/crates/interactive_process)
[![docs.rs](https://img.shields.io/badge/docs-release-brightgreen)](https://docs.rs/interactive_process/)

A tiny Rust library for interacting with a running process over `stdio`.

A common pattern in Unix is to have programs that either produce or consume
newline-delimited text over standard input and output (`stdio`) streams.

This crate provides a light wrapper (really light, look at [src/lib.rs](src/lib.rs))
that provides a tidy little abstraction for this pattern on top of Rust's
built-in `std::process`. Besides `std`, this crate has no dependencies.

## Usage

The examples in [examples/](examples/) are instructive. For example, here's
`echo_stream.rs`:

```rust
use interactive_process::InteractiveProcess;
use std::{process::Command, thread::sleep, time::Duration};

fn main() {
    /// Use Rust's built-in `std::process` to construct a `Command`.
    /// `examples/echo_stream.py` repeats back lines sent to it,
    /// prefixed with "echo: ".
    let cmd = Command::new("examples/echo_stream.py");

    /// Pass this command to `InteractiveProcess`, along with a
    /// callback. In this case, we'll print every line that the
    /// process prints to `stdout`, prefixed by "Got: ".
    let mut proc = InteractiveProcess::new(cmd, |line| {
        println!("Got: {}", line.unwrap());
    })
    .unwrap();

    /// Send some data, waiting in between.
    /// The result of this is "Got: echo: data1" being printed by our callback,
    /// since our callback preprends "Got: " and the child process prepends
    /// "echo: ".
    proc.send("data1").unwrap();

    /// Sleep in this thread. Note that the process' `stdout` is processed in
    /// another thread, so while this thread sleeps, that thread will pick
    /// up the message printed by the child process and run the callback.
    sleep(Duration::from_secs(1));

    /// Repeat that a few more times, for kicks.
    proc.send("data2").unwrap();
    sleep(Duration::from_secs(1));
    proc.send("data3").unwrap();

    // If we don't sleep here, the process won't have time to reply
    // before we kill it.
    sleep(Duration::from_millis(1));

    /// We're done with the process, but it is not self-terminating,
    /// so we can't use `proc.wait()`. Instead, we'll take the `Child` from
    /// the `InteractiveProcess` and kill it ourselves.
    proc.take().kill().unwrap();
}
```

## Limitations

I've tested this for simple things on Linux, but it's not battle-tested and I
haven't tested it on other platforms. If you encounter issues, please open an
issue and I'll do my best to work through it with you.