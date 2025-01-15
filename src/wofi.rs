use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Stdio,
    str::FromStr,
};

use crate::icons::{CANCEL, FSI, LRI, LRM, PDI};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub wofi: Option<WofiConfig>,
    pub menu: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WofiConfig {
    pub path: Option<String>,
    pub extra_args: Option<String>,
}

impl Config {
    pub fn read<T: AsRef<Path>>(path: T) -> Result<Option<Self>> {
        let path = path.as_ref();

        if fs::metadata(path).is_err() {
            return Ok(None);
        }

        let raw = fs::read_to_string(path)?;

        let config = toml::from_str(&raw)
            .with_context(|| format!("{}: Invalid format", path.to_string_lossy()))?;

        Ok(Some(config))
    }
}

pub struct Menu {
    title: String,
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct Item {
    id: String,
    title: String,
    icon: char,
    cmd: String,
    requires_confirmation: bool,
    enabled: bool,
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
                    "yes",
                    format!("Yes, {}", item.title),
                    item.icon,
                    item.cmd.to_owned(),
                    false,
                ),
                Item::new("no", "No, cancel", CANCEL, String::new(), false),
            ],
        )
    }

    pub fn render(&self) -> String {
        self.items
            .iter()
            .filter(|i| i.enabled)
            .map(|i| i.render())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn nth(&self, n: usize) -> Option<&Item> {
        self.items
            .iter()
            .filter(|i| i.enabled)
            .collect::<Vec<&Item>>()
            .get(n)
            .copied()
    }

    pub fn item(&self, id: impl Into<String>) -> Option<&Item> {
        let id = id.into();

        self.items.iter().find(|i| i.id == id)
    }

    pub fn item_mut(&mut self, id: impl Into<String>) -> Option<&mut Item> {
        let id = id.into();

        self.items.iter_mut().find(|i| i.id == id)
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn size(&self) -> usize {
        self.items.iter().filter(|i| i.enabled).count()
    }

    pub fn merge(&mut self, config: HashMap<String, HashMap<String, String>>) -> Result<()> {
        for (key, value) in config {
            if let Some(item) = self.item_mut(key.as_str()) {
                item.merge(value)?;
            } else {
                self.add_item(Item::from_config(key, value)?);
            }
        }
        Ok(())
    }

    pub fn iter(&self) -> MenuIterator {
        MenuIterator {
            curr: 0,
            menu: self,
        }
    }
}

pub struct MenuIterator<'a> {
    curr: usize,
    menu: &'a Menu,
}

impl Iterator for MenuIterator<'_> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.menu.nth(self.curr).cloned();
        self.curr += 1;
        next
    }
}

impl Item {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        icon: char,
        cmd: impl Into<String>,
        requires_confirmation: bool,
    ) -> Self {
        Item {
            id: id.into(),
            title: title.into(),
            icon,
            cmd: cmd.into(),
            requires_confirmation,
            enabled: true,
        }
    }

    pub fn from_config(id: impl Into<String>, config: HashMap<String, String>) -> Result<Self> {
        let id = id.into();
        let title = config
            .get("title")
            .ok_or_else(|| anyhow!(format!("{}: title not found in config", id)))?
            .to_string();
        let cmd = config
            .get("cmd")
            .ok_or_else(|| anyhow!(format!("{}: cmd not found in config", id)))?
            .to_string();
        let icon = config
            .get("icon")
            .ok_or_else(|| anyhow!("failed to get stdin"))?
            .chars()
            .next()
            .ok_or_else(|| anyhow!(format!("{}: unexpected empty string", id)))?;
        let requires_confirmation = config
            .get("requires_confirmation")
            .unwrap_or(&String::from("false"))
            .parse()?;
        let enabled = config
            .get("enabled")
            .unwrap_or(&String::from("true"))
            .parse()?;

        let attributes = ["title", "icon", "cmd", "requires_confirmation", "enabled"];
        config
            .iter()
            .filter(|(k, _)| !attributes.contains(&k.as_str()))
            .for_each(|(k, _)| println!("[WARNING] {}: invalid property declared in '{}'", k, id));

        Ok(Item {
            id,
            title,
            icon,
            cmd,
            requires_confirmation,
            enabled,
        })
    }

    fn render(&self) -> String {
        let span_icon = format!(r#"<span font_size="large">{}</span>"#, self.icon);
        let span_text = format!(r#"<span font_size="large">{}</span>"#, self.title);

        format!("{LRM}{span_icon}  {FSI}{span_text}{PDI}")
    }

    pub fn requires_confirmation(&self) -> bool {
        self.requires_confirmation
    }

    pub fn set_confirmation(&mut self, confirm: bool) {
        self.requires_confirmation = confirm;
    }

    pub fn cmd(&self) -> String {
        self.cmd.to_string()
    }

    pub fn merge(&mut self, other: HashMap<String, String>) -> Result<()> {
        for (key, value) in other {
            match key.as_str() {
                "title" => self.title = value,
                "icon" => self.icon = value.chars().next().unwrap_or(LRI),
                "cmd" => self.cmd = value,
                "requires_confirmation" => self.requires_confirmation = value.parse()?,
                "enabled" => self.enabled = value.parse()?,
                _ => bail!(format!("{}: invalid property", key)),
            }
        }
        Ok(())
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} [disabled: {}, confirmation: {}]",
            self.id, self.title, !self.enabled, self.requires_confirmation
        )
    }
}

pub struct Wofi {
    path: String,
    args: String,
    dry_run: bool,
}

impl Wofi {
    pub fn new(path: impl Into<String>, args: impl Into<String>) -> Self {
        Wofi {
            path: path.into(),
            args: args.into(),
            dry_run: false,
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

    pub fn path(&self) -> String {
        self.path.to_string()
    }

    pub fn args(&self) -> String {
        self.args.to_string()
    }

    pub fn merge(&mut self, config: WofiConfig) -> Result<()> {
        if let Some(path) = config.path {
            self.path = path;
        }

        if let Some(args) = config.extra_args {
            self.args = args;
        }

        Ok(())
    }

    pub fn update_path(&mut self, path: impl Into<String>) {
        self.path = path.into();
    }

    pub fn dry_run(&mut self, dry_run: bool) {
        self.dry_run = dry_run;
    }
}

pub fn get_config(
    file_name: impl Into<String>,
    config_path: &Option<String>,
) -> anyhow::Result<Option<Config>> {
    let path = match config_path {
        // If --config <CONFIG> was passed, use the specified toml config file
        Some(path) => PathBuf::from_str(path)?,
        // Else default to $HOME/.config/wofi-power-menu.toml
        None => directories_next::BaseDirs::new()
            .ok_or_else(|| anyhow!("Error reading config"))?
            .config_dir()
            .join(format!("{}.toml", file_name.into())),
    };

    Config::read(path)
}
