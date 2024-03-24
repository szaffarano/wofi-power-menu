use anyhow::{anyhow, bail, Result};
use std::{
    io::{self, Write},
    process::Stdio,
    thread,
};

const LOCK_SCREEN: char = '\u{f033e}';
const LOGOUT: char = '\u{f0343}';
const SUSPEND: char = '\u{f04b2}';
const HIBERNATE: char = '\u{f02ca}';
const REBOOT: char = '\u{f0709}';
const SHUTDOWN: char = '\u{f0425}';
const CANCEL: char = '\u{f0156}';
const LRM: char = '\u{200e}';
const FSI: char = '\u{2068}';
const PDI: char = '\u{2069}';

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

impl Default for Menu {
    fn default() -> Self {
        Menu {
            title: String::from("Power menu"),
            items: vec![
                Item::new("Shut down", SHUTDOWN, "systemctl poweroff", true),
                Item::new("Reboot", REBOOT, "systemctl reboot", true),
                Item::new("Suspend", SUSPEND, "systemctl suspend", true),
                Item::new("Hibernate", HIBERNATE, "systemctl hibernate", false),
                Item::new("Logout", LOGOUT, "loginctl terminate-session", false),
                Item::new("Lock screen", LOCK_SCREEN, "loginctl lock-session", false),
            ],
        }
    }
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
}

impl Item {
    fn new(
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
    args: Vec<String>,
}

impl Default for Wofi {
    fn default() -> Self {
        Wofi {
            path: String::from("/home/sebas/.nix-profile/bin/wofi"),
            args: vec![
                String::from("-S"),
                String::from("dmenu"),
                String::from("-m"),
                String::from("-i"),
                String::from("-Ddmenu-print_line_num=true"),
                String::from("-w"),
                String::from("1"),
                String::from("-L"),
                String::from("8"),
                String::from("-b"),
            ],
        }
    }
}

impl Wofi {
    pub fn spawn(&self, menu: &Menu) -> Result<String> {
        let mut args = vec![String::from("-p"), menu.title.clone()];

        args.extend(self.args.clone());

        let mut child = std::process::Command::new(&self.path)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = child.stdin.take().ok_or(anyhow!("failed to get stdin"))?;

        let menu_content = menu.render();
        thread::spawn(move || {
            stdin
                .write_all(menu_content.as_bytes())
                .expect("Failed to write to stdin");
        });

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
            .strip_suffix("\n")
            .unwrap_or(&selection)
            .to_string())
    }
}
