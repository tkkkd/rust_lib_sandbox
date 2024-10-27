pub mod cmd;

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Command, Output, Stdio};

    #[test]
    fn it_works() {
        let args = [String::from("-a")];
        let output = cmd::ls(&args);

        assert!(output.status.success());
        // assert_eq!(
        //     String::from_utf8_lossy(&output.stdout),
        //     ".\n..\n.git\n.gitignore\nCargo.lock\nCargo.toml\nREADME.md\nsrc\ntarget\n"
        // );
    }

    // #[test]
    // fn grep_ls() {
    //     let ls_output = cmd::ls(&[]);

    //     let output = Command::new("grep")
    //         .arg("Cargo.toml")
    //         .stdin(String::from_utf8_lossy(&ls_output.stdout))
    //         .output();
    //     assert!(output.status.success());
    //     assert_eq!(String::from_utf8_lossy(&output.stdout), "Cargo.toml");
    // }
}
