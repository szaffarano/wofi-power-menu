use anyhow::Result;
use std::process::{Command, Stdio};

pub fn run(cmd: impl Into<String>) -> Result<()> {
    let cmd = String::from(cmd.into());
    let mut cmd = cmd.split_whitespace();

    if let Some(cmd_name) = cmd.nth(0) {
        let args = cmd.collect::<Vec<&str>>();

        println!("About to execute: '{}' with args {:?}", cmd_name, args);

        Command::new(cmd_name)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .output()?;
    }

    Ok(())
}
