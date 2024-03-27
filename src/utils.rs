use anyhow::Result;
use std::process::Command;

pub fn run(cmd: String) -> Result<()> {
    let mut cmd = cmd.split_whitespace();

    if let Some(cmd_name) = cmd.next() {
        let args = cmd.collect::<Vec<&str>>();

        println!("About to execute: '{}' with args {:?}", cmd_name, args);

        Command::new(cmd_name).args(args).output()?;
    }

    Ok(())
}
