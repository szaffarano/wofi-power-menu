use anyhow::{anyhow, bail};
use std::io::{self, Write};

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

#[derive(Debug)]
pub struct MenuItem {
    pub title: String,
    pub icon: char,
    pub cmd: String,
    pub requires_confirmation: bool,
}

#[derive(Debug)]
pub struct Menu {
    pub title: String,
    options: Vec<MenuItem>,
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            title: String::from("Power menu"),
            options: vec![
                MenuItem {
                    title: String::from("Shut down"),
                    icon: SHUTDOWN,
                    cmd: String::from("systemctl poweroff"),
                    requires_confirmation: true,
                },
                MenuItem {
                    title: String::from("Reboot"),
                    icon: REBOOT,
                    cmd: String::from("systemctl reboot"),
                    requires_confirmation: true,
                },
                MenuItem {
                    title: String::from("Suspend"),
                    icon: SUSPEND,
                    cmd: String::from("systemctl suspend"),
                    requires_confirmation: true,
                },
                MenuItem {
                    title: String::from("Hibernate"),
                    icon: HIBERNATE,
                    cmd: String::from("systemctl hibernate"),
                    requires_confirmation: false,
                },
                MenuItem {
                    title: String::from("Log out"),
                    icon: LOGOUT,
                    cmd: String::from("loginctl terminate-session ${XDG_SESSION_ID-}"),
                    requires_confirmation: true,
                },
                MenuItem {
                    title: String::from("Lock screen"),
                    icon: LOCK_SCREEN,
                    cmd: String::from("loginctl lock-session ${XDG_SESSION_ID-}"),
                    requires_confirmation: false,
                },
            ],
        }
    }
}

impl MenuItem {
    fn render(&self) -> String {
        let span_icon = format!(r#"<span font_size="large">{}</span>"#, self.icon);
        let span_text = format!(r#"<span font_size="large">{}</span>"#, self.title);

        format!("{LRM}{span_icon}  {FSI}{span_text}{PDI}")
    }
}

impl Menu {
    pub fn new(title: impl Into<String>, options: Vec<MenuItem>) -> Self {
        Menu {
            title: title.into(),
            options,
        }
    }

    pub fn new_confirmation(item: &MenuItem) -> Self {
        Menu::new(
            "Are you sure?",
            vec![
                MenuItem {
                    icon: item.icon,
                    title: format!("Yes, {}", item.title),
                    cmd: item.cmd.clone(),
                    requires_confirmation: false,
                },
                MenuItem {
                    icon: CANCEL,
                    title: String::from("No, cancel"),
                    cmd: String::from(""),
                    requires_confirmation: false,
                },
            ],
        )
    }

    pub fn render(&self) -> String {
        self.options
            .iter()
            .map(|o| o.render())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn nth(&self, n: usize) -> Option<&MenuItem> {
        self.options.get(n)
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
    pub fn spawn(&self, menu: &Menu) -> anyhow::Result<String> {
        let mut args = vec![String::from("-p"), menu.title.clone()];

        args.extend(self.args.clone());

        let mut child = std::process::Command::new(&self.path)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let mut stdin = child.stdin.take().ok_or(anyhow!("failed to get stdin"))?;

        let menu_content = menu.render();
        std::thread::spawn(move || {
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

        let selection = String::from_utf8(out.stdout)?;
        Ok(selection
            .strip_suffix("\n")
            .unwrap_or(&selection)
            .to_string())
    }
}

pub fn run(cmd: impl Into<String>) -> anyhow::Result<()> {
    let cmd = String::from(cmd.into());
    let mut cmd = cmd.split_whitespace();

    if let Some(cmd_name) = cmd.nth(0) {
        let args = cmd.collect::<Vec<&str>>();

        std::process::Command::new(cmd_name)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .output()?;
    }

    Ok(())
}
