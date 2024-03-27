use anyhow::Result;
use clap::Parser;

use crate::{
    icons,
    wofi::{Config, Item, Menu, Wofi},
};

/// Power menu using the wofi launcher
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Print additional information
    #[arg(short, long, default_value = "false")]
    verbose: bool,

    /// Path to the wofi binary
    #[arg(short, long)]
    wofi_path: Option<String>,

    /// Comma-separated list of menu items to disable
    #[arg(short, long)]
    disable: Vec<String>,

    /// Simulate the command without executing it
    #[arg(short = 'D', long, default_value = "false")]
    dry_run: bool,

    /// Comma-separated list of menu items to force confirmation
    #[arg(short, long)]
    confirm: Vec<String>,

    /// Show the menu items and exit
    #[arg(short, long)]
    pub list_items: bool,
}

pub fn default_menu() -> Menu {
    Menu::new(
        String::from("Power menu"),
        vec![
            Item::new(
                "shutdown",
                "Shut down",
                icons::SHUTDOWN,
                "systemctl poweroff",
                true,
            ),
            Item::new("reboot", "Reboot", icons::REBOOT, "systemctl reboot", true),
            Item::new(
                "suspend",
                "Suspend",
                icons::SUSPEND,
                "systemctl suspend",
                true,
            ),
            Item::new(
                "hibernate",
                "Hibernate",
                icons::HIBERNATE,
                "systemctl hibernate",
                false,
            ),
            Item::new(
                "logout",
                "Logout",
                icons::LOGOUT,
                "loginctl terminate-session",
                false,
            ),
            Item::new(
                "lock-screen",
                "Lock screen",
                icons::LOCK_SCREEN,
                "loginctl lock-session",
                false,
            ),
        ],
    )
}

pub fn default_wofi() -> Wofi {
    Wofi::new("wofi", "--allow-markup --columns=1 --hide-scroll")
}

pub fn merge_config(menu: &mut Menu, wofi: &mut Wofi, config: Option<Config>) -> Result<()> {
    if let Some(config) = config {
        if let Some(wofi_config) = config.wofi {
            wofi.merge(wofi_config)?;
        }

        if let Some(menu_config) = config.menu {
            menu.merge(menu_config)?;
        }
    } else {
        println!("No config file found, using default values");
    }

    Ok(())
}

pub fn merge_cli_args(menu: &mut Menu, wofi: &mut Wofi, cli: &CliArgs) -> Result<()> {
    if let Some(path) = &cli.wofi_path {
        wofi.update_path(path);
    }

    cli.disable.iter().for_each(|i| {
        if let Some(item) = menu.item_mut(i) {
            item.disable();
        }
    });

    wofi.dry_run(cli.dry_run);

    cli.confirm.iter().for_each(|i| {
        if let Some(item) = menu.item_mut(i) {
            item.set_confirmation(true);
        }
    });

    Ok(())
}
