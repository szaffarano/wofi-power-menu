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
            let result = Command::new(cmd_name).args(args).output()?;
            let stdout = String::from_utf8_lossy(&result.stdout);
            let stderr = String::from_utf8_lossy(&result.stderr);
            println!("{}", stdout.trim_end());
            eprintln!("{}", stderr.trim_end());
            std::process::exit(result.status.code().unwrap_or(1));
        }
    }

    Ok(())
}
