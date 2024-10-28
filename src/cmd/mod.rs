use std::ffi::OsStr;
use std::process::{Command, Output};
pub fn ls<T: AsRef<OsStr> + ?Sized>(args: &[&T]) -> Output {
    let output = Command::new("ls")
        .args(args)
        .output()
        .expect("ls command failed to start");

    output
}
