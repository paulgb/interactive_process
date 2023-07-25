use interactive_process::InteractiveProcess;
use std::process::Command;

fn main() {
    let mut cmd = Command::new("examples/closing_stream.py");
    let proc = InteractiveProcess::new_with_exit_callback(
        &mut cmd,
        |line| {
            println!("Got: {}", line.unwrap());
        },
        || println!("Child exited."),
    )
    .unwrap();

    println!("{}", proc.wait().unwrap());
}
