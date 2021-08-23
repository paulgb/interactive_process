use interactive_process::InteractiveProcess;
use std::{process::Command, thread::sleep, time::Duration};

fn main() {
    let cmd = Command::new("examples/echo_stream.py");
    let mut proc = InteractiveProcess::new(cmd, |line| {
        println!("Got: {}", line.unwrap());
    })
    .unwrap();

    proc.send("data1").unwrap();
    sleep(Duration::from_secs(1));
    proc.send("data2").unwrap();
    sleep(Duration::from_secs(1));
    proc.send("data3").unwrap();

    // If we don't sleep here, the process won't have time to reply
    // before we kill it.
    sleep(Duration::from_millis(1));

    proc.close().kill().unwrap();
}
