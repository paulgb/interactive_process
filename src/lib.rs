use std::io::{BufRead, BufReader, Result, Write};
use std::process::{Child, ChildStdin, Command, ExitStatus, Stdio};
use std::thread;

const ASCII_NEWLINE: u8 = 10;

/// Wraps a [Child] object in an interface for doing newline-dellimited string IO
/// with a child process.
///
/// Calling `send` sends a string to the process's `stdin`. A newline delimiter
/// is automatically appended. If newline characters are present in the provided
/// string, they will _not_ be escaped.
///
/// Each newline-separated string sent by the child process over `stdout` results
/// a call to the provided `line_callback` function. The line is wrapped in a
/// [std::io::Result]; it will be in the `Err` state if the line is not valid
/// UTF-8.
///
/// A callback may optionally be provided (via `new_with_exit_callback`) which is
/// invoked when the child's `stdout` stream is closed.
pub struct InteractiveProcess {
    child: Child,
    stdin: ChildStdin,
}

impl InteractiveProcess {
    /// Attempt to start a process for the provided [Command], capturing the
    /// standard in and out streams for later use. The provided callback is
    /// called for every newline-terminated string written to `stdout` by the
    /// process.
    pub fn new<T>(command: Command, line_callback: T) -> Result<Self>
    where
        T: Fn(Result<String>) + Send + 'static,
    {
        Self::new_with_exit_callback(command, line_callback, || ())
    }

    /// Constructor with the same semantics as `new`, except that an additional
    /// no-argument closure is provided which is called when the client exits.
    pub fn new_with_exit_callback<T, S>(
        mut command: Command,
        line_callback: T,
        exit_callback: S,
    ) -> std::io::Result<Self>
    where
        T: Fn(Result<String>) + Send + 'static,
        S: Fn() + Send + 'static,
    {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child
            .stdout
            .take()
            .expect("Accessing stdout should never fail after passing Stdio::piped().");

        let stdin = child
            .stdin
            .take()
            .expect("Accessing stdin should never fail after passing Stdio::piped().");

        thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                line_callback(line);
            }
            exit_callback();
        });

        Ok(InteractiveProcess { stdin, child })
    }

    /// Send a string to the client process's `stdin` stream. A newline will be
    /// appended to the string.
    pub fn send(&mut self, data: &str) -> std::io::Result<()> {
        self.stdin.write_all(data.as_bytes())?;
        self.stdin.write_all(&[ASCII_NEWLINE])
    }

    /// Send a string to the client process's `stdin` stream, without appending a
    /// newline.
    pub fn send_unterminated(&mut self, data: &str) -> std::io::Result<()> {
        self.stdin.write_all(data.as_bytes())
    }

    /// Consume this `InteractiveProcess` and return its child. This is useful if you
    /// want to take over control of the child, for example, to kill it:
    ///
    ///     proc = InteractiveProces::new(...);
    ///     proc.take().kill().unwrap();
    pub fn take(self) -> Child {
        self.child
    }

    /// Block the current thread on the process exiting, and return the exit code when
    /// it does. This does _not_ send a signal to kill the child, so it only makes
    /// sense when the child process is self-terminating.
    pub fn wait(mut self) -> std::io::Result<ExitStatus> {
        self.child.wait()
    }
}
