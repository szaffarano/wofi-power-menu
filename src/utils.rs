use anyhow::Result;
use std::process::Command;

pub fn run(cmd: String, dry_run: bool) -> Result<()> {
    let mut cmd = cmd.split_whitespace();

    if let Some(cmd_name) = cmd.next() {
        let args = cmd.collect::<Vec<&str>>();

        let dry = if dry_run { " (dry run)" } else { "" };
        println!(
            "About to execute: '{}' with args {:?} {}",
            cmd_name, args, dry
        );

        if !dry_run {
            Command::new(cmd_name).args(args).output()?;
        }
    }

    Ok(())
}
