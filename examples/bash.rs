//! This example shows an example of interacting with a
//! `bash` shell session. It opens a `bash` session,
//! sends some command (`echo` and `ls`), and waits for `bash`
//! to respond. Then it closes the stdin stream by
//! calling `close()`, and waits for `bash` to exit.

use interactive_process::InteractiveProcess;
use std::{process::Command, thread::sleep, time::Duration};

fn main() {
    let mut cmd = Command::new("/usr/bin/bash");
    let mut proc = InteractiveProcess::new(&mut cmd, |line| {
        println!("Got: {}", line.unwrap());
    })
    .unwrap();

    sleep(Duration::from_millis(10));

    proc.send("echo 'Hi from bash. Running ls:'").unwrap();
    proc.send("ls").unwrap();

    sleep(Duration::from_millis(10));

    proc.close();
}
