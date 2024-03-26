use anyhow::{anyhow, bail, Result};
use std::{
    io::{self, Write},
    process::Stdio,
};

use crate::icons::{CANCEL, FSI, LRM, PDI};

pub struct Menu {
    title: String,
    items: Vec<Item>,
}

pub struct Item {
    title: String,
    icon: char,
    pub cmd: String,
    pub requires_confirmation: bool,
}

impl Menu {
    pub fn new(title: impl Into<String>, items: Vec<Item>) -> Self {
        Menu {
            title: title.into(),
            items,
        }
    }

    pub fn new_confirmation(item: &Item) -> Self {
        Menu::new(
            "Are you sure?",
            vec![
                Item::new(
                    format!("Yes, {}", item.title),
                    item.icon,
                    item.cmd.to_owned(),
                    false,
                ),
                Item::new("No, cancel", CANCEL, String::new(), false),
            ],
        )
    }

    pub fn render(&self) -> String {
        self.items
            .iter()
            .map(|i| i.render())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn nth(&self, n: usize) -> Option<&Item> {
        self.items.get(n)
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

impl Item {
    pub fn new(
        title: impl Into<String>,
        icon: char,
        cmd: impl Into<String>,
        requires_confirmation: bool,
    ) -> Self {
        Item {
            title: title.into(),
            icon,
            cmd: cmd.into(),
            requires_confirmation,
        }
    }

    fn render(&self) -> String {
        let span_icon = format!(r#"<span font_size="large">{}</span>"#, self.icon);
        let span_text = format!(r#"<span font_size="large">{}</span>"#, self.title);

        format!("{LRM}{span_icon}  {FSI}{span_text}{PDI}")
    }
}

pub struct Wofi {
    path: String,
    args: String,
}

impl Wofi {
    pub fn new(path: impl Into<String>, args: impl Into<String>) -> Self {
        Wofi {
            path: path.into(),
            args: args.into(),
        }
    }

    pub fn spawn(&self, menu: &Menu) -> Result<String> {
        let mut child = std::process::Command::new(&self.path)
            // both prompt and lines can be overridden by self.args
            .arg("--prompt")
            .arg(&menu.title)
            .arg(format!("--lines={}", menu.size() + 1))
            // default/configured args
            .args(self.args.split_whitespace().collect::<Vec<_>>())
            // mandatory arguments
            .arg("-Ddmenu-print_line_num=true")
            .arg("--dmenu")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        child
            .stdin
            .as_mut()
            .ok_or(anyhow!("failed to get stdin"))?
            .write_all(menu.render().as_bytes())?;

        let out = child.wait_with_output()?;

        if !out.status.success() {
            io::stderr().write_all(&out.stderr)?;
            if out.status.code() == Some(1) {
                return Ok(String::from(""));
            }
            bail!("Wofi exited with status: {}", out.status);
        }

        let selection = String::from_utf8_lossy(&out.stdout);
        Ok(selection
            .strip_suffix('\n')
            .unwrap_or(&selection)
            .to_string())
    }
}
