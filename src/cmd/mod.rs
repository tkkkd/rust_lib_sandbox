// use std::io::{self, Write};
use std::process::{Command, Output, Stdio};
pub fn ls(args: &[String]) -> Output {
    let output = Command::new("ls")
        .args(args)
        // .stdout(Stdio::piped())
        // .stderr(Stdio::piped())
        .output()
        .expect("ls command failed to start");

    // println!("{}", String::from_utf8_lossy(&output.stdout))
    // println!("status: {}", output.status);
    // io::stdout().write_all(&output.stdout).unwrap();
    // io::stderr().write_all(&output.stderr).unwrap();
    output
}
