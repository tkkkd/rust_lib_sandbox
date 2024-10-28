pub mod cmd;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_str() {
        let args = ["-a"];
        let output = cmd::ls(&args);

        assert!(output.status.success());
    }

    #[test]
    fn it_works_string() {
        let args = [&String::from("-a")];
        let output = cmd::ls(&args);

        assert!(output.status.success());
    }
    // #[test]
    // fn grep_ls() {
    //     https://stackoverflow.com/questions/73469520/how-to-pipe-commands-in-rust
    // }
}
