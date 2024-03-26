use anyhow::{anyhow, Result};

use wofi_power_menu::{
    cmd, power_menu,
    wofi::{self, Menu, Wofi},
};

fn main() -> Result<()> {
    let mut menu = power_menu::default_menu();
    let mut wofi = power_menu::default_wofi();

    if let Some(config) = wofi::get_config(env!("CARGO_BIN_NAME"))? {
        if let Some(wofi_config) = config.wofi {
            wofi = Wofi::new(
                wofi_config.path.unwrap_or(wofi.path()),
                wofi_config.extra_args.unwrap_or(wofi.args()),
            );
        }

        if let Some(menu_config) = config.menu {
            menu.merge(menu_config)?;
        }
    } else {
        println!("No config file found, using default values");
    }

    let selection = wofi.spawn(&menu)?;

    if selection.is_empty() {
        return Ok(());
    }

    let item_selected = menu
        .nth(selection.parse::<usize>()?)
        .ok_or(anyhow!(format!("Invalid selection: {}", selection)))?;

    let cmd = if item_selected.requires_confirmation() {
        let confirmation = Menu::new_confirmation(item_selected);
        let response = wofi.spawn(&confirmation)?;
        if response.is_empty() {
            return Ok(());
        }

        let option = confirmation
            .nth(response.parse::<usize>()?)
            .ok_or(anyhow!(format!("Invalid response: {}", selection)))?;
        option.cmd()
    } else {
        item_selected.cmd()
    };

    cmd::run(cmd)?;

    Ok(())
}
